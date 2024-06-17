use deadpool_diesel::sqlite::Pool;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use tracing::error;

use crate::files::models::{Bucket, NewBucket};
use crate::schema::buckets::{self, dsl};
use crate::uuid::generate_id;
use crate::Result;

pub async fn list_buckets(db_pool: Pool, client_id: &str) -> Result<Vec<Bucket>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::buckets
                .filter(dsl::client_id.eq(cid))
                .select(Bucket::as_select())
                .load::<Bucket>(conn)
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(items) => Ok(items),
            Err(e) => {
                error!("{e}");
                Err("Error reading buckets".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}

pub async fn create_bucket(db_pool: Pool, client_id: &str, data: NewBucket) -> Result<Bucket> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let bucket = Bucket {
        id: generate_id(),
        client_id: client_id.to_string(),
        name: data.name,
        label: data.label,
    };

    let bucket_copy = bucket.clone();
    let conn_result = db
        .interact(move |conn| {
            diesel::insert_into(buckets::table)
                .values(&bucket_copy)
                .execute(conn)
        })
        .await;

    match conn_result {
        Ok(insert_res) => match insert_res {
            Ok(_) => Ok(bucket),
            Err(e) => {
                error!("{e}");
                Err("Error creating a bucket".into())
            }
        },
        Err(e) => {
            error!("{e}");
            Err("Error using the db connection".into())
        }
    }
}
