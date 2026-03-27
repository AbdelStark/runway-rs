use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::TextToImageRequest;
use crate::types::task::TaskCreateResponse;

pub struct TextToImageResource {
    pub(crate) client: RunwayClient,
}

impl TextToImageResource {
    pub async fn create(&self, request: TextToImageRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/text_to_image", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
