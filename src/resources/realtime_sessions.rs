use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::realtime::{CreateRealtimeSessionRequest, RealtimeSession};

pub struct RealtimeSessionsResource {
    pub(crate) client: RunwayClient,
}

impl RealtimeSessionsResource {
    pub async fn create(
        &self,
        request: CreateRealtimeSessionRequest,
    ) -> Result<RealtimeSession, RunwayError> {
        self.client.post("/v1/realtime_sessions", &request).await
    }

    pub async fn get(&self, id: &str) -> Result<RealtimeSession, RunwayError> {
        self.client
            .get(&format!("/v1/realtime_sessions/{}", id))
            .await
    }

    pub async fn cancel(&self, id: &str) -> Result<(), RunwayError> {
        self.client
            .delete(&format!("/v1/realtime_sessions/{}", id))
            .await
    }
}
