use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::error::RunwayError;

/// Default Runway API base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.dev.runwayml.com";
/// Default API version header value.
pub const DEFAULT_API_VERSION: &str = "2024-11-06";
/// Default HTTP request timeout (300s / 5 minutes — video generation is slow).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(300);
/// Default maximum retry attempts for rate-limited requests.
pub const DEFAULT_MAX_RETRIES: u32 = 3;
/// Default interval between task status polls (Runway recommends >= 5s).
pub const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(5);
/// Default maximum duration to poll before timing out (600s / 10 minutes).
pub const DEFAULT_MAX_POLL_DURATION: Duration = Duration::from_secs(600);

/// Configuration for the Runway API client.
///
/// Use the builder methods to customize behavior:
///
/// ```
/// use runway_sdk::Config;
/// use std::time::Duration;
///
/// let config = Config::new("my-api-key")
///     .timeout(Duration::from_secs(60))
///     .max_retries(5);
/// ```
#[derive(Clone)]
pub struct Config {
    /// Bearer token used to authenticate with the Runway API.
    pub api_key: String,
    /// Base URL for API requests.
    pub base_url: String,
    /// Value sent in the `X-Runway-Version` header.
    pub api_version: String,
    /// Default timeout applied to each HTTP request.
    pub timeout: Duration,
    /// Maximum number of retry attempts for retryable requests.
    pub max_retries: u32,
    /// Delay between polls when waiting on tasks or workflow invocations.
    pub poll_interval: Duration,
    /// Maximum time to wait when polling for a terminal result.
    pub max_poll_duration: Duration,
    /// Headers merged into every request from this client.
    pub default_headers: HeaderMap,
    /// Query parameters merged into every request from this client.
    pub default_query: Vec<(String, String)>,
}

impl Config {
    /// Create a configuration with Runway defaults and the provided API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
            timeout: DEFAULT_TIMEOUT,
            max_retries: DEFAULT_MAX_RETRIES,
            poll_interval: DEFAULT_POLL_INTERVAL,
            max_poll_duration: DEFAULT_MAX_POLL_DURATION,
            default_headers: HeaderMap::new(),
            default_query: Vec::new(),
        }
    }

    /// Override the API base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Override the `X-Runway-Version` header value.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Override the default request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Override the default retry budget.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Override the default poll interval for pending tasks and workflows.
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Override the default polling deadline for pending tasks and workflows.
    pub fn max_poll_duration(mut self, duration: Duration) -> Self {
        self.max_poll_duration = duration;
        self
    }

    /// Add a header that will be sent with every request from this client.
    pub fn default_header(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self, RunwayError> {
        let name = HeaderName::try_from(name.as_ref()).map_err(|_| RunwayError::Validation {
            message: format!("Invalid header name: {}", name.as_ref()),
        })?;
        let value = HeaderValue::from_str(value.as_ref()).map_err(|_| RunwayError::Validation {
            message: format!("Invalid header value for {}", name.as_str()),
        })?;
        self.default_headers.insert(name, value);
        Ok(self)
    }

    /// Add a query parameter that will be sent with every request from this client.
    pub fn default_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_query.push((key.into(), value.into()));
        self
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("api_version", &self.api_version)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("poll_interval", &self.poll_interval)
            .field("max_poll_duration", &self.max_poll_duration)
            .field("default_headers", &self.default_headers)
            .field("default_query", &self.default_query)
            .finish()
    }
}
