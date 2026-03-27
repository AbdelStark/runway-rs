use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::voice::{
    CreateVoiceRequest, PreviewVoiceRequest, PreviewVoiceResponse, Voice, VoiceList,
};

pub struct VoicesResource {
    pub(crate) client: RunwayClient,
}

impl VoicesResource {
    pub async fn list(&self) -> Result<VoiceList, RunwayError> {
        self.client.get("/v1/voices").await
    }

    pub async fn get(&self, id: &str) -> Result<Voice, RunwayError> {
        self.client.get(&format!("/v1/voices/{}", id)).await
    }

    pub async fn create(&self, request: CreateVoiceRequest) -> Result<Voice, RunwayError> {
        self.client.post("/v1/voices", &request).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), RunwayError> {
        self.client.delete(&format!("/v1/voices/{}", id)).await
    }

    pub async fn preview(
        &self,
        request: PreviewVoiceRequest,
    ) -> Result<PreviewVoiceResponse, RunwayError> {
        self.client.post("/v1/voices/preview", &request).await
    }
}
