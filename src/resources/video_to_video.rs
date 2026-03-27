use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::polling::PendingTask;
use crate::types::generation::VideoToVideoRequest;
use crate::types::task::TaskCreateResponse;

pub struct VideoToVideoResource {
    pub(crate) client: RunwayClient,
}

impl VideoToVideoResource {
    pub async fn create(&self, request: VideoToVideoRequest) -> Result<PendingTask, RunwayError> {
        let resp: TaskCreateResponse = self.client.post("/v1/video_to_video", &request).await?;
        Ok(PendingTask::new(self.client.clone(), resp.id))
    }
}
