use crate::novel_147::Novel147;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct NovelInfo {
    pub name: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub desc: Option<String>,
    pub index_url: Option<String>,
    pub chapters: Option<Vec<NovelChapter>>,
}

#[derive(Debug, Clone)]
pub struct NovelChapter {
    pub index: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
}
#[async_trait]
pub trait NovelSourceTrait {
    fn name() -> &'static str;
    fn host_url() -> &'static str;
    fn book_url_pattern() -> &'static str;
    async fn search_name(name: &String) -> anyhow::Result<Vec<NovelInfo>>;
    async fn get_chapters(mut novel: NovelInfo) -> anyhow::Result<NovelInfo>;
    async fn get_chapters_content(chapters: Vec<NovelChapter>,) -> anyhow::Result<Vec<NovelChapter>>;
}

pub enum NovelSource {
    Novel147,
    UnDo
}

impl NovelSource{
    pub fn name(&self) -> &'static str {
        match self {
            NovelSource::Novel147 => {
                Novel147::name()
            },
            _ => {
                "未实现"
            }
        }
    }
    pub fn host_url(&self) -> &'static str {
        match self {
            NovelSource::Novel147 => {
                Novel147::host_url()
            },
            _ => {
                "未实现"
            }
        }
    }
    pub fn book_url_pattern(&self) -> &'static str {
        match self {
            NovelSource::Novel147 => {
                Novel147::book_url_pattern()
            },
            _ => {
                "未实现"
            }
        }
    }
    pub async fn search_name(&self, name: &String) -> anyhow::Result<Vec<NovelInfo>> {
        match self {
            NovelSource::Novel147 => {
                Novel147::search_name(name).await
            },
            _ => todo!()
        }
    }
    pub async fn get_chapters(&self, novel: NovelInfo) -> anyhow::Result<NovelInfo> {
        match self {
            NovelSource::Novel147 => {
                Novel147::get_chapters(novel).await
            },
            _ => todo!()
        }
    }
    pub async fn get_chapters_content(&self, chapters: Vec<NovelChapter>,) -> anyhow::Result<Vec<NovelChapter>> {
        match self {
            NovelSource::Novel147 => {
                Novel147::get_chapters_content(chapters).await
            },
            _ => todo!()
        }
    }

}
