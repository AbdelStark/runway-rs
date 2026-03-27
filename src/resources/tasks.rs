use uuid::Uuid;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::task::Task;

pub struct TasksResource {
    pub(crate) client: RunwayClient,
}

impl TasksResource {
    pub async fn get(&self, id: Uuid) -> Result<Task, RunwayError> {
        self.client.get(&format!("/v1/tasks/{}", id)).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RunwayError> {
        self.client.delete(&format!("/v1/tasks/{}", id)).await
    }
}
