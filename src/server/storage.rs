use aws_sdk_s3::{primitives::ByteStream, Client};
use serde::{de::DeserializeOwned, Serialize};

use super::config::Config;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("S3 : {0}")]
    S3(String),
    #[error("Sérialisation : {0}")]
    Serde(String),
}

pub struct Storage {
    client: Client,
    pub bucket: String,
    pub region: String,
}

impl Storage {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let aws_cfg = aws_config::from_env().load().await;
        let client  = Client::new(&aws_cfg);
        Ok(Self {
            client,
            bucket: config.s3_bucket.clone(),
            region: config.s3_region.clone(),
        })
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StorageError> {
        let resp = self.client.get_object().bucket(&self.bucket).key(key).send().await;
        match resp {
            Ok(out) => {
                let data = out.body.collect().await
                    .map_err(|e| StorageError::S3(e.to_string()))?
                    .into_bytes();
                let val = serde_json::from_slice(&data)
                    .map_err(|e| StorageError::Serde(e.to_string()))?;
                Ok(Some(val))
            }
            Err(sdk_err) => {
                let svc = sdk_err.into_service_error();
                if svc.is_no_such_key() { Ok(None) } else { Err(StorageError::S3(svc.to_string())) }
            }
        }
    }

    pub async fn get_json_or_default<T: DeserializeOwned + Default>(&self, key: &str) -> Result<T, StorageError> {
        Ok(self.get_json::<T>(key).await?.unwrap_or_default())
    }

    pub async fn put_json<T: Serialize>(&self, key: &str, data: &T) -> Result<(), StorageError> {
        let bytes = serde_json::to_vec(data).map_err(|e| StorageError::Serde(e.to_string()))?;
        self.client.put_object()
            .bucket(&self.bucket).key(key)
            .content_type("application/json")
            .body(ByteStream::from(bytes))
            .send().await
            .map_err(|e| StorageError::S3(e.to_string()))?;
        Ok(())
    }

    pub async fn put_photo(&self, key: &str, data: Vec<u8>, content_type: String) -> Result<String, StorageError> {
        self.client.put_object()
            .bucket(&self.bucket).key(key)
            .content_type(&content_type)
            .body(ByteStream::from(data))
            .send().await
            .map_err(|e| StorageError::S3(e.to_string()))?;
        Ok(format!("https://{}.s3.{}.amazonaws.com/{}", self.bucket, self.region, key))
    }
}
