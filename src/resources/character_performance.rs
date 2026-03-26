use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::CharacterPerformanceRequest;
use crate::types::task::TaskCreateResponse;

pub struct CharacterPerformanceResource {
    pub(crate) client: RunwayClient,
}

impl CharacterPerformanceResource {
    pub async fn create(
        &self,
        request: CharacterPerformanceRequest,
    ) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self
            .client
            .post("/v1/character_performance", &request)
            .await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
