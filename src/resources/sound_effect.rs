use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::SoundEffectRequest;
use crate::types::task::TaskCreateResponse;

pub struct SoundEffectResource {
    pub(crate) client: RunwayClient,
}

impl SoundEffectResource {
    pub async fn create(&self, request: SoundEffectRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/sound_effect", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
