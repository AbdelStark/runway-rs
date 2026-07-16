//! Client for persisted realtime avatar conversation records.

use async_stream::try_stream;
use futures_core::Stream;

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::avatar_conversation::{
    AvatarConversation, AvatarConversationList, AvatarConversationListQuery,
};

/// Operations for listing, retrieving, and deleting avatar conversations.
pub struct AvatarConversationsResource {
    pub(crate) client: RunwayClient,
}

impl AvatarConversationsResource {
    /// List avatar conversations using cursor-based pagination.
    pub async fn list(
        &self,
        query: AvatarConversationListQuery,
    ) -> Result<AvatarConversationList, RunwayError> {
        Ok(self
            .list_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    /// List avatar conversations with per-request transport overrides.
    pub async fn list_with_options(
        &self,
        query: AvatarConversationListQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<AvatarConversationList>, RunwayError> {
        query.validate()?;
        self.client
            .get_with_query_with_options("/v1/avatar_conversations", &query, &options)
            .await
    }

    /// Stream conversation pages until the server returns no next cursor.
    pub fn list_pages(
        &self,
        query: AvatarConversationListQuery,
    ) -> impl Stream<Item = Result<AvatarConversationList, RunwayError>> {
        let client = self.client.clone();
        try_stream! {
            query.validate()?;
            let mut query = query;
            loop {
                let page: AvatarConversationList = client
                    .get_with_query_with_options(
                        "/v1/avatar_conversations",
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

    /// Stream individual conversation summaries across every cursor page.
    pub fn list_all(
        &self,
        query: AvatarConversationListQuery,
    ) -> impl Stream<
        Item = Result<crate::types::avatar_conversation::AvatarConversationSummary, RunwayError>,
    > {
        let client = self.client.clone();
        try_stream! {
            query.validate()?;
            let mut query = query;
            loop {
                let page: AvatarConversationList = client
                    .get_with_query_with_options(
                        "/v1/avatar_conversations",
                        &query,
                        &RequestOptions::default(),
                    )
                    .await?
                    .data;
                let next_cursor = page.next_cursor.clone();
                for conversation in page.data {
                    yield conversation;
                }
                let Some(next_cursor) = next_cursor.filter(|cursor| !cursor.is_empty()) else {
                    break;
                };
                query.cursor = Some(next_cursor);
            }
        }
    }

    /// Retrieve a conversation including its transcript and recording URL.
    pub async fn retrieve(&self, id: &str) -> Result<AvatarConversation, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    /// Retrieve a conversation with per-request transport overrides.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<AvatarConversation>, RunwayError> {
        let path = RunwayClient::path(&["v1", "avatar_conversations", id])?;
        self.client.get_with_options(&path, &options).await
    }

    /// Delete a conversation and its associated data.
    pub async fn delete(&self, id: &str) -> Result<(), RunwayError> {
        self.delete_with_options(id, RequestOptions::default())
            .await?;
        Ok(())
    }

    /// Delete a conversation with per-request transport overrides.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        let path = RunwayClient::path(&["v1", "avatar_conversations", id])?;
        self.client.delete_with_options(&path, &options).await
    }

    /// Alias for [`Self::retrieve`].
    pub async fn get(&self, id: &str) -> Result<AvatarConversation, RunwayError> {
        self.retrieve(id).await
    }
}
