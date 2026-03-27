use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::ImageToVideoRequest;
use crate::types::task::TaskCreateResponse;

pub struct ImageToVideoResource {
    pub(crate) client: RunwayClient,
}

impl ImageToVideoResource {
    pub async fn create(
        &self,
        request: impl Into<ImageToVideoRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: impl Into<ImageToVideoRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/image_to_video", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.clone(), response.data.id),
            response: response.response,
        })
    }
}
