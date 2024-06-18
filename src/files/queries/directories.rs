use deadpool_diesel::sqlite::Pool;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use tracing::error;

use crate::files::models::{Directory, NewDirectory};
use crate::schema::directories::{self, dsl};
use crate::uuid::generate_id;
use crate::Result;

pub async fn list_directories(db_pool: Pool, bucket_id: &str) -> Result<Vec<Directory>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let bid = bucket_id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::directories
                .filter(dsl::bucket_id.eq(bid))
                .select(Directory::as_select())
                .load::<Directory>(conn)
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(items) => Ok(items),
            Err(e) => {
                error!("{e}");
                Err("Error reading directories".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

pub async fn create_directory(
    db_pool: Pool,
    bucket_id: &str,
    data: NewDirectory,
) -> Result<Directory> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let today = chrono::Utc::now().timestamp();
    let dir = Directory {
        id: generate_id(),
        dir_type: "files".to_string(),
        bucket_id: bucket_id.to_string(),
        name: data.name,
        label: data.label,
        file_count: 0,
        created_at: today,
        updated_at: today,
    };

    let dir_copy = dir.clone();
    let conn_result = db
        .interact(move |conn| {
            diesel::insert_into(directories::table)
                .values(&dir_copy)
                .execute(conn)
        })
        .await;

    match conn_result {
        Ok(insert_res) => match insert_res {
            Ok(_) => Ok(dir),
            Err(e) => {
                error!("{e}");
                Err("Error creating a directory".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

pub async fn get_directory(db_pool: Pool, id: &str) -> Result<Option<Directory>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let did = id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::directories
                .find(did)
                .select(Directory::as_select())
                .first::<Directory>(conn)
                .optional()
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(item) => Ok(item),
            Err(e) => {
                error!("{e}");
                Err("Error reading directories".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}
