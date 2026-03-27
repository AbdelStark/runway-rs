use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::TextToSpeechRequest;
use crate::types::task::TaskCreateResponse;

pub struct TextToSpeechResource {
    pub(crate) client: RunwayClient,
}

impl TextToSpeechResource {
    pub async fn create(&self, request: TextToSpeechRequest) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: TextToSpeechRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/text_to_speech", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.clone(), response.data.id),
            response: response.response,
        })
    }
}
