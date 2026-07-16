//! Client for speech-driven avatar video generation.

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::avatar_video::AvatarVideoCreateRequest;
use crate::types::task::TaskCreateResponse;

/// Operations for generating videos of avatars speaking.
pub struct AvatarVideosResource {
    pub(crate) client: RunwayClient,
}

impl AvatarVideosResource {
    /// Start an avatar-video generation task.
    pub async fn create(
        &self,
        request: AvatarVideoCreateRequest,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Start an avatar-video task with per-request transport overrides.
    pub async fn create_with_options(
        &self,
        request: AvatarVideoCreateRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/avatar_videos", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }
}
