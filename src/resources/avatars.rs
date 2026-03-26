use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::avatar::{Avatar, AvatarList, CreateAvatarRequest, UpdateAvatarRequest};

pub struct AvatarsResource {
    pub(crate) client: RunwayClient,
}

impl AvatarsResource {
    pub async fn list(&self) -> Result<AvatarList, RunwayError> {
        self.client.get("/v1/avatars").await
    }

    pub async fn get(&self, id: &str) -> Result<Avatar, RunwayError> {
        self.client.get(&format!("/v1/avatars/{}", id)).await
    }

    pub async fn create(&self, request: CreateAvatarRequest) -> Result<Avatar, RunwayError> {
        self.client.post("/v1/avatars", &request).await
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateAvatarRequest,
    ) -> Result<Avatar, RunwayError> {
        self.client
            .patch(&format!("/v1/avatars/{}", id), &request)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), RunwayError> {
        self.client.delete(&format!("/v1/avatars/{}", id)).await
    }
}
