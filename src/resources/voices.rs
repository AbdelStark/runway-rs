use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::voice::{
    CreateVoiceRequest, PreviewVoiceRequest, PreviewVoiceResponse, Voice, VoiceCreateResponse,
    VoiceList, VoiceListQuery,
};

pub struct VoicesResource {
    pub(crate) client: RunwayClient,
}

impl VoicesResource {
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
        self.client
            .get_with_query_with_options("/v1/voices", &query, &options)
            .await
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
        self.client
            .get_with_options(&format!("/v1/voices/{}", id), &options)
            .await
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
        self.client
            .delete_with_options(&format!("/v1/voices/{}", id), &options)
            .await
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
        self.client
            .post_with_options("/v1/voices/preview", &request, &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Voice, RunwayError> {
        self.retrieve(id).await
    }
}
