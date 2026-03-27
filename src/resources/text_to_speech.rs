use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::TextToSpeechRequest;
use crate::types::task::TaskCreateResponse;

pub struct TextToSpeechResource {
    pub(crate) client: RunwayClient,
}

impl TextToSpeechResource {
    pub async fn create(&self, request: TextToSpeechRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse =
            self.client.post("/v1/text_to_speech", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
