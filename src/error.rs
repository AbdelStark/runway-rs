use std::time::Duration;
use uuid::Uuid;

/// All errors that can occur when using the Runway SDK.
///
/// Errors are categorized by source: API responses, task lifecycle,
/// network/serialization, and local validation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RunwayError {
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<String>,
    },

    #[error("Task failed: {message} (code: {code})")]
    TaskFailed {
        task_id: Uuid,
        message: String,
        code: String,
    },

    #[error("Rate limited — retry after {retry_after:?}")]
    RateLimited { retry_after: Option<Duration> },

    #[error("Task polling timed out after {elapsed:?}")]
    Timeout { task_id: Uuid, elapsed: Duration },

    #[error("Authentication failed — check RUNWAYML_API_SECRET")]
    Unauthorized,

    #[error("Invalid input: {message}")]
    Validation { message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing API key — set RUNWAYML_API_SECRET env var or pass explicitly")]
    MissingApiKey,
}
