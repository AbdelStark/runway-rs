use async_stream::try_stream;
use futures_core::Stream;

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::avatar::{
    Avatar, AvatarList, AvatarListQuery, AvatarUsage, AvatarUsageQuery, CreateAvatarRequest,
    UpdateAvatarRequest,
};

pub struct AvatarsResource {
    pub(crate) client: RunwayClient,
}

impl AvatarsResource {
    fn validate_list_query(query: &AvatarListQuery) -> Result<(), RunwayError> {
        if query.limit == Some(0) {
            return Err(RunwayError::Validation {
                message: "Avatar list limit must be greater than zero".into(),
            });
        }
        Ok(())
    }

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
        Self::validate_list_query(&query)?;
        self.client
            .get_with_query_with_options("/v1/avatars", &query, &options)
            .await
    }

    /// Stream avatar pages until the server returns no next cursor.
    pub fn list_pages(
        &self,
        query: AvatarListQuery,
    ) -> impl Stream<Item = Result<AvatarList, RunwayError>> {
        let client = self.client.clone();
        try_stream! {
            Self::validate_list_query(&query)?;
            let mut query = query;
            loop {
                let page: AvatarList = client
                    .get_with_query_with_options(
                        "/v1/avatars",
                        &query,
                        &RequestOptions::default(),
                    )
                    .await?
                    .data;
                let next_cursor = page.next_cursor.clone();
                yield page;
                let Some(next_cursor) = next_cursor.filter(|cursor| !cursor.is_empty()) else {
                    break;
                };
                query.cursor = Some(next_cursor);
            }
        }
    }

    /// Stream individual avatars across every cursor page.
    pub fn list_all(
        &self,
        query: AvatarListQuery,
    ) -> impl Stream<Item = Result<Avatar, RunwayError>> {
        let client = self.client.clone();
        try_stream! {
            Self::validate_list_query(&query)?;
            let mut query = query;
            loop {
                let page: AvatarList = client
                    .get_with_query_with_options(
                        "/v1/avatars",
                        &query,
                        &RequestOptions::default(),
                    )
                    .await?
                    .data;
                let next_cursor = page.next_cursor.clone();
                for avatar in page.data {
                    yield avatar;
                }
                let Some(next_cursor) = next_cursor.filter(|cursor| !cursor.is_empty()) else {
                    break;
                };
                query.cursor = Some(next_cursor);
            }
        }
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
        let path = RunwayClient::path(&["v1", "avatars", id])?;
        self.client.get_with_options(&path, &options).await
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
        let path = RunwayClient::path(&["v1", "avatars", id])?;
        self.client
            .patch_with_options(&path, &request, &options)
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
        let path = RunwayClient::path(&["v1", "avatars", id])?;
        self.client.delete_with_options(&path, &options).await
    }

    pub async fn get(&self, id: &str) -> Result<Avatar, RunwayError> {
        self.retrieve(id).await
    }

    /// Get aggregate avatar-conversation usage for a UTC date range.
    pub async fn get_usage(&self, query: AvatarUsageQuery) -> Result<AvatarUsage, RunwayError> {
        Ok(self
            .get_usage_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    /// Get avatar-conversation usage with per-request transport overrides.
    pub async fn get_usage_with_options(
        &self,
        query: AvatarUsageQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<AvatarUsage>, RunwayError> {
        self.client
            .get_with_query_with_options("/v1/avatar_usage", &query, &options)
            .await
    }
}
