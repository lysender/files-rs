use std::time::Duration;

use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::buckets::get::GetBucketRequest;
use google_cloud_storage::http::buckets::list::ListBucketsRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
use google_cloud_storage::http::Error as CloudError;
use google_cloud_storage::sign::SignedURLOptions;

use crate::files::File;
use crate::{Error, Result};

pub async fn read_bucket(name: &str) -> Result<String> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    let res = client
        .get_bucket(&GetBucketRequest {
            bucket: name.to_string(),
            ..Default::default()
        })
        .await;

    match res {
        Ok(bucket) => Ok(bucket.name),
        Err(e) => match e {
            CloudError::Response(gerr) => {
                if gerr.code >= 400 && gerr.code < 500 {
                    Err(Error::ValidationError(gerr.message))
                } else {
                    Err(format!("Google error: {}", gerr.message).as_str().into())
                }
            }
            _ => Err("Failed to read bucket from cloud storage.".into()),
        },
    }
}

pub async fn list_objects(bucket_name: &str, prefix: &str, dir: &str) -> Result<Vec<File>> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    let res = client
        .list_objects(&ListObjectsRequest {
            bucket: bucket_name.to_string(),
            prefix: Some(format!("{}{}", prefix, dir)),
            max_results: Some(1000),
            ..Default::default()
        })
        .await;

    match res {
        Ok(items) => {
            let Some(objects) = items.items else {
                return Ok(vec![]);
            };

            let files = objects
                .iter()
                .map(|obj| {
                    let name = obj.name.clone();
                    let url = obj.media_link.clone();
                    File { name, url }
                })
                .collect();

            Ok(signed_urls(bucket_name, &files).await?)
        }
        Err(e) => match e {
            CloudError::Response(gerr) => {
                if gerr.code >= 400 && gerr.code < 500 {
                    Err(Error::ValidationError(gerr.message))
                } else {
                    Err(format!("Google error: {}", gerr.message).as_str().into())
                }
            }
            _ => Err("Failed to list bucket objects from cloud storage.".into()),
        },
    }
}

pub async fn signed_urls(bucket_name: &str, files: &Vec<File>) -> Result<Vec<File>> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    let mut signed_files = Vec::with_capacity(files.len());
    for file in files.iter() {
        let mut updated_file = file.clone();
        let url = signed_url(&client, bucket_name, &file.name).await?;
        updated_file.url = url;
        signed_files.push(updated_file);
    }

    Ok(signed_files)
}

async fn signed_url(client: &Client, bucket_name: &str, object_name: &str) -> Result<String> {
    let mut options = SignedURLOptions::default();
    options.expires = Duration::from_secs(3600 * 24);

    let res = client
        .signed_url(bucket_name, &object_name, None, None, options)
        .await;

    match res {
        Ok(url) => Ok(url),
        Err(_) => Err("Unable to sign object URL.".into()),
    }
}

pub async fn test_list_buckets(project_id: &str) -> Result<()> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    let res = client
        .list_buckets(&ListBucketsRequest {
            project: project_id.to_string(),
            max_results: Some(1),
            ..Default::default()
        })
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => match e {
            CloudError::Response(gerr) => {
                if gerr.code >= 400 && gerr.code < 500 {
                    Err(Error::ValidationError(gerr.message))
                } else {
                    Err(format!("Google error: {}", gerr.message).as_str().into())
                }
            }
            _ => Err("Failed to list buckets from cloud storage.".into()),
        },
    }
}
