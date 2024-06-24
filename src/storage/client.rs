use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::buckets::get::GetBucketRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
use google_cloud_storage::http::Error as CloudError;

use crate::dirs::File;
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

            Ok(objects
                .iter()
                .map(|obj| {
                    let name = obj.name.clone();
                    let url = obj.media_link.clone();
                    File { name, url }
                })
                .collect())
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
