use reqwest::header::HeaderMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ApiErrorKind {
    /// The request payload or parameters were invalid.
    BadRequest,
    /// Authentication failed.
    Authentication,
    /// The authenticated principal lacks permission to perform the action.
    PermissionDenied,
    /// The requested resource does not exist.
    NotFound,
    /// The request conflicted with current server state.
    Conflict,
    /// The request was syntactically valid but semantically rejected.
    UnprocessableEntity,
    /// The caller is being rate limited.
    RateLimited,
    /// The server failed while processing the request.
    InternalServer,
    /// The response did not map cleanly onto a known Runway error class.
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
    /// Return the HTTP status code when one is available.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::RateLimited { .. } => Some(429),
            Self::Unauthorized => Some(401),
            _ => None,
        }
    }

    /// Return the response headers for API-derived errors.
    pub fn headers(&self) -> Option<&HeaderMap> {
        match self {
            Self::Api { headers, .. } => Some(headers.as_ref()),
            Self::RateLimited { headers, .. } => Some(headers.as_ref()),
            _ => None,
        }
    }

    /// Return the classified Runway API error kind when available.
    pub fn api_kind(&self) -> Option<ApiErrorKind> {
        match self {
            Self::Api { kind, .. } => Some(*kind),
            Self::RateLimited { .. } => Some(ApiErrorKind::RateLimited),
            Self::Unauthorized => Some(ApiErrorKind::Authentication),
            _ => None,
        }
    }

    /// Return the parsed `Retry-After` delay for rate-limit errors.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}
