use crate::novel_147::Novel147;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct NovelInfo {
    pub name: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub desc: Option<String>,
    pub index_url: Option<String>,
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
    async fn search_name(name: &String) -> anyhow::Result<Vec<NovelInfo>>;
    async fn get_chapters(novel: &NovelInfo) -> anyhow::Result<Vec<NovelChapter>>;
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
    pub async fn search_name(&self, name: &String) -> anyhow::Result<Vec<NovelInfo>> {
        match self {
            NovelSource::Novel147 => {
                Novel147::search_name(name).await
            },
            _ => todo!()
        }
    }
    pub async fn get_chapters(&self, novel: &NovelInfo) -> anyhow::Result<Vec<NovelChapter>> {
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
