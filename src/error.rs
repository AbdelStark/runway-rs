use reqwest::header::HeaderMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ApiErrorKind {
    BadRequest,
    Authentication,
    PermissionDenied,
    NotFound,
    Conflict,
    UnprocessableEntity,
    RateLimited,
    InternalServer,
    Unknown,
}

/// All errors that can occur when using the Runway SDK.
///
/// Errors are categorized by source: API responses, task or workflow lifecycle,
/// transport/runtime behavior, and local validation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RunwayError {
    #[error("API error ({status}, {kind:?}): {message}")]
    Api {
        status: u16,
        kind: ApiErrorKind,
        message: String,
        code: Option<String>,
        headers: Box<HeaderMap>,
    },

    #[error("Task failed: {message} (code: {code})")]
    TaskFailed {
        task_id: Uuid,
        message: String,
        code: String,
    },

    #[error("Workflow invocation failed: {message} (code: {code})")]
    WorkflowInvocationFailed {
        invocation_id: String,
        message: String,
        code: String,
    },

    #[error("Rate limited: {message}")]
    RateLimited {
        retry_after: Option<Duration>,
        message: String,
        code: Option<String>,
        headers: Box<HeaderMap>,
    },

    #[error("Task polling timed out after {elapsed:?}")]
    Timeout { task_id: Uuid, elapsed: Duration },

    #[error("Workflow polling timed out after {elapsed:?}")]
    WorkflowTimeout {
        invocation_id: String,
        elapsed: Duration,
    },

    #[error("Authentication failed — check RUNWAYML_API_SECRET")]
    Unauthorized,

    #[error("Invalid input: {message}")]
    Validation { message: String },

    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Connection timed out")]
    ConnectionTimeout,

    #[error("Request was aborted")]
    RequestAborted,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing API key — set RUNWAYML_API_SECRET env var or pass explicitly")]
    MissingApiKey,
}

impl RunwayError {
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::RateLimited { .. } => Some(429),
            Self::Unauthorized => Some(401),
            _ => None,
        }
    }

    pub fn headers(&self) -> Option<&HeaderMap> {
        match self {
            Self::Api { headers, .. } => Some(headers.as_ref()),
            Self::RateLimited { headers, .. } => Some(headers.as_ref()),
            _ => None,
        }
    }

    pub fn api_kind(&self) -> Option<ApiErrorKind> {
        match self {
            Self::Api { kind, .. } => Some(*kind),
            Self::RateLimited { .. } => Some(ApiErrorKind::RateLimited),
            Self::Unauthorized => Some(ApiErrorKind::Authentication),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}
