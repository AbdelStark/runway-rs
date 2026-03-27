use std::time::Duration;

use async_stream::try_stream;
use futures_core::Stream;
use uuid::Uuid;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::task::{Task, TaskStatus};

/// A handle to a submitted generation task that has not yet completed.
///
/// Returned by all generation resource `.create()` methods. Use
/// [`wait_for_output`](Self::wait_for_output) to poll until completion,
/// or [`stream_status`](Self::stream_status) to receive incremental updates.
#[derive(Debug)]
pub struct PendingTask {
    client: RunwayClient,
    task_id: Uuid,
}

impl PendingTask {
    #[doc(hidden)]
    pub fn new(client: RunwayClient, task_id: Uuid) -> Self {
        Self { client, task_id }
    }

    /// Get the task ID without waiting.
    pub fn id(&self) -> Uuid {
        self.task_id
    }

    /// Poll until task succeeds or fails. Returns completed Task with output URLs.
    pub async fn wait_for_output(self) -> Result<Task, RunwayError> {
        let poll_interval = self.client.inner.config.poll_interval;
        let max_poll_duration = self.client.inner.config.max_poll_duration;
        self.wait_with_config(poll_interval, max_poll_duration)
            .await
    }

    /// Poll with custom interval and timeout.
    pub async fn wait_with_config(
        self,
        poll_interval: Duration,
        max_duration: Duration,
    ) -> Result<Task, RunwayError> {
        let start = tokio::time::Instant::now();

        // Initial delay before first poll
        tokio::time::sleep(Duration::from_secs(2)).await;

        loop {
            let elapsed = start.elapsed();
            if elapsed >= max_duration {
                return Err(RunwayError::Timeout {
                    task_id: self.task_id,
                    elapsed,
                });
            }

            let task: Task = self
                .client
                .get(&format!("/v1/tasks/{}", self.task_id))
                .await?;

            match task.status {
                TaskStatus::Succeeded => return Ok(task),
                TaskStatus::Failed => {
                    return Err(RunwayError::TaskFailed {
                        task_id: self.task_id,
                        message: task.failure.unwrap_or_else(|| "Unknown error".into()),
                        code: task.failure_code.unwrap_or_else(|| "UNKNOWN".into()),
                    });
                }
                TaskStatus::Pending | TaskStatus::Throttled | TaskStatus::Running => {
                    tracing::debug!(
                        "Task {} status: {:?}, progress: {:?}",
                        self.task_id,
                        task.status,
                        task.progress
                    );
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }

    /// Stream task status updates.
    pub fn stream_status(self) -> impl Stream<Item = Result<Task, RunwayError>> {
        let client = self.client;
        let task_id = self.task_id;
        let poll_interval = client.inner.config.poll_interval;

        try_stream! {
            tokio::time::sleep(Duration::from_secs(2)).await;

            loop {
                let task: Task = client
                    .get(&format!("/v1/tasks/{}", task_id))
                    .await?;

                let is_terminal = matches!(task.status, TaskStatus::Succeeded | TaskStatus::Failed);
                yield task;

                if is_terminal {
                    break;
                }

                tokio::time::sleep(poll_interval).await;
            }
        }
    }
}
