use std::path::PathBuf;
use std::time::Duration;

use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::buckets::get::GetBucketRequest;
use google_cloud_storage::http::buckets::list::ListBucketsRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::http::Error as CloudError;
use google_cloud_storage::sign::SignedURLOptions;

use crate::buckets::Bucket;
use crate::dirs::Dir;
use crate::files::{FileDto, FileDtox, FileUrls, ORIGINAL_PATH};
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

pub async fn upload_object(
    bucket: &Bucket,
    dir: &Dir,
    source_dir: &PathBuf,
    file: &FileDtox,
) -> Result<FileDtox> {
    match file.is_image {
        true => upload_image_object(bucket, dir, source_dir, file).await,
        false => upload_regular_object(bucket, dir, source_dir, file).await,
    }
}

async fn upload_regular_object(
    bucket: &Bucket,
    dir: &Dir,
    source_dir: &PathBuf,
    file: &FileDtox,
) -> Result<FileDtox> {
    let Ok(config) = ClientConfig::default().with_auth().await else {
        return Err("Failed to initialize storage client configuration.".into());
    };
    let client = Client::new(config);

    // Prepare media
    let file_path = format!("{}/{}/{}", &dir.name, ORIGINAL_PATH, &file.filename);
    let mut media = Media::new(file_path.clone());
    media.content_type = file.content_type.clone().into();
    let upload_type = UploadType::Simple(media);

    // Read file, preferred a stream but skill issues...
    let source_path = source_dir.join(ORIGINAL_PATH).join(&file.filename);
    let Ok(data) = std::fs::read(&source_path) else {
        return Err("Failed to read file for upload.".into());
    };

    let upload_res = client
        .upload_object(
            &UploadObjectRequest {
                bucket: bucket.name.clone(),
                ..Default::default()
            },
            data,
            &upload_type,
        )
        .await;

    match upload_res {
        Ok(_) => {
            let mut file_copy = file.clone();
            let url = generate_url(&client, &bucket.name, &file_path).await?;
            file_copy.url = Some(url);
            Ok(file_copy)
        }
        Err(e) => match e {
            CloudError::Response(gerr) => {
                if gerr.code >= 400 && gerr.code < 500 {
                    Err(Error::ValidationError(gerr.message))
                } else {
                    Err(format!("Google error: {}", gerr.message).as_str().into())
                }
            }
            _ => Err("Failed to upload object to cloud storage.".into()),
        },
    }
}

async fn upload_image_object(
    bucket: &Bucket,
    dir: &Dir,
    source_dir: &PathBuf,
    file: &FileDtox,
) -> Result<FileDtox> {
    todo!()
}

pub async fn list_objects(bucket_name: &str, dir: &str) -> Result<Vec<FileDto>> {
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
                    FileDto {
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

pub async fn format_files(
    bucket_name: &str,
    dir: &str,
    files: &Vec<FileDto>,
) -> Result<Vec<FileDto>> {
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

    let mut updated_files: Vec<FileDto> = Vec::with_capacity(files.len());
    for task in tasks {
        let Ok(res) = task.await else {
            return Err("Unable to extract data from spanwed task.".into());
        };
        let file = res?;
        updated_files.push(file);
    }

    Ok(updated_files)
}

async fn format_file(
    client: &Client,
    bucket_name: &str,
    dir: &str,
    file: &FileDto,
) -> Result<FileDto> {
    let expires = Duration::from_secs(3600 * 12);
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

    Ok(FileDto {
        name: file.name.clone(),
        urls: FileUrls {
            o: orig_url,
            s: thumb_url,
        },
    })
}

async fn generate_url(client: &Client, bucket_name: &str, file_path: &str) -> Result<String> {
    let expires = Duration::from_secs(3600 * 12);
    let mut options = SignedURLOptions::default();
    options.expires = expires;

    let res = client
        .signed_url(bucket_name, file_path, None, None, options)
        .await;

    match res {
        Ok(url) => Ok(url),
        Err(_) => Err("Failed to sign object URL.".into()),
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
