use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::buckets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bucket {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub label: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::buckets)]
pub struct NewBucket {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub label: String,
}

#[derive(Queryable, Selectable)]
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
pub struct File {
    pub name: String,
    pub url: String,
}
