use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingWorkflowInvocation;
use crate::types::workflow::{RunWorkflowRequest, Workflow, WorkflowList, WorkflowRunResponse};

pub struct WorkflowsResource {
    pub(crate) client: RunwayClient,
}

impl WorkflowsResource {
    pub async fn list(&self) -> Result<WorkflowList, RunwayError> {
        Ok(self
            .list_with_options(RequestOptions::default())
            .await?
            .data)
    }

    pub async fn list_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<WithResponse<WorkflowList>, RunwayError> {
        self.client
            .get_with_options("/v1/workflows", &options)
            .await
    }

    pub async fn retrieve(&self, id: &str) -> Result<Workflow, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<Workflow>, RunwayError> {
        let path = RunwayClient::path(&["v1", "workflows", id])?;
        self.client.get_with_options(&path, &options).await
    }

    pub async fn get(&self, id: &str) -> Result<Workflow, RunwayError> {
        self.retrieve(id).await
    }

    pub async fn run(
        &self,
        id: &str,
        request: RunWorkflowRequest,
    ) -> Result<WorkflowRunResponse, RunwayError> {
        Ok(self
            .run_with_options(id, request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn run_with_options(
        &self,
        id: &str,
        request: RunWorkflowRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<WorkflowRunResponse>, RunwayError> {
        let path = RunwayClient::path(&["v1", "workflows", id])?;
        self.client
            .post_with_options(&path, &request, &options)
            .await
    }

    pub async fn run_pending(
        &self,
        id: &str,
        request: RunWorkflowRequest,
    ) -> Result<PendingWorkflowInvocation, RunwayError> {
        Ok(self
            .run_pending_with_options(id, request, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn run_pending_with_options(
        &self,
        id: &str,
        request: RunWorkflowRequest,
        options: RequestOptions,
    ) -> Result<WithResponse<PendingWorkflowInvocation>, RunwayError> {
        let continuation_client = self.client.continuation_client(&options)?;
        let response = self.run_with_options(id, request, options).await?;
        Ok(WithResponse {
            data: PendingWorkflowInvocation::new(continuation_client, response.data.id),
            response: response.response,
        })
    }
}
