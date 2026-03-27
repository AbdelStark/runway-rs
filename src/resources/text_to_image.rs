use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::TextToImageRequest;
use crate::types::task::TaskCreateResponse;

pub struct TextToImageResource {
    pub(crate) client: RunwayClient,
}

impl TextToImageResource {
    pub async fn create(
        &self,
        request: impl Into<TextToImageRequest>,
    ) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: impl Into<TextToImageRequest>,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        let request = request.into();
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/text_to_image", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.clone(), response.data.id),
            response: response.response,
        })
    }
}
