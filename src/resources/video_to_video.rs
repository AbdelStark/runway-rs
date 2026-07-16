use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VideoToVideoCreateRequest;
use crate::types::task::TaskCreateResponse;

pub struct VideoToVideoResource {
    pub(crate) client: RunwayClient,
}

impl VideoToVideoResource {
    /// Start a video-to-video generation task.
    pub async fn create(
        &self,
        request: impl Into<VideoToVideoCreateRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    /// Start a generation with per-request transport options and retain the response metadata.
    pub async fn create_with_options(
        &self,
        request: impl Into<VideoToVideoCreateRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/video_to_video", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.continuation_client(&options)?, response.data.id),
            response: response.response,
        })
    }
}
