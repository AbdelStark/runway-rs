use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::document::{
    CreateDocumentRequest, Document, DocumentList, DocumentListQuery, UpdateDocumentRequest,
};

pub struct DocumentsResource {
    pub(crate) client: RunwayClient,
}

impl DocumentsResource {
    pub async fn list(&self, query: DocumentListQuery) -> Result<DocumentList, RunwayError> {
        Ok(self
            .list_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn list_with_options(
        &self,
        query: DocumentListQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<DocumentList>, RunwayError> {
        self.client
            .get_with_query_with_options("/v1/documents", &query, &options)
            .await
    }

    pub async fn retrieve(&self, id: &str) -> Result<Document, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<Document>, RunwayError> {
        self.client
            .get_with_options(&format!("/v1/documents/{}", id), &options)
            .await
    }

    pub async fn create(&self, request: CreateDocumentRequest) -> Result<Document, RunwayError> {
        Ok(self
            .create_with_options(request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn create_with_options(
        &self,
        request: CreateDocumentRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<Document>, RunwayError> {
        self.client
            .post_with_options("/v1/documents", &request, &options)
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateDocumentRequest,
    ) -> Result<(), RunwayError> {
        self.update_with_options(id, request, RequestOptions::default())
            .await?;
        Ok(())
    }

    pub async fn update_with_options(
        &self,
        id: &str,
        request: UpdateDocumentRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        self.client
            .patch_empty_with_options(&format!("/v1/documents/{}", id), &request, &options)
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
            .delete_with_options(&format!("/v1/documents/{}", id), &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Document, RunwayError> {
        self.retrieve(id).await
    }
}
