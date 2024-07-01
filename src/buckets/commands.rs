use crate::buckets::{create_bucket, delete_bucket, NewBucket};
use crate::config::BucketCommand;
use crate::db::create_db_pool;
use crate::Result;

use super::{get_bucket, list_buckets};

pub async fn run_bucket_command(cmd: BucketCommand) -> Result<()> {
    match cmd {
        BucketCommand::List { client_id } => run_list_buckets(client_id).await,
        BucketCommand::Create { client_id, name } => run_create_bucket(client_id, name).await,
        BucketCommand::Delete { id } => run_delete_bucket(id).await,
    }
}

async fn run_list_buckets(client_id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let buckets = list_buckets(&db_pool, &client_id).await?;
    for bucket in buckets.iter() {
        println!("ID: {}, Name: {}", bucket.id, bucket.name);
    }
    Ok(())
}

async fn run_create_bucket(client_id: String, name: String) -> Result<()> {
    let db_pool = create_db_pool();

    let data = NewBucket { name };
    let bucket = create_bucket(&db_pool, &client_id, &data).await?;
    println!("ID: {}, Name: {}", bucket.id, bucket.name);
    println!("Created bucket.");
    Ok(())
}

async fn run_delete_bucket(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let bucket = get_bucket(&db_pool, &id).await?;
    if let Some(_) = bucket {
        let _ = delete_bucket(&db_pool, &id).await?;
        println!("Bucket deleted.");
    } else {
        println!("Bucket not found.");
    }
    Ok(())
}
