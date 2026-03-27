use std::time::Duration;

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
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub api_version: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub poll_interval: Duration,
    pub max_poll_duration: Duration,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
            timeout: DEFAULT_TIMEOUT,
            max_retries: DEFAULT_MAX_RETRIES,
            poll_interval: DEFAULT_POLL_INTERVAL,
            max_poll_duration: DEFAULT_MAX_POLL_DURATION,
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    pub fn max_poll_duration(mut self, duration: Duration) -> Self {
        self.max_poll_duration = duration;
        self
    }
}
