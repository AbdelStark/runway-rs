//! Stable image-upscale operations.

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::task::TaskCreateResponse;
use crate::types::upscale::ImageUpscaleCreateRequest;

/// Client for the stable image-upscale endpoint.
pub struct ImageUpscaleResource {
    pub(crate) client: RunwayClient,
}

impl ImageUpscaleResource {
    /// Start an image-upscale task.
    pub async fn create(
        &self,
        request: impl Into<ImageUpscaleCreateRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Start an image-upscale task with per-request transport overrides.
    pub async fn create_with_options(
        &self,
        request: impl Into<ImageUpscaleCreateRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/image_upscale", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }
}
