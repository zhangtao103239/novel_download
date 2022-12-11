use std::time::Duration;

use crate::novel::*;
use anyhow::{bail, Ok};
use log::{debug, error, info};
use scraper::{ElementRef, Html, Selector};
// use tokio_stream::{self as stream, StreamExt};

pub struct Novel147 {}
impl Novel147 {
    pub fn name() -> &'static str {
        "147小说"
    }

    pub fn host_url() -> String {
        "https://www.147xs.org".to_string()
    }

    pub fn search_url() -> String {
        Self::host_url() + "/search.php?keyword={{key}}"
    }

    pub async fn search_name(name: &String) -> anyhow::Result<Vec<NovelInfo>> {
        let name = urlencoding::encode(name.as_str()).to_string();
        let url = Self::search_url().replace("{{key}}", &name);
        info!("即将访问：{}", &url);
        let res = reqwest::get(url).await?.text().await?;
        let html = Html::parse_document(&res);
        let selector = Selector::parse("#bookcase_list > tr").unwrap();
        let mut books: Vec<NovelInfo> = Vec::new();
        let mut books_select = html.select(&selector);
        while let Some(book) = books_select.next() {
            if let Some(book) = Self::generate_book(book) {
                books.push(book);
            }
        }
        Ok(books)
    }
    pub async fn get_content(novel: &NovelInfo) -> anyhow::Result<Vec<NovelChapter>> {
        if let Some(url) = &novel.index_url {
            let res = reqwest::get(url).await?.text().await?;
            let html = Html::parse_document(&res);
            let selector = Selector::parse("dl > dd > a").unwrap();
            let mut chapters: Vec<NovelChapter> = Vec::new();
            let mut chapter_selector = html.select(&selector);
            let mut chapter_index = 0;

            while let Some(chapter) = chapter_selector.next() {
                chapter_index += 1;
                let chapter_name: String = chapter.text().collect();
                if let Some(href) = chapter.value().attr("href") {
                    let href = String::from(Self::host_url() + href);
                    chapters.push(NovelChapter { index: chapter_index, name: Some(chapter_name), url: Some(href), content: None });
                };
            }
            Self::get_chapters_content(chapters).await
        } else {
            bail!("该小说{:#?}没有目录URL", novel)
        }
    }
    pub async fn get_chapters_content(chapters: Vec<NovelChapter>) -> anyhow::Result<Vec<NovelChapter>> {
        let mut tasks = Vec::with_capacity(chapters.len());
        for mut chapter in chapters {
            if let Some(_) = chapter.content.clone() {
                info!("第{}章已经完成了下载，跳过", chapter.index);
                tasks.push(tokio::spawn(async move {
                    Ok(chapter)
                }));
            } else {
                if let Some(href) = chapter.url.clone() {
                    tasks.push(tokio::spawn(async move {
                        std::thread::sleep(Duration::from_millis(1200));
                        info!("开始下载第{}章的内容", chapter.index);
                        let content = reqwest::get(&href).await?.text().await?;
                        let html = Html::parse_document(&content);
                        let selector = Selector::parse("#content").unwrap();
                        return if let Some(content) = html.select(&selector).next() {
                            let content: String = content.text().collect();
                            info!("已获取到第{}章的内容", &chapter.index);
                            lazy_static::lazy_static! {
                                static ref RE: regex::Regex = regex::Regex::new(r"^.*野果.*$").unwrap();
                            }
                            let content = RE.replace_all(&content, "").to_string();
                            debug!("替换野果阅读后的内容为：{}", content);
                            chapter.content = Some(content);
                            Ok(chapter)
                        } else {
                            error!(
                                "获取不到第{}章{}的内容，下载内容为{}",
                                &chapter.index, &href, &content
                            );
                            chapter.content = None;
                            Ok(chapter)
                        };
                    }));
                } else {
                    error!("当前章节无URL: {:#?}", chapter);
                }
            }
        }

        let mut chapters = Vec::new();
        for task in tasks {
            let chapter = task.await??;
            chapters.push(chapter);
        }
        Ok(chapters)
    }
    fn generate_book(book: ElementRef) -> Option<NovelInfo> {
        let mut index: i32 = -1;
        let mut novel_info = NovelInfo{name: None, url: None, author: None, desc: None, index_url: None};
        debug!("当前节点为{:#?}", book.html());
        let selector = Selector::parse("td").unwrap();
        let mut book_select = book.select(&selector);
        while let Some(td) = book_select.next() {
            debug!("当前td节点为{:#?}", td.html());
            index += 1;
            if index == 1 {
                let name: String = td.text().collect();
                novel_info.name = Some(name);
                let url: String = td
                    .first_child()?
                    .value()
                    .as_element()?
                    .attr("href")?
                    .to_string();
                novel_info.url = Some(url.clone());
                novel_info.index_url = Some(url);
            } else if index == 3 {
                let author: String = td.text().collect();
                novel_info.author = Some(author);
                break;
            }
        }
        info!("获取到书籍内容：{:#?}", novel_info);
        Some(novel_info)
    }
}
