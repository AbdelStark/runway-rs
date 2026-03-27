use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VideoToVideoRequest;
use crate::types::task::TaskCreateResponse;

pub struct VideoToVideoResource {
    pub(crate) client: RunwayClient,
}

impl VideoToVideoResource {
    pub async fn create(&self, request: VideoToVideoRequest) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: VideoToVideoRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/video_to_video", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.clone(), response.data.id),
            response: response.response,
        })
    }
}
