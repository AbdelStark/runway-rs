use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::workflow::{RunWorkflowRequest, Workflow, WorkflowList, WorkflowRunResponse};

pub struct WorkflowsResource {
    pub(crate) client: RunwayClient,
}

impl WorkflowsResource {
    pub async fn list(&self) -> Result<WorkflowList, RunwayError> {
        self.client.get("/v1/workflows").await
    }

    pub async fn get(&self, id: &str) -> Result<Workflow, RunwayError> {
        self.client.get(&format!("/v1/workflows/{}", id)).await
    }

    pub async fn run(
        &self,
        id: &str,
        request: RunWorkflowRequest,
    ) -> Result<WorkflowRunResponse, RunwayError> {
        self.client
            .post(&format!("/v1/workflows/{}", id), &request)
            .await
    }
}
