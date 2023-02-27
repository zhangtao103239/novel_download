use anyhow::{bail, Result};
use log::*;
mod novel;
mod novel_147;
use std::{env, fs};

use crate::novel::NovelSource;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let name = if args.len() < 2 {
        error!("请输入要下载的小说名称！");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    } else {
        args[1].to_string()
    };
    let search_engine = if args.len() < 3 || args[2].is_empty() {
        error!("请输入要使用的小说书源名称(147xs)！");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    } else {
        args[2].to_string()
    };
    let search_engine = match search_engine.trim() {
        "147xs" => NovelSource::Novel147,
        &_ => NovelSource::UnDo,
    };
    let novel = 
    if name.starts_with("http") {
        info!("获取到书籍链接{}，直接使用该链接下载！", name);
        let book_url_pattern = regex::Regex::new(search_engine.book_url_pattern()).unwrap();
        if !book_url_pattern.is_match(name.as_str()) {
            bail!("书籍链接{}不符合书源规则{}，无法解析", name, search_engine.book_url_pattern())
        }
        novel::NovelInfo {
            name: Some("未知书籍".to_string()),
            url: Some(name.clone()),
            author: None,
            desc: None,
            index_url: Some(name.clone()),
            chapters: None,
        }
    } else {
        info!("开始使用{}({})进行搜索：{}", search_engine.name(), search_engine.host_url(), &name);
        let novels = search_engine.search_name(&name).await?;
        
        if novels.is_empty() {
            bail!("未搜索到{}，请尝试其他名字吧", name)
        }
        debug!("搜索出以下结果：\n{:#?}", novels);
        novels[0].to_owned()
    };
    if let Some(name) = novel.name.clone() {
        info!("即将开始获取小说{}的信息", name);
        let novel = search_engine.get_chapters(novel).await?;
        if novel.chapters.is_none() {
            bail!("未获取到小说的章节");
        }
        let mut chapters = novel.chapters.unwrap();

        let mut failed_count = chapters.iter().filter(|c| c.content.is_none()).count();
        let retry_count = 5;
        let mut retry_index = 0;
        while failed_count != 0 && retry_index < retry_count {
            chapters = search_engine.get_chapters_content(chapters).await?;
            failed_count = chapters.iter().filter(|c| c.content.is_none()).count();
            retry_index += 1;
        }
        if failed_count > 0 {
            error!("仍然有未获取到的小说章节，共有{}章失败！", failed_count);
        }
        let mut content_list = Vec::new();
        for chapter in &chapters {
            if let Some(content) = &chapter.name {
                content_list.push(format!("第{}章  ", chapter.index) + content.as_str());
            } else {
                error!("第{}章未获取到标题:{:#?}", chapter.index, chapter);
            }
            if let Some(content) = &chapter.content {
                content_list.push(content.to_string());
            } else {
                error!("第{}章未获取到正文:{:#?}", chapter.index, chapter);
            }
        }
        info!(
            "已获取到小说内容，共{}章，准备写入文件{}.txt",
            chapters.len(),
            name
        );

        fs::write(format!("{}.txt", name), content_list.join("\n\n"))?;
    } else {
        bail!("搜索出的第一个小说无名称！: {:#?}", novel)
    }
    Ok(())
}
