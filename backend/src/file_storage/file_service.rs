use crate::r2_client::R2Client;
use aws_sdk_s3::presigning::PresigningConfig;
use bytes::Bytes;
use chrono::Utc;
use std::time::Duration;

#[derive(Clone)]
pub struct FileService {
    r2_client: R2Client,
}

#[derive(serde::Serialize)]
pub struct UploadResponse {
    pub key: String,
    pub url: String,
}

#[derive(serde::Serialize)]
pub struct SignedUrlResponse {
    pub upload_url: String,
    pub key: String,
}

#[derive(serde::Serialize)]
pub struct DownloadResponse {
    pub download_url: String,
}

#[derive(serde::Serialize)]
pub struct ViewResponse {
    pub content_type: String,
    pub content_length: i64,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

impl FileService {
    pub fn new(r2_client: R2Client) -> Self {
        Self { r2_client }
    }

    pub async fn upload_file(
        &self,
        filename: &str,
        content: Bytes,
        content_type: &str,
    ) -> anyhow::Result<UploadResponse> {

        let timestamp = Utc::now().timestamp_millis();
        let key = format!("uploads/{}-{}", timestamp, filename);

        self.r2_client
        .client
        .put_object()
        .bucket(&self.r2_client.bucket_name)
        .key(&key)
        .body(content.into())
        .content_type(content_type)
        .send()
        .await?;

        Ok(UploadResponse { key: key.clone(), url: format!("https://{}.r2.cloudflarestorage.com/{}", self.r2_client.bucket_name, key) })
    }

    pub async fn upload_file_url(
        &self,
        filename: &str,
        content_type: &str,
    ) -> anyhow::Result<SignedUrlResponse> {
        let timestamp = Utc::now().timestamp_millis();
        let key = format!("uploads/{}-{}", timestamp, filename);
        let expires_in = 3600u64; // 1 hour

        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expires_in))?;

        let presigned_url = self.r2_client
        .client
        .put_object()
        .bucket(&self.r2_client.bucket_name)
        .key(&key)
        .content_type(content_type)
        .presigned(presigning_config)
        .await?;

        Ok(SignedUrlResponse {
            upload_url: presigned_url.uri().to_string(),
            key,
        })
    }

    pub async fn download_file(&self, key: &str) -> anyhow::Result<DownloadResponse> {
        let expires_in = 3600u64; // 1 hour
        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expires_in))?;

        let presigned_url = self.r2_client
            .client
            .get_object()
            .bucket(&self.r2_client.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(DownloadResponse {
            download_url: presigned_url.uri().to_string(),
        })
    }

    pub async fn view_file(&self, key: &str) -> anyhow::Result<ViewResponse>{
        let response = self.r2_client.client.get_object().bucket(&self.r2_client.bucket_name).key(key).send().await?;
        let content_type = response.content_type.clone().unwrap_or_default();
        let content_length = response.content_length.unwrap_or(0);
        let body_bytes = response.body.collect().await?.into_bytes().to_vec();

        Ok(ViewResponse {
            content_type,
            content_length,
            body: body_bytes,
        })
    }

    pub async fn delete_file(&self, key: &str) -> anyhow::Result<()> {
        self.r2_client.client.delete_object().bucket(&self.r2_client.bucket_name).key(key).send().await?;
        Ok(())
    }
}
