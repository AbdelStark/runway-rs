use std::path::Path;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::generation::{CreateUploadRequest, CreateUploadResponse};

pub struct UploadsResource {
    pub(crate) client: RunwayClient,
}

impl UploadsResource {
    /// Create a new upload and get back a presigned URL.
    pub async fn create(&self, filename: impl Into<String>) -> Result<CreateUploadResponse, RunwayError> {
        let req = CreateUploadRequest {
            filename: filename.into(),
        };
        self.client.post("/v1/uploads", &req).await
    }

    /// Upload a local file: creates the upload, PUTs the file to the presigned URL,
    /// and returns the runway:// URI to use in generation requests.
    pub async fn upload_file(&self, path: &Path) -> Result<String, RunwayError> {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| RunwayError::Validation {
                message: "Invalid file path".into(),
            })?
            .to_string();

        let resp = self.create(&filename).await?;

        let data = tokio::fs::read(path).await?;
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        self.client
            .inner
            .http
            .put(&resp.upload_url)
            .header("Content-Type", mime)
            .body(data)
            .send()
            .await?
            .error_for_status()?;

        Ok(format!("runway://{}", resp.id))
    }
}
