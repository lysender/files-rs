use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub name: String,
    pub url: String,
}
