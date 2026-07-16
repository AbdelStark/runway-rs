//! Text-to-speech generation operations.

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::audio_generation::TextToSpeechCreateRequest;
use crate::types::task::TaskCreateResponse;

/// Client for the text-to-speech generation endpoint.
pub struct TextToSpeechResource {
    pub(crate) client: RunwayClient,
}

impl TextToSpeechResource {
    /// Start a text-to-speech task using Seed Audio or Eleven Multilingual V2.
    pub async fn create(
        &self,
        request: impl Into<TextToSpeechCreateRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Start a text-to-speech task with per-request transport overrides.
    pub async fn create_with_options(
        &self,
        request: impl Into<TextToSpeechCreateRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/text_to_speech", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }
}
