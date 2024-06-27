use std::time::Duration;

use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::buckets::get::GetBucketRequest;
use google_cloud_storage::http::buckets::list::ListBucketsRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
use google_cloud_storage::http::Error as CloudError;
use google_cloud_storage::sign::SignedURLOptions;

use crate::files::{File, FileUrls};
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

pub async fn list_objects(bucket_name: &str, dir: &str) -> Result<Vec<File>> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    // List objects from the original image sizes
    let prefix = format!("o/{}/", dir);
    let res = client
        .list_objects(&ListObjectsRequest {
            bucket: bucket_name.to_string(),
            prefix: Some(prefix.clone()),
            max_results: Some(1000),
            ..Default::default()
        })
        .await;

    match res {
        Ok(items) => {
            let Some(objects) = items.items else {
                return Ok(vec![]);
            };
            let full_prefix = format!("{}", prefix);

            let files = objects
                .iter()
                .map(|obj| {
                    let mut name = obj.name.clone();
                    name = name.replace(&full_prefix, "");
                    File {
                        name,
                        urls: FileUrls::new(),
                    }
                })
                .collect();

            format_files(bucket_name, dir, &files).await
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

pub async fn format_files(bucket_name: &str, dir: &str, files: &Vec<File>) -> Result<Vec<File>> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    let mut tasks = Vec::with_capacity(files.len());
    for file in files.iter() {
        let client_copy = client.clone();
        let file_copy = file.clone();
        let bname = bucket_name.to_string();
        let dir_name = dir.to_string();

        tasks.push(tokio::spawn(async move {
            format_file(&client_copy, &bname, &dir_name, &file_copy).await
        }));
    }

    let mut updated_files: Vec<File> = Vec::with_capacity(files.len());
    for task in tasks {
        let Ok(res) = task.await else {
            return Err("Unable to extract data from spanwed task.".into());
        };
        let file = res?;
        updated_files.push(file);
    }

    Ok(updated_files)
}

async fn format_file(client: &Client, bucket_name: &str, dir: &str, file: &File) -> Result<File> {
    let expires = Duration::from_secs(3600 * 24);
    let mut options = SignedURLOptions::default();
    options.expires = expires;

    let orig_path = format!("o/{}/{}", dir, file.name);
    let thumb_path = format!("s/{}/{}", dir, file.name);

    let Ok(orig_url) = client
        .signed_url(bucket_name, &orig_path, None, None, options)
        .await
    else {
        return Err("Unable to sign object URL.".into());
    };

    // For some reason, we cannot clone options
    let mut options = SignedURLOptions::default();
    options.expires = expires;

    let Ok(thumb_url) = client
        .signed_url(bucket_name, &thumb_path, None, None, options)
        .await
    else {
        return Err("Unable to sign object URL.".into());
    };

    Ok(File {
        name: file.name.clone(),
        urls: FileUrls {
            o: orig_url,
            s: thumb_url,
        },
    })
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
