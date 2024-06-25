use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct File {
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
