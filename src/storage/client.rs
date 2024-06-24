use cloud_storage::Client;
use cloud_storage::Error as CloudError;

use crate::{Error, Result};

pub async fn read_bucket(name: &str) -> Result<String> {
    let client = Client::new();
    let res = client.bucket().read(name).await;
    match res {
        Ok(bucket) => Ok(bucket.name),
        Err(e) => match e {
            CloudError::Google(gerr) => {
                if gerr.error.code >= 400 && gerr.error.code < 500 {
                    Err(Error::ValidationError(gerr.error.message))
                } else {
                    Err(format!("Google error: {}", gerr.error.message)
                        .as_str()
                        .into())
                }
            }
            _ => Err("Failed to read bucket from cloud storage.".into()),
        },
    }
}
