#[derive(Debug, Clone)]
pub struct Directory {
    pub id: String,
    pub dir_type: String,
    pub bucket_id: String,
    pub name: String,
    pub label: String,
    pub file_count: u32,
    pub created_at: i64,
    pub updated_at: i64,
}
