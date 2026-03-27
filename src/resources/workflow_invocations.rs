use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::polling::PendingWorkflowInvocation;
use crate::types::workflow::WorkflowInvocation;

pub struct WorkflowInvocationsResource {
    pub(crate) client: RunwayClient,
}

impl WorkflowInvocationsResource {
    pub async fn retrieve(&self, id: &str) -> Result<WorkflowInvocation, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<WithResponse<WorkflowInvocation>, RunwayError> {
        self.client
            .get_with_options(&format!("/v1/workflow_invocations/{}", id), &options)
            .await
    }

    pub async fn get(&self, id: &str) -> Result<WorkflowInvocation, RunwayError> {
        self.retrieve(id).await
    }

    pub fn pending(&self, id: impl Into<String>) -> PendingWorkflowInvocation {
        PendingWorkflowInvocation::new(self.client.clone(), id.into())
    }
}
