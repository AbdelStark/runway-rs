use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::ImageToVideoRequest;
use crate::types::task::TaskCreateResponse;

pub struct ImageToVideoResource {
    pub(crate) client: RunwayClient,
}

impl ImageToVideoResource {
    pub async fn create(&self, request: ImageToVideoRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/image_to_video", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
