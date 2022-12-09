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
