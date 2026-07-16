use reqwest::header::HeaderMap;
use std::fmt;
use std::ops::Deref;
use std::time::Duration;
use uuid::Uuid;

/// HTTP response headers retained on API errors.
///
/// Header values remain available through [`Deref`], while [`Debug`] redacts
/// every value so diagnostic logging cannot disclose cookies or opaque tokens.
#[derive(Clone, Default)]
pub struct ErrorResponseHeaders(HeaderMap);

impl ErrorResponseHeaders {
    /// Borrow the underlying response header map.
    pub fn as_header_map(&self) -> &HeaderMap {
        &self.0
    }

    /// Consume the wrapper and return the underlying response header map.
    pub fn into_header_map(self) -> HeaderMap {
        self.0
    }
}

impl From<HeaderMap> for ErrorResponseHeaders {
    fn from(headers: HeaderMap) -> Self {
        Self(headers)
    }
}

impl AsRef<HeaderMap> for ErrorResponseHeaders {
    fn as_ref(&self) -> &HeaderMap {
        &self.0
    }
}

impl Deref for ErrorResponseHeaders {
    type Target = HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for ErrorResponseHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for name in self.0.keys() {
            map.entry(&name.as_str(), &"[REDACTED]");
        }
        map.finish()
    }
}

/// A bounded response-body excerpt retained for decode diagnostics.
///
/// Its [`Debug`](std::fmt::Debug) implementation is deliberately redacted so
/// logs cannot accidentally disclose generated content or signed asset URLs.
pub struct ResponseBodyExcerpt(String);

impl ResponseBodyExcerpt {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    /// Borrow the excerpt for an explicit, deliberate diagnostic action.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for ResponseBodyExcerpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED RESPONSE BODY]")
    }
}

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
        headers: Box<ErrorResponseHeaders>,
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
        headers: Box<ErrorResponseHeaders>,
    },

    #[error("Task polling timed out after {elapsed:?}")]
    Timeout { task_id: Uuid, elapsed: Duration },

    #[error("Workflow polling timed out after {elapsed:?}")]
    WorkflowTimeout {
        invocation_id: String,
        elapsed: Duration,
    },

    #[error("Authentication failed: {message}")]
    Unauthorized {
        message: String,
        code: Option<String>,
        headers: Box<ErrorResponseHeaders>,
    },

    #[error("Invalid input: {message}")]
    Validation { message: String },

    #[error("Connection error: {source}")]
    ConnectionError {
        #[source]
        source: reqwest::Error,
    },

    #[error("Connection timed out: {source}")]
    ConnectionTimeout {
        #[source]
        source: reqwest::Error,
    },

    #[error("Request was aborted")]
    RequestAborted,

    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Could not decode API response (HTTP {status}): {message}")]
    ResponseDecode {
        status: u16,
        message: String,
        body_excerpt: ResponseBodyExcerpt,
        #[source]
        source: serde_json::Error,
    },

    #[error("API response body exceeded the {limit_bytes}-byte safety limit (HTTP {status})")]
    ResponseTooLarge {
        status: u16,
        limit_bytes: usize,
        content_length: Option<u64>,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing API key — set RUNWAYML_API_SECRET env var or pass explicitly")]
    MissingApiKey,
}

impl From<reqwest::Error> for RunwayError {
    fn from(error: reqwest::Error) -> Self {
        // Reqwest errors retain their request URL. Strip it before the error can
        // reach Debug/Display because URLs may contain signed storage credentials
        // or caller-supplied secret query parameters.
        Self::Http(error.without_url())
    }
}

impl RunwayError {
    /// Return the HTTP status code when one is available.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::RateLimited { .. } => Some(429),
            Self::Unauthorized { .. } => Some(401),
            Self::ResponseDecode { status, .. } | Self::ResponseTooLarge { status, .. } => {
                Some(*status)
            }
            _ => None,
        }
    }

    /// Return the response headers for API-derived errors.
    pub fn headers(&self) -> Option<&HeaderMap> {
        match self {
            Self::Api { headers, .. } | Self::Unauthorized { headers, .. } => {
                Some(headers.as_header_map())
            }
            Self::RateLimited { headers, .. } => Some(headers.as_header_map()),
            _ => None,
        }
    }

    /// Return the classified Runway API error kind when available.
    pub fn api_kind(&self) -> Option<ApiErrorKind> {
        match self {
            Self::Api { kind, .. } => Some(*kind),
            Self::RateLimited { .. } => Some(ApiErrorKind::RateLimited),
            Self::Unauthorized { .. } => Some(ApiErrorKind::Authentication),
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
