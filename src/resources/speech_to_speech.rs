use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::SpeechToSpeechRequest;
use crate::types::task::TaskCreateResponse;

pub struct SpeechToSpeechResource {
    pub(crate) client: RunwayClient,
}

impl SpeechToSpeechResource {
    pub async fn create(
        &self,
        request: SpeechToSpeechRequest,
    ) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse =
            self.client.post("/v1/speech_to_speech", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
