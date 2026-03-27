use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VoiceIsolationRequest;
use crate::types::task::TaskCreateResponse;

pub struct VoiceIsolationResource {
    pub(crate) client: RunwayClient,
}

impl VoiceIsolationResource {
    pub async fn create(&self, request: VoiceIsolationRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/voice_isolation", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
