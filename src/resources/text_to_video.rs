use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::TextToVideoRequest;
use crate::types::task::TaskCreateResponse;

pub struct TextToVideoResource {
    pub(crate) client: RunwayClient,
}

impl TextToVideoResource {
    pub async fn create(&self, request: TextToVideoRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/text_to_video", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
