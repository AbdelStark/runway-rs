use std::path::Path;

use reqwest::multipart::{Form, Part};
use tokio_util::io::ReaderStream;

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
        part: Part,
        file_metadata: Option<String>,
        fields: CreateUploadResponse,
    ) -> Form {
        let mut form = Form::new();
        for (key, value) in fields.fields {
            form = form.text(key, value);
        }
        if let Some(metadata) = file_metadata {
            form = form.text("metadata", metadata);
        }
        form.part("file", part)
    }

    async fn upload_part(
        &self,
        filename: String,
        part: Part,
        file_metadata: Option<String>,
        options: RequestOptions,
    ) -> Result<WithResponse<UploadCreateEphemeralResponse>, RunwayError> {
        if filename.trim().is_empty() {
            return Err(RunwayError::Validation {
                message: "Upload filename cannot be empty".into(),
            });
        }

        let placeholder = self.start_ephemeral_upload(&filename, &options).await?;
        let runway_uri = placeholder.runway_uri.clone();
        let upload_url = placeholder.upload_url.clone();
        let form = Self::build_storage_form(part, file_metadata, placeholder);

        // This dedicated client never inherits the Runway Authorization header.
        let mut upload_request = self
            .client
            .inner
            .storage_http
            .post(&upload_url)
            .multipart(form);
        if let Some(timeout) = options.timeout {
            upload_request = upload_request.timeout(timeout);
        }

        let send = upload_request.send();
        let upload_resp = if let Some(cancellation_token) = options.cancellation_token.as_ref() {
            tokio::select! {
                response = send => response?,
                _ = cancellation_token.cancelled() => return Err(RunwayError::RequestAborted),
            }
        } else {
            send.await?
        };

        let status = upload_resp.status();
        let headers = upload_resp.headers().clone();
        if !status.is_success() {
            const MAX_UPLOAD_ERROR_BYTES: usize = 64 * 1024;
            let mut upload_resp = upload_resp;
            let mut body = Vec::new();
            loop {
                let next_chunk = upload_resp.chunk();
                let chunk = if let Some(cancellation_token) = options.cancellation_token.as_ref() {
                    tokio::select! {
                        chunk = next_chunk => chunk?,
                        _ = cancellation_token.cancelled() => return Err(RunwayError::RequestAborted),
                    }
                } else {
                    next_chunk.await?
                };
                let Some(chunk) = chunk else {
                    break;
                };
                let remaining = MAX_UPLOAD_ERROR_BYTES.saturating_sub(body.len());
                if remaining == 0 {
                    break;
                }
                body.extend_from_slice(&chunk[..chunk.len().min(remaining)]);
            }
            let text = String::from_utf8_lossy(&body);
            return Err(RunwayError::Api {
                status: status.as_u16(),
                kind: if status.is_server_error() {
                    ApiErrorKind::InternalServer
                } else {
                    ApiErrorKind::Unknown
                },
                message: format!("Upload to presigned URL failed: {text}"),
                code: None,
                headers: Box::new(headers.into()),
            });
        }

        Ok(WithResponse {
            data: UploadCreateEphemeralResponse { uri: runway_uri },
            response: ResponseMetadata {
                status: status.as_u16(),
                headers,
            },
        })
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
        let content_type = request
            .content_type
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let part = Part::bytes(request.bytes)
            .file_name(request.filename.clone())
            .mime_str(&content_type)
            .map_err(|error| RunwayError::Validation {
                message: format!("Invalid content type for upload: {error}"),
            })?;
        self.upload_part(request.filename, part, request.file_metadata, options)
            .await
    }

    /// Upload a local file and return the resulting `runway://` URI.
    pub async fn upload_file(&self, path: &Path) -> Result<String, RunwayError> {
        Ok(self
            .upload_file_with_options(path, RequestOptions::default())
            .await?
            .data
            .uri)
    }

    /// Stream a local file to Runway without buffering the complete asset in memory.
    pub async fn upload_file_with_options(
        &self,
        path: &Path,
        options: RequestOptions,
    ) -> Result<WithResponse<UploadCreateEphemeralResponse>, RunwayError> {
        self.upload_file_with_metadata_and_options(path, None, options)
            .await
    }

    /// Stream a local file with optional storage metadata and request overrides.
    pub async fn upload_file_with_metadata_and_options(
        &self,
        path: &Path,
        file_metadata: Option<String>,
        options: RequestOptions,
    ) -> Result<WithResponse<UploadCreateEphemeralResponse>, RunwayError> {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| RunwayError::Validation {
                message: format!("Invalid file path: {}", path.display()),
            })?
            .to_string();
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        let file = tokio::fs::File::open(path).await?;
        let length = file.metadata().await?.len();
        let body = reqwest::Body::wrap_stream(ReaderStream::new(file));
        let part = Part::stream_with_length(body, length)
            .file_name(filename.clone())
            .mime_str(&mime)
            .map_err(|error| RunwayError::Validation {
                message: format!("Invalid content type for upload: {error}"),
            })?;

        self.upload_part(filename, part, file_metadata, options)
            .await
    }
}
