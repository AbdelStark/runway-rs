use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::document::{
    CreateDocumentRequest, Document, DocumentList, UpdateDocumentRequest,
};

pub struct DocumentsResource {
    pub(crate) client: RunwayClient,
}

impl DocumentsResource {
    pub async fn list(&self) -> Result<DocumentList, RunwayError> {
        self.client.get("/v1/documents").await
    }

    pub async fn get(&self, id: &str) -> Result<Document, RunwayError> {
        self.client.get(&format!("/v1/documents/{}", id)).await
    }

    pub async fn create(&self, request: CreateDocumentRequest) -> Result<Document, RunwayError> {
        self.client.post("/v1/documents", &request).await
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateDocumentRequest,
    ) -> Result<Document, RunwayError> {
        self.client
            .patch(&format!("/v1/documents/{}", id), &request)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), RunwayError> {
        self.client.delete(&format!("/v1/documents/{}", id)).await
    }
}
