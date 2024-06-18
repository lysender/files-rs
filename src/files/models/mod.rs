use diesel::prelude::*;

#[derive(Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::buckets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bucket {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NewBucket {
    pub name: String,
    pub label: String,
}

#[derive(Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::directories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Directory {
    pub id: String,
    pub dir_type: String,
    pub bucket_id: String,
    pub name: String,
    pub label: String,
    pub file_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct NewDirectory {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub url: String,
}
