use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::LipSyncRequest;
use crate::types::task::TaskCreateResponse;

pub struct LipSyncResource {
    pub(crate) client: RunwayClient,
}

impl LipSyncResource {
    pub async fn create(&self, request: LipSyncRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/lip_sync", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
