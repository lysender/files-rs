use deadpool_diesel::sqlite::Pool;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use tracing::error;

use crate::files::models::{Bucket, NewBucket};
use crate::schema::buckets::dsl;
use crate::schema::buckets::dsl::buckets;
use crate::Result;

pub async fn list_buckets(db_pool: Pool, client_id: &str) -> Result<Vec<Bucket>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();
    let conn_result = db
        .interact(move |conn| {
            buckets
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
    todo!("foo")
}
