use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::directories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dir {
    pub id: String,
    pub dir_type: String,
    pub bucket_id: String,
    pub name: String,
    pub label: String,
    pub file_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewDir {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::schema::directories)]
pub struct UpdateDir {
    #[validate(length(min = 1, max = 100))]
    pub label: Option<String>,
}
