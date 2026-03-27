use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VoiceIsolationRequest;
use crate::types::task::TaskCreateResponse;

pub struct VoiceIsolationResource {
    pub(crate) client: RunwayClient,
}

impl VoiceIsolationResource {
    pub async fn create(&self, request: VoiceIsolationRequest) -> Result<PendingTask, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: VoiceIsolationRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingTask>, RunwayError> {
        request.validate()?;
        let response: WithResponse<TaskCreateResponse> = self
            .client
            .post_with_options("/v1/voice_isolation", &request, &options)
            .await?;
        Ok(WithResponse {
            data: PendingTask::new(self.client.clone(), response.data.id),
            response: response.response,
        })
    }
}
