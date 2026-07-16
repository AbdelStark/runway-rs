use std::future::{pending, Future};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use async_stream::try_stream;
use futures_core::Stream;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::client::{RequestOptions, RunwayClient};
use crate::error::RunwayError;
use crate::types::task::{Task, TaskStatus};
use crate::types::workflow::{WorkflowInvocation, WorkflowInvocationStatus};

/// Overrides for task and workflow polling behavior.
#[derive(Debug, Clone, Default)]
pub struct WaitOptions {
    /// Target delay between status polls.
    ///
    /// Each wait, including the initial wait, is jittered by up to 25% to avoid
    /// synchronized polling. Use at least five seconds against the live API.
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

fn validate_wait_options(poll_interval: Duration, timeout: Duration) -> Result<(), RunwayError> {
    if poll_interval.is_zero() {
        return Err(RunwayError::Validation {
            message: "Poll interval must be greater than zero".into(),
        });
    }
    if timeout.is_zero() {
        return Err(RunwayError::Validation {
            message: "Polling timeout must be greater than zero".into(),
        });
    }
    Ok(())
}

static JITTER_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn jittered_poll_interval(poll_interval: Duration) -> Duration {
    let interval_nanos = poll_interval.as_nanos();
    let max_jitter = interval_nanos / 4;
    if max_jitter == 0 {
        return poll_interval;
    }

    let sequence = JITTER_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let clock = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let entropy = clock ^ sequence.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let jitter_span = max_jitter * 2 + 1;
    let jittered_nanos = interval_nanos - max_jitter + u128::from(entropy) % jitter_span;
    let jittered_nanos = jittered_nanos.min(Duration::MAX.as_nanos());

    Duration::new(
        (jittered_nanos / 1_000_000_000) as u64,
        (jittered_nanos % 1_000_000_000) as u32,
    )
}

struct PollDeadline {
    started_at: tokio::time::Instant,
    deadline: tokio::time::Instant,
    cancellation_token: Option<CancellationToken>,
}

impl PollDeadline {
    fn new(
        timeout: Duration,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Self, RunwayError> {
        let started_at = tokio::time::Instant::now();
        let deadline = started_at
            .checked_add(timeout)
            .ok_or_else(|| RunwayError::Validation {
                message: "Polling timeout is too large for the runtime clock".into(),
            })?;
        Ok(Self {
            started_at,
            deadline,
            cancellation_token,
        })
    }

    fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    fn remaining<E>(&self, timeout_error: &E) -> Result<Duration, RunwayError>
    where
        E: Fn(Duration) -> RunwayError,
    {
        if self
            .cancellation_token
            .as_ref()
            .is_some_and(CancellationToken::is_cancelled)
        {
            return Err(RunwayError::RequestAborted);
        }

        let remaining = self
            .deadline
            .checked_duration_since(tokio::time::Instant::now())
            .filter(|remaining| !remaining.is_zero())
            .ok_or_else(|| timeout_error(self.elapsed()))?;
        Ok(remaining)
    }

    fn request_options<E>(&self, timeout_error: &E) -> Result<RequestOptions, RunwayError>
    where
        E: Fn(Duration) -> RunwayError,
    {
        let mut options = RequestOptions::new().timeout(self.remaining(timeout_error)?);
        if let Some(cancellation_token) = &self.cancellation_token {
            options = options.cancellation_token(cancellation_token.clone());
        }
        Ok(options)
    }

    async fn run<T, F, E>(&self, future: F, timeout_error: &E) -> Result<T, RunwayError>
    where
        F: Future<Output = Result<T, RunwayError>>,
        E: Fn(Duration) -> RunwayError,
    {
        self.remaining(timeout_error)?;

        tokio::select! {
            biased;
            _ = wait_for_cancellation(self.cancellation_token.as_ref()) => {
                Err(RunwayError::RequestAborted)
            }
            _ = tokio::time::sleep_until(self.deadline) => {
                Err(timeout_error(self.elapsed()))
            }
            result = future => result,
        }
    }

    async fn sleep<E>(&self, duration: Duration, timeout_error: &E) -> Result<(), RunwayError>
    where
        E: Fn(Duration) -> RunwayError,
    {
        self.run(
            async move {
                tokio::time::sleep(duration).await;
                Ok(())
            },
            timeout_error,
        )
        .await
    }
}

async fn wait_for_cancellation(cancellation_token: Option<&CancellationToken>) {
    if let Some(cancellation_token) = cancellation_token {
        cancellation_token.cancelled().await;
    } else {
        pending::<()>().await;
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
        validate_wait_options(poll_interval, max_duration)?;
        let poll_deadline = PollDeadline::new(max_duration, options.cancellation_token)?;
        let timeout_error = |elapsed| RunwayError::Timeout {
            task_id: self.task_id,
            elapsed,
        };

        poll_deadline
            .sleep(jittered_poll_interval(poll_interval), &timeout_error)
            .await?;

        loop {
            let request_options = poll_deadline.request_options(&timeout_error)?;
            let task: Task = poll_deadline
                .run(
                    async {
                        Ok(self
                            .client
                            .get_with_options(
                                &format!("/v1/tasks/{}", self.task_id),
                                &request_options,
                            )
                            .await?
                            .data)
                    },
                    &timeout_error,
                )
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
                    poll_deadline
                        .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                        .await?;
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
        let max_duration = options
            .timeout
            .unwrap_or(client.inner.config.max_poll_duration);
        let cancellation_token = options.cancellation_token;

        try_stream! {
            validate_wait_options(poll_interval, max_duration)?;
            let poll_deadline = PollDeadline::new(max_duration, cancellation_token)?;
            let timeout_error = |elapsed| RunwayError::Timeout {
                task_id,
                elapsed,
            };

            poll_deadline
                .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                .await?;

            loop {
                let request_options = poll_deadline.request_options(&timeout_error)?;
                let task: Task = poll_deadline
                    .run(
                        async {
                            Ok(client
                                .get_with_options(
                                    &format!("/v1/tasks/{}", task_id),
                                    &request_options,
                                )
                                .await?
                                .data)
                        },
                        &timeout_error,
                    )
                    .await?;

                let is_terminal = task.is_terminal();
                yield task;

                if is_terminal {
                    break;
                }

                poll_deadline
                    .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                    .await?;
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
        validate_wait_options(poll_interval, max_duration)?;
        let invocation_path =
            RunwayClient::path(&["v1", "workflow_invocations", &self.invocation_id])?;
        let poll_deadline = PollDeadline::new(max_duration, options.cancellation_token)?;
        let timeout_invocation_id = self.invocation_id.clone();
        let timeout_error = move |elapsed| RunwayError::WorkflowTimeout {
            invocation_id: timeout_invocation_id.clone(),
            elapsed,
        };

        poll_deadline
            .sleep(jittered_poll_interval(poll_interval), &timeout_error)
            .await?;

        loop {
            let request_options = poll_deadline.request_options(&timeout_error)?;
            let invocation: WorkflowInvocation = poll_deadline
                .run(
                    async {
                        Ok(self
                            .client
                            .get_with_options(&invocation_path, &request_options)
                            .await?
                            .data)
                    },
                    &timeout_error,
                )
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
                    poll_deadline
                        .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                        .await?;
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
        let max_duration = options
            .timeout
            .unwrap_or(client.inner.config.max_poll_duration);
        let cancellation_token = options.cancellation_token;

        try_stream! {
            validate_wait_options(poll_interval, max_duration)?;
            let invocation_path = RunwayClient::path(&[
                "v1",
                "workflow_invocations",
                &invocation_id,
            ])?;
            let poll_deadline = PollDeadline::new(max_duration, cancellation_token)?;
            let timeout_invocation_id = invocation_id.clone();
            let timeout_error = move |elapsed| RunwayError::WorkflowTimeout {
                invocation_id: timeout_invocation_id.clone(),
                elapsed,
            };

            poll_deadline
                .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                .await?;

            loop {
                let request_options = poll_deadline.request_options(&timeout_error)?;
                let invocation: WorkflowInvocation = poll_deadline
                    .run(
                        async {
                            Ok(client
                                .get_with_options(&invocation_path, &request_options)
                                .await?
                                .data)
                        },
                        &timeout_error,
                    )
                    .await?;

                let is_terminal = invocation.is_terminal();
                yield invocation;

                if is_terminal {
                    break;
                }

                poll_deadline
                    .sleep(jittered_poll_interval(poll_interval), &timeout_error)
                    .await?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_jitter_stays_within_twenty_five_percent() {
        let interval = Duration::from_secs(6);
        let minimum = Duration::from_millis(4_500);
        let maximum = Duration::from_millis(7_500);

        for _ in 0..100 {
            let jittered = jittered_poll_interval(interval);
            assert!(jittered >= minimum, "{jittered:?} was below {minimum:?}");
            assert!(jittered <= maximum, "{jittered:?} was above {maximum:?}");
        }
    }

    #[test]
    fn polling_deadline_rejects_clock_overflow() {
        assert!(matches!(
            PollDeadline::new(Duration::MAX, None),
            Err(RunwayError::Validation { .. })
        ));
    }
}
