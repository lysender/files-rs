use crate::Result;
use crate::buckets::{NewBucket, create_bucket, delete_bucket};
use crate::config::BucketCommand;
use crate::db::create_db_pool;

use super::{get_bucket, list_buckets};

pub async fn run_bucket_command(cmd: BucketCommand) -> Result<()> {
    match cmd {
        BucketCommand::List { client_id } => run_list_buckets(client_id).await,
        BucketCommand::Create {
            client_id,
            name,
            images_only,
        } => run_create_bucket(client_id, name, images_only).await,
        BucketCommand::Delete { id } => run_delete_bucket(id).await,
    }
}

async fn run_list_buckets(client_id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let buckets = list_buckets(&db_pool, &client_id).await?;
    for bucket in buckets.iter() {
        println!(
            "{{ id = {}, name = {}, images_only = {} }}",
            bucket.id, bucket.name, bucket.images_only
        );
    }
    Ok(())
}

async fn run_create_bucket(client_id: String, name: String, images_only: String) -> Result<()> {
    let db_pool = create_db_pool();

    let res: Result<bool> = match images_only.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Invalid boolean".into()),
    };

    let Ok(img_only) = res else {
        return Err("images_only must be either true or false".into());
    };

    let data = NewBucket {
        name,
        images_only: img_only,
    };
    let bucket = create_bucket(&db_pool, &client_id, &data).await?;
    println!(
        "{{ id = {}, name = {}, images_only = {} }}",
        bucket.id, bucket.name, bucket.images_only
    );
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
