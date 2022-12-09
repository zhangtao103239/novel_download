#[derive(Debug, Clone)]
pub struct NovelInfo {
    pub name: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub desc: Option<String>,
    pub index_url: Option<String>,
}

impl NovelInfo {
    pub fn new(name: Option<String>, url: Option<String>, author: Option<String>, desc: Option<String>, index_url: Option<String>) -> Self {
        Self {
            name,
            url,
            author,
            desc,
            index_url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NovelChapter {
    pub index: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
}

impl NovelChapter {
    pub fn new(index:i32, name: Option<String>, url: Option<String>) -> Self {
        Self {
            index, name, url, content: None
        }
    }
}
