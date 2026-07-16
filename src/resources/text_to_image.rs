//! Text-to-image generation operations.

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::task::TaskCreateResponse;
use crate::types::text_to_image::TextToImageCreateRequest;

/// Client for the text-to-image generation endpoint.
pub struct TextToImageResource {
    pub(crate) client: RunwayClient,
}

impl TextToImageResource {
    /// Start a text-to-image task using any supported model-specific request.
    pub async fn create(
        &self,
        request: impl Into<TextToImageCreateRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Start a text-to-image task with per-request transport overrides.
    pub async fn create_with_options(
        &self,
        request: impl Into<TextToImageCreateRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/text_to_image", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }
}
