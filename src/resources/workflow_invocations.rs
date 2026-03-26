use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::workflow::WorkflowInvocation;

pub struct WorkflowInvocationsResource {
    pub(crate) client: RunwayClient,
}

impl WorkflowInvocationsResource {
    pub async fn get(&self, id: &str) -> Result<WorkflowInvocation, RunwayError> {
        self.client
            .get(&format!("/v1/workflow_invocations/{}", id))
            .await
    }
}
