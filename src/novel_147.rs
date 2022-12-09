use std::sync::Arc;

use crate::novel::*;
use anyhow::{bail, Ok};
use log::{debug, info};
use scraper::{ElementRef, Html, Selector};

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
                let mut novel_chapter: NovelChapter =
                    NovelChapter::new(chapter_index, Some(chapter.text().collect()), None);
                if let Some(href) = chapter.value().attr("href") {
                    let href = String::from(Self::host_url() + href);
                    novel_chapter.url = Some(href.clone());

                    let content =
                        tokio::spawn(async move { reqwest::get(href).await?.text().await })
                            .await??;

                    let html = Html::parse_document(&content);
                    let selector = Selector::parse("#content").unwrap();
                    let content = html.select(&selector).next().unwrap();
                    let content = content.text().collect();
                    novel_chapter.content = Some(content);
                    info!("完成第{}章的文本内容读取", &novel_chapter.index);
                    chapters.push(novel_chapter);
                };
            }
            Ok(chapters)
        } else {
            bail!("该小说{:#?}没有目录URL", novel)
        }
    }
    fn generate_book(book: ElementRef) -> Option<NovelInfo> {
        let mut index: i32 = -1;
        let mut novel_info = NovelInfo::new(None, None, None, None, None);
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
