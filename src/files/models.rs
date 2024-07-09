use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub id: String,
    pub dir_id: String,
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub img_dimension: Option<String>,
    pub img_versions: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileDto {
    pub name: String,
    pub urls: FileUrls,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileUrls {
    pub o: String,
    pub s: String,
}

impl FileUrls {
    pub fn new() -> Self {
        Self {
            o: "".to_string(),
            s: "".to_string(),
        }
    }
}
