use anyhow::{bail, Result};
use log::*;
use novel_147::Novel147;

mod novel;
mod novel_147;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let name = if args.len() < 2 {
        error!("请输入要下载的小说名称！");
        "我的".to_string()
    } else {
        args[1].to_string()
    };
    print!("{}", &name);
    info!("开始使用{}进行搜索：{}", Novel147::name() ,&name);
    let novels = Novel147::search_name(&name).await?;
    if novels.is_empty() {
        bail!("未搜索到{}，请尝试其他名字吧", name)
    }
    debug!("搜索出以下结果：\n{:#?}", novels);
    let novel = &novels[0];
    if let Some(name) = &novel.name {
        info!("即将开始获取小说{}的信息", name);
        let mut chapters = Novel147::get_content(novel).await?;
        if chapters.is_empty() {
            bail!("未获取到小说的章节")
        }

        let mut failed_count = chapters.iter().filter(|c| c.content.is_none()).count();
        let retry_count = 5;
        let mut retry_index = 0;
        while failed_count != 0 && retry_index < retry_count{
            chapters = Novel147::get_chapters_content(chapters).await?;
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
            "已获取到小说内容，共{}章，准备写入文件{}.txt",chapters.len(), name
        );

        fs::write(format!("{}.txt", name), content_list.join("\n\n"))?;
    } else {
        bail!("搜索出的第一个小说无名称！: {:#?}", novel)
    }
    Ok(())
}
