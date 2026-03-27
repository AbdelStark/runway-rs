use std::path::Path;

use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};

use crate::client::{RequestOptions, ResponseMetadata, RunwayClient, WithResponse};
use crate::error::{ApiErrorKind, RunwayError};
use crate::types::generation::{
    CreateEphemeralUploadRequest, CreateUploadRequest, CreateUploadResponse,
    UploadCreateEphemeralResponse, UploadType,
};

pub struct UploadsResource {
    pub(crate) client: RunwayClient,
}

impl UploadsResource {
    async fn start_ephemeral_upload(
        &self,
        filename: impl Into<String>,
        options: &RequestOptions,
    ) -> Result<CreateUploadResponse, RunwayError> {
        let req = CreateUploadRequest {
            filename: filename.into(),
            upload_type: UploadType::Ephemeral,
        };
        Ok(self
            .client
            .post_with_options("/v1/uploads", &req, options)
            .await?
            .data)
    }

    fn build_storage_form(
        request: CreateEphemeralUploadRequest,
        fields: CreateUploadResponse,
    ) -> Result<Form, RunwayError> {
        let content_type = request
            .content_type
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let part = Part::bytes(request.bytes)
            .file_name(request.filename)
            .mime_str(&content_type)
            .map_err(|err| RunwayError::Validation {
                message: format!("Invalid content type for upload: {err}"),
            })?;

        let mut form = Form::new();
        for (key, value) in fields.fields {
            form = form.text(key, value);
        }
        if let Some(metadata) = request.file_metadata {
            form = form.text("metadata", metadata);
        }
        Ok(form.part("file", part))
    }

    /// Uploads an ephemeral media asset and returns its `runway://` URI.
    pub async fn create_ephemeral(
        &self,
        request: CreateEphemeralUploadRequest,
    ) -> Result<UploadCreateEphemeralResponse, RunwayError> {
        Ok(self
            .create_ephemeral_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_ephemeral_with_options(
        &self,
        request: CreateEphemeralUploadRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<UploadCreateEphemeralResponse>, RunwayError> {
        if request.filename.trim().is_empty() {
            return Err(RunwayError::Validation {
                message: "Upload filename cannot be empty".into(),
            });
        }

        let placeholder = self
            .start_ephemeral_upload(request.filename.clone(), &options)
            .await?;
        let runway_uri = placeholder.runway_uri.clone();
        let upload_url = placeholder.upload_url.clone();
        let form = Self::build_storage_form(request, placeholder)?;

        // Use a separate client without default auth headers for the presigned URL.
        // The presigned URL already contains authentication; sending our API key
        // to a third-party storage endpoint would be a credential leak.
        let upload_http = reqwest::Client::new();
        let mut upload_request = upload_http.post(&upload_url).multipart(form);
        if let Some(timeout) = options.timeout {
            upload_request = upload_request.timeout(timeout);
        }
        let upload_resp = upload_request.send().await?;

        if !upload_resp.status().is_success() {
            let status = upload_resp.status().as_u16();
            let text = upload_resp.text().await.unwrap_or_default();
            return Err(RunwayError::Api {
                status,
                kind: if status >= 500 {
                    ApiErrorKind::InternalServer
                } else {
                    ApiErrorKind::Unknown
                },
                message: format!("Upload to presigned URL failed: {}", text),
                code: None,
                headers: Box::new(HeaderMap::new()),
            });
        }

        Ok(WithResponse {
            data: UploadCreateEphemeralResponse { uri: runway_uri },
            response: ResponseMetadata {
                status: 200,
                headers: HeaderMap::new(),
            },
        })
    }

    /// Upload a local file and return the resulting `runway://` URI.
    pub async fn upload_file(&self, path: &Path) -> Result<String, RunwayError> {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| RunwayError::Validation {
                message: format!("Invalid file path: {}", path.display()),
            })?
            .to_string();

        let data = tokio::fs::read(path).await?;
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        let response = self
            .create_ephemeral(CreateEphemeralUploadRequest::new(filename, data).content_type(mime))
            .await?;

        Ok(response.uri)
    }
}
