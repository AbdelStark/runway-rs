use std::path::Path;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::generation::{CreateUploadRequest, CreateUploadResponse};

pub struct UploadsResource {
    pub(crate) client: RunwayClient,
}

impl UploadsResource {
    /// Create a new upload and get back a presigned URL.
    pub async fn create(
        &self,
        filename: impl Into<String>,
    ) -> Result<CreateUploadResponse, RunwayError> {
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
                message: format!("Invalid file path: {}", path.display()),
            })?
            .to_string();

        let resp = self.create(&filename).await?;

        let data = tokio::fs::read(path).await?;
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        // Use a separate client without default auth headers for the presigned URL.
        // The presigned URL already contains authentication; sending our API key
        // to a third-party storage endpoint would be a credential leak.
        let upload_http = reqwest::Client::new();
        let upload_resp = upload_http
            .put(&resp.upload_url)
            .header("Content-Type", mime)
            .body(data)
            .send()
            .await?;

        if !upload_resp.status().is_success() {
            let status = upload_resp.status().as_u16();
            let text = upload_resp.text().await.unwrap_or_default();
            return Err(RunwayError::Api {
                status,
                message: format!("Upload to presigned URL failed: {}", text),
                code: None,
            });
        }

        Ok(format!("runway://{}", resp.id))
    }
}
