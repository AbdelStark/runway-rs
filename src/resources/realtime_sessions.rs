use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::realtime::{
    CreateRealtimeSessionRequest, RealtimeSession, RealtimeSessionCreateResponse,
};

pub struct RealtimeSessionsResource {
    pub(crate) client: RunwayClient,
}

impl RealtimeSessionsResource {
    pub async fn create(
        &self,
        request: CreateRealtimeSessionRequest,
    ) -> Result<RealtimeSessionCreateResponse, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: CreateRealtimeSessionRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<RealtimeSessionCreateResponse>, RunwayError> {
        self.client
            .post_with_options("/v1/realtime_sessions", &request, &options)
            .await
    }

    pub async fn retrieve(&self, id: &str) -> Result<RealtimeSession, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<RealtimeSession>, RunwayError> {
        self.client
            .get_with_options(&format!("/v1/realtime_sessions/{}", id), &options)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), RunwayError> {
        self.delete_with_options(id, RequestOptions::default())
            .await?;
        Ok(())
    }

    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        self.client
            .delete_with_options(&format!("/v1/realtime_sessions/{}", id), &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<RealtimeSession, RunwayError> {
        self.retrieve(id).await
    }

    pub async fn cancel(&self, id: &str) -> Result<(), RunwayError> {
        self.delete(id).await
    }
}
