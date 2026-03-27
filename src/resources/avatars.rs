use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::avatar::{
    Avatar, AvatarList, AvatarListQuery, CreateAvatarRequest, UpdateAvatarRequest,
};

pub struct AvatarsResource {
    pub(crate) client: RunwayClient,
}

impl AvatarsResource {
    pub async fn list(&self, query: AvatarListQuery) -> Result<AvatarList, RunwayError> {
        Ok(self
            .list_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn list_with_options(
        &self,
        query: AvatarListQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<AvatarList>, RunwayError> {
        self.client
            .get_with_query_with_options("/v1/avatars", &query, &options)
            .await
    }

    pub async fn retrieve(&self, id: &str) -> Result<Avatar, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<Avatar>, RunwayError> {
        self.client
            .get_with_options(&format!("/v1/avatars/{}", id), &options)
            .await
    }

    pub async fn create(&self, request: CreateAvatarRequest) -> Result<Avatar, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: CreateAvatarRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<Avatar>, RunwayError> {
        self.client
            .post_with_options("/v1/avatars", &request, &options)
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateAvatarRequest,
    ) -> Result<Avatar, RunwayError> {
        Ok(self
            .update_with_options(id, request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn update_with_options(
        &self,
        id: &str,
        request: UpdateAvatarRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<Avatar>, RunwayError> {
        self.client
            .patch_with_options(&format!("/v1/avatars/{}", id), &request, &options)
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
            .delete_with_options(&format!("/v1/avatars/{}", id), &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Avatar, RunwayError> {
        self.retrieve(id).await
    }
}
