use deadpool_diesel::sqlite::Pool;

use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use tracing::error;
use validator::Validate;

use crate::buckets::{Bucket, NewBucket, UpdateBucket};
use crate::dirs::count_bucket_dirs;
use crate::schema::buckets::{self, dsl};
use crate::storage::read_bucket;
use crate::util::generate_id;
use crate::validators::flatten_errors;
use crate::web::pagination::Paginated;
use crate::{Error, Result};

use super::ListBucketsParams;

const MAX_BUCKETS: i32 = 10;
const MAX_PER_PAGE: i32 = 50;

pub async fn list_buckets(
    db_pool: &Pool,
    client_id: &str,
    params: &ListBucketsParams,
) -> Result<Paginated<Bucket>> {
    if let Err(errors) = params.validate() {
        return Err(Error::ValidationError(flatten_errors(&errors)));
    }
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();

    let total_records = list_buckets_count(db_pool, client_id, params).await?;
    let mut page: i32 = 1;
    let mut per_page: i32 = MAX_PER_PAGE;
    let mut offset: i64 = 0;

    if let Some(per_page_param) = params.per_page {
        if per_page_param > 0 && per_page_param <= MAX_PER_PAGE {
            per_page = per_page_param;
        }
    }

    let total_pages: i64 = (total_records as f64 / per_page as f64).ceil() as i64;

    if let Some(p) = params.page {
        let p64 = p as i64;
        if p64 > 0 && p64 <= total_pages {
            page = p;
            offset = (p64 - 1) * per_page as i64;
        }
    }

    let params_copy = params.clone();
    let conn_result = db
        .interact(move |conn| {
            let mut query = dsl::buckets.into_boxed();
            query = query.filter(dsl::client_id.eq(cid));

            if let Some(keyword) = params_copy.keyword {
                if keyword.len() > 0 {
                    let pattern = format!("%{}%", keyword);
                    query =
                        query.filter(dsl::name.like(pattern.clone()).or(dsl::label.like(pattern)));
                }
            }

            query
                .limit(per_page as i64)
                .offset(offset)
                .select(Bucket::as_select())
                .order(dsl::label.asc())
                .load::<Bucket>(conn)
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(items) => Ok(Paginated::new(items, page, per_page, total_records)),
            Err(e) => {
                error!("{}", e);
                Err("Error reading buckets".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

async fn list_buckets_count(
    db_pool: &Pool,
    client_id: &str,
    params: &ListBucketsParams,
) -> Result<i64> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();
    let params_copy = params.clone();

    let conn_result = db
        .interact(move |conn| {
            let mut query = dsl::buckets.into_boxed();
            query = query.filter(dsl::client_id.eq(cid.as_str()));
            if let Some(keyword) = params_copy.keyword {
                if keyword.len() > 0 {
                    let pattern = format!("%{}%", keyword);
                    query =
                        query.filter(dsl::name.like(pattern.clone()).or(dsl::label.like(pattern)));
                }
            }
            query.select(count_star()).get_result::<i64>(conn)
        })
        .await;

    match conn_result {
        Ok(count_res) => match count_res {
            Ok(count) => Ok(count),
            Err(e) => {
                error!("{}", e);
                Err("Error counting buckets".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn create_bucket(db_pool: &Pool, client_id: &str, data: &NewBucket) -> Result<Bucket> {
    if let Err(errors) = data.validate() {
        return Err(Error::ValidationError(flatten_errors(&errors)));
    }

    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    // Limit the number of buckets per client
    let _ = match count_client_buckets(db_pool, client_id).await {
        Ok(count) => {
            if count >= MAX_BUCKETS as i64 {
                return Err(Error::ValidationError(
                    "Maximum number of buckets reached".to_string(),
                ));
            }
        }
        Err(e) => return Err(e),
    };

    // Bucket name must be unique for the client
    if let Some(_) = find_client_bucket(db_pool, client_id, &data.name).await? {
        return Err(Error::ValidationError(
            "Bucket name already exists".to_string(),
        ));
    }

    // Validate against the cloud storage
    let _ = read_bucket(&data.name).await?;

    let data_copy = data.clone();
    let bucket = Bucket {
        id: generate_id(),
        client_id: client_id.to_string(),
        name: data_copy.name,
        label: data_copy.label,
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
                error!("{}", e);
                Err("Error creating a bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn get_bucket(db_pool: &Pool, id: &str) -> Result<Option<Bucket>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let bid = id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::buckets
                .find(bid)
                .select(Bucket::as_select())
                .first::<Bucket>(conn)
                .optional()
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(item) => Ok(item),
            Err(e) => {
                error!("{}", e);
                Err("Error finding bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn find_client_bucket(
    db_pool: &Pool,
    client_id: &str,
    name: &str,
) -> Result<Option<Bucket>> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();
    let name_copy = name.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::buckets
                .filter(dsl::client_id.eq(cid.as_str()))
                .filter(dsl::name.eq(name_copy.as_str()))
                .select(Bucket::as_select())
                .first::<Bucket>(conn)
                .optional()
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(item) => Ok(item),
            Err(e) => {
                error!("{}", e);
                Err("Error finding bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn count_client_buckets(db_pool: &Pool, client_id: &str) -> Result<i64> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let cid = client_id.to_string();
    let conn_result = db
        .interact(move |conn| {
            dsl::buckets
                .filter(dsl::client_id.eq(cid.as_str()))
                .select(count_star())
                .get_result::<i64>(conn)
        })
        .await;

    match conn_result {
        Ok(count_res) => match count_res {
            Ok(count) => Ok(count),
            Err(e) => {
                error!("{}", e);
                Err("Error counting buckets".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn update_bucket(db_pool: &Pool, id: &str, data: &UpdateBucket) -> Result<bool> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    if let Err(errors) = data.validate() {
        return Err(Error::ValidationError(flatten_errors(&errors)));
    }

    // Do not update if there is no data to update
    if data.label.is_none() {
        return Ok(false);
    }

    let data_copy = data.clone();
    let bucket_id = id.to_string();
    let conn_result = db
        .interact(move |conn| {
            diesel::update(dsl::buckets)
                .filter(dsl::id.eq(bucket_id.as_str()))
                .set(data_copy)
                .execute(conn)
        })
        .await;

    match conn_result {
        Ok(update_res) => match update_res {
            Ok(item) => Ok(item > 0),
            Err(e) => {
                error!("{}", e);
                Err("Error updating bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn delete_bucket(db_pool: &Pool, id: &str) -> Result<()> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    // Do not delete if there are still directories inside
    let dir_count = count_bucket_dirs(db_pool, id).await?;
    if dir_count > 0 {
        return Err(Error::ValidationError(
            "Cannot delete bucket with directories inside".to_string(),
        ));
    }

    let bucket_id = id.to_string();
    let conn_result = db
        .interact(move |conn| {
            diesel::delete(dsl::buckets.filter(dsl::id.eq(bucket_id.as_str()))).execute(conn)
        })
        .await;

    match conn_result {
        Ok(delete_res) => match delete_res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{}", e);
                Err("Error deleting bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}

pub async fn test_read_bucket(db_pool: &Pool) -> Result<()> {
    let Ok(db) = db_pool.get().await else {
        return Err("Error getting db connection".into());
    };

    let conn_result = db
        .interact(move |conn| {
            dsl::buckets
                .select(Bucket::as_select())
                .first::<Bucket>(conn)
                .optional()
        })
        .await;

    match conn_result {
        Ok(select_res) => match select_res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{}", e);
                Err("Error finding bucket".into())
            }
        },
        Err(e) => {
            error!("{}", e);
            Err("Error using the db connection".into())
        }
    }
}
