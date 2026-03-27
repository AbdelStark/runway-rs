use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VoiceDubbingRequest;
use crate::types::task::TaskCreateResponse;

pub struct VoiceDubbingResource {
    pub(crate) client: RunwayClient,
}

impl VoiceDubbingResource {
    pub async fn create(&self, request: VoiceDubbingRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/voice_dubbing", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
