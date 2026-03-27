use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::ImageUpscaleRequest;
use crate::types::task::TaskCreateResponse;

pub struct ImageUpscaleResource {
    pub(crate) client: RunwayClient,
}

impl ImageUpscaleResource {
    pub async fn create(&self, request: ImageUpscaleRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/image_upscale", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
