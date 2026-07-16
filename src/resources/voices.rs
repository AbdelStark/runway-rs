use async_stream::try_stream;
use futures_core::Stream;

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::voice::{
    CreateVoiceRequest, PreviewVoiceRequest, PreviewVoiceResponse, UpdateVoiceRequest, Voice,
    VoiceCreateResponse, VoiceList, VoiceListQuery,
};

pub struct VoicesResource {
    pub(crate) client: RunwayClient,
}

impl VoicesResource {
    fn validate_list_query(query: &VoiceListQuery) -> Result<(), RunwayError> {
        if query.limit == Some(0) {
            return Err(RunwayError::Validation {
                message: "Voice list limit must be greater than zero".into(),
            });
        }
        Ok(())
    }

    pub async fn list(&self, query: VoiceListQuery) -> Result<VoiceList, RunwayError> {
        Ok(self
            .list_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn list_with_options(
        &self,
        query: VoiceListQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<VoiceList>, RunwayError> {
        Self::validate_list_query(&query)?;
        self.client
            .get_with_query_with_options("/v1/voices", &query, &options)
            .await
    }

    /// Stream voice pages until the server returns no next cursor.
    pub fn list_pages(
        &self,
        query: VoiceListQuery,
    ) -> impl Stream<Item = Result<VoiceList, RunwayError>> {
        let client = self.client.clone();
        try_stream! {
            Self::validate_list_query(&query)?;
            let mut query = query;
            loop {
                let page: VoiceList = client
                    .get_with_query_with_options(
                        "/v1/voices",
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

    /// Stream individual voices across every cursor page.
    pub fn list_all(
        &self,
        query: VoiceListQuery,
    ) -> impl Stream<Item = Result<Voice, RunwayError>> {
        let client = self.client.clone();
        try_stream! {
            Self::validate_list_query(&query)?;
            let mut query = query;
            loop {
                let page: VoiceList = client
                    .get_with_query_with_options(
                        "/v1/voices",
                        &query,
                        &RequestOptions::default(),
                    )
                    .await?
                    .data;
                let next_cursor = page.next_cursor.clone();
                for voice in page.data {
                    yield voice;
                }
                let Some(next_cursor) = next_cursor.filter(|cursor| !cursor.is_empty()) else {
                    break;
                };
                query.cursor = Some(next_cursor);
            }
        }
    }

    pub async fn retrieve(&self, id: &str) -> Result<Voice, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<Voice>, RunwayError> {
        let path = RunwayClient::path(&["v1", "voices", id])?;
        self.client.get_with_options(&path, &options).await
    }

    pub async fn create(
        &self,
        request: CreateVoiceRequest,
    ) -> Result<VoiceCreateResponse, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: CreateVoiceRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<VoiceCreateResponse>, RunwayError> {
        request.validate()?;
        self.client
            .post_with_options("/v1/voices", &request, &options)
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
        let path = RunwayClient::path(&["v1", "voices", id])?;
        self.client.delete_with_options(&path, &options).await
    }

    pub async fn preview(
        &self,
        request: PreviewVoiceRequest,
    ) -> Result<PreviewVoiceResponse, RunwayError> {
        Ok(self
            .preview_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn preview_with_options(
        &self,
        request: PreviewVoiceRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PreviewVoiceResponse>, RunwayError> {
        request.validate()?;
        self.client
            .post_with_options("/v1/voices/preview", &request, &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Voice, RunwayError> {
        self.retrieve(id).await
    }

    /// Update the display name and/or description of a custom voice.
    pub async fn update(
        &self,
        id: &str,
        request: UpdateVoiceRequest,
    ) -> Result<Voice, RunwayError> {
        Ok(self
            .update_with_options(id, request, RequestOptions::default())
            .await?
            .data)
    }

    /// Update a custom voice with per-request transport overrides.
    pub async fn update_with_options(
        &self,
        id: &str,
        request: UpdateVoiceRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<Voice>, RunwayError> {
        let path = RunwayClient::path(&["v1", "voices", id])?;
        self.client
            .patch_with_options(&path, &request, &options)
            .await
    }
}
