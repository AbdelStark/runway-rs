use uuid::Uuid;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::task::{Task, TaskList, TaskListQuery};

pub struct TasksResource {
    pub(crate) client: RunwayClient,
}

impl TasksResource {
    /// List tasks, optionally filtered by status with pagination.
    pub async fn list(&self, query: TaskListQuery) -> Result<TaskList, RunwayError> {
        let mut params = Vec::new();
        if let Some(ref status) = query.status {
            let status_str = serde_json::to_value(status)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            params.push(format!("status={}", status_str));
        }
        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = query.offset {
            params.push(format!("offset={}", offset));
        }
        let path = if params.is_empty() {
            "/v1/tasks".to_string()
        } else {
            format!("/v1/tasks?{}", params.join("&"))
        };
        self.client.get(&path).await
    }

    pub async fn get(&self, id: Uuid) -> Result<Task, RunwayError> {
        self.client.get(&format!("/v1/tasks/{}", id)).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RunwayError> {
        self.client.delete(&format!("/v1/tasks/{}", id)).await
    }

    /// Cancel a running task. Unlike delete, cancel stops an in-progress task
    /// without removing it from the task list.
    pub async fn cancel(&self, id: Uuid) -> Result<(), RunwayError> {
        self.client
            .post::<serde_json::Value, serde_json::Value>(
                &format!("/v1/tasks/{}/cancel", id),
                &serde_json::json!({}),
            )
            .await?;
        Ok(())
    }
}
