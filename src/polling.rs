use std::time::Duration;

use async_stream::try_stream;
use futures_core::Stream;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::client::RunwayClient;
use crate::error::RunwayError;
use crate::types::task::{Task, TaskStatus};
use crate::types::workflow::{WorkflowInvocation, WorkflowInvocationStatus};

/// Overrides for task and workflow polling behavior.
#[derive(Debug, Clone, Default)]
pub struct WaitOptions {
    /// Delay between status polls.
    pub poll_interval: Option<Duration>,
    /// Maximum time to wait before returning a timeout error.
    pub timeout: Option<Duration>,
    /// Cancellation token that aborts polling when triggered.
    pub cancellation_token: Option<CancellationToken>,
}

impl WaitOptions {
    /// Create an empty set of polling overrides.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the poll interval.
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = Some(poll_interval);
        self
    }

    /// Override the maximum wait duration.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Abort polling when this token is cancelled.
    pub fn cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.cancellation_token = Some(cancellation_token);
        self
    }
}

async fn sleep_or_cancel(
    duration: Duration,
    cancellation_token: Option<&CancellationToken>,
) -> Result<(), RunwayError> {
    if let Some(cancellation_token) = cancellation_token {
        tokio::select! {
            _ = tokio::time::sleep(duration) => Ok(()),
            _ = cancellation_token.cancelled() => Err(RunwayError::RequestAborted),
        }
    } else {
        tokio::time::sleep(duration).await;
        Ok(())
    }
}

/// A handle to a submitted generation task that has not yet completed.
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

    /// Return the Runway task identifier.
    pub fn id(&self) -> Uuid {
        self.task_id
    }

    /// Poll until the task reaches a terminal state using client defaults.
    pub async fn wait_for_output(self) -> Result<Task, RunwayError> {
        self.wait_with_options(WaitOptions::default()).await
    }

    /// Poll with explicit interval and timeout overrides.
    pub async fn wait_with_config(
        self,
        poll_interval: Duration,
        max_duration: Duration,
    ) -> Result<Task, RunwayError> {
        self.wait_with_options(
            WaitOptions::new()
                .poll_interval(poll_interval)
                .timeout(max_duration),
        )
        .await
    }

    /// Poll with fine-grained [`WaitOptions`] overrides.
    pub async fn wait_with_options(self, options: WaitOptions) -> Result<Task, RunwayError> {
        let poll_interval = options
            .poll_interval
            .unwrap_or(self.client.inner.config.poll_interval);
        let max_duration = options
            .timeout
            .unwrap_or(self.client.inner.config.max_poll_duration);
        let cancellation_token = options.cancellation_token;
        let start = tokio::time::Instant::now();

        sleep_or_cancel(Duration::from_secs(2), cancellation_token.as_ref()).await?;

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

            match task.status() {
                TaskStatus::Succeeded => return Ok(task),
                TaskStatus::Failed | TaskStatus::Cancelled => {
                    return Err(RunwayError::TaskFailed {
                        task_id: self.task_id,
                        message: task.failure().unwrap_or("Task cancelled").to_string(),
                        code: task.failure_code().unwrap_or("CANCELLED").to_string(),
                    });
                }
                TaskStatus::Pending | TaskStatus::Throttled | TaskStatus::Running => {
                    tracing::debug!(
                        "Task {} status: {:?}, progress: {:?}",
                        self.task_id,
                        task.status(),
                        task.progress()
                    );
                    sleep_or_cancel(poll_interval, cancellation_token.as_ref()).await?;
                }
            }
        }
    }

    /// Stream task snapshots until the task reaches a terminal state.
    pub fn stream_status(self) -> impl Stream<Item = Result<Task, RunwayError>> {
        self.stream_status_with_options(WaitOptions::default())
    }

    /// Stream task snapshots with custom polling options.
    pub fn stream_status_with_options(
        self,
        options: WaitOptions,
    ) -> impl Stream<Item = Result<Task, RunwayError>> {
        let client = self.client;
        let task_id = self.task_id;
        let poll_interval = options
            .poll_interval
            .unwrap_or(client.inner.config.poll_interval);
        let cancellation_token = options.cancellation_token;

        try_stream! {
            sleep_or_cancel(Duration::from_secs(2), cancellation_token.as_ref()).await?;

            loop {
                let task: Task = client
                    .get(&format!("/v1/tasks/{}", task_id))
                    .await?;

                let is_terminal = task.is_terminal();
                yield task;

                if is_terminal {
                    break;
                }

                sleep_or_cancel(poll_interval, cancellation_token.as_ref()).await?;
            }
        }
    }
}

/// A handle to a submitted workflow invocation that has not yet completed.
#[derive(Debug)]
pub struct PendingWorkflowInvocation {
    client: RunwayClient,
    invocation_id: String,
}

impl PendingWorkflowInvocation {
    #[doc(hidden)]
    pub fn new(client: RunwayClient, invocation_id: impl Into<String>) -> Self {
        Self {
            client,
            invocation_id: invocation_id.into(),
        }
    }

    /// Return the workflow invocation identifier.
    pub fn id(&self) -> &str {
        &self.invocation_id
    }

    /// Poll until the workflow invocation reaches a terminal state using client defaults.
    pub async fn wait_for_output(self) -> Result<WorkflowInvocation, RunwayError> {
        self.wait_with_options(WaitOptions::default()).await
    }

    /// Poll with explicit interval and timeout overrides.
    pub async fn wait_with_config(
        self,
        poll_interval: Duration,
        max_duration: Duration,
    ) -> Result<WorkflowInvocation, RunwayError> {
        self.wait_with_options(
            WaitOptions::new()
                .poll_interval(poll_interval)
                .timeout(max_duration),
        )
        .await
    }

    /// Poll with fine-grained [`WaitOptions`] overrides.
    pub async fn wait_with_options(
        self,
        options: WaitOptions,
    ) -> Result<WorkflowInvocation, RunwayError> {
        let poll_interval = options
            .poll_interval
            .unwrap_or(self.client.inner.config.poll_interval);
        let max_duration = options
            .timeout
            .unwrap_or(self.client.inner.config.max_poll_duration);
        let cancellation_token = options.cancellation_token;
        let start = tokio::time::Instant::now();

        sleep_or_cancel(Duration::from_secs(2), cancellation_token.as_ref()).await?;

        loop {
            let elapsed = start.elapsed();
            if elapsed >= max_duration {
                return Err(RunwayError::WorkflowTimeout {
                    invocation_id: self.invocation_id.clone(),
                    elapsed,
                });
            }

            let invocation: WorkflowInvocation = self
                .client
                .get(&format!("/v1/workflow_invocations/{}", self.invocation_id))
                .await?;

            match invocation.status() {
                WorkflowInvocationStatus::Succeeded => return Ok(invocation),
                WorkflowInvocationStatus::Failed | WorkflowInvocationStatus::Cancelled => {
                    return Err(RunwayError::WorkflowInvocationFailed {
                        invocation_id: self.invocation_id.clone(),
                        message: invocation
                            .failure()
                            .unwrap_or("Workflow invocation cancelled")
                            .to_string(),
                        code: invocation.failure_code().unwrap_or("CANCELLED").to_string(),
                    });
                }
                WorkflowInvocationStatus::Pending
                | WorkflowInvocationStatus::Throttled
                | WorkflowInvocationStatus::Running => {
                    tracing::debug!(
                        "Workflow invocation {} status: {:?}, progress: {:?}",
                        self.invocation_id,
                        invocation.status(),
                        invocation.progress()
                    );
                    sleep_or_cancel(poll_interval, cancellation_token.as_ref()).await?;
                }
            }
        }
    }

    /// Stream workflow invocation snapshots until the invocation is terminal.
    pub fn stream_status(self) -> impl Stream<Item = Result<WorkflowInvocation, RunwayError>> {
        self.stream_status_with_options(WaitOptions::default())
    }

    /// Stream workflow invocation snapshots with custom polling options.
    pub fn stream_status_with_options(
        self,
        options: WaitOptions,
    ) -> impl Stream<Item = Result<WorkflowInvocation, RunwayError>> {
        let client = self.client;
        let invocation_id = self.invocation_id;
        let poll_interval = options
            .poll_interval
            .unwrap_or(client.inner.config.poll_interval);
        let cancellation_token = options.cancellation_token;

        try_stream! {
            sleep_or_cancel(Duration::from_secs(2), cancellation_token.as_ref()).await?;

            loop {
                let invocation: WorkflowInvocation = client
                    .get(&format!("/v1/workflow_invocations/{}", invocation_id))
                    .await?;

                let is_terminal = invocation.is_terminal();
                yield invocation;

                if is_terminal {
                    break;
                }

                sleep_or_cancel(poll_interval, cancellation_token.as_ref()).await?;
            }
        }
    }
}
