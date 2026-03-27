use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::error::RunwayError;
use crate::resources::*;

/// Inner client state shared via `Arc`.
pub struct ClientInner {
    pub(crate) http: reqwest::Client,
    /// The configuration used to construct this client.
    pub config: Config,
}

/// The main entry point for interacting with the Runway API.
///
/// `RunwayClient` is cheaply cloneable (backed by `Arc`) and safe to share
/// across tasks and threads.
///
/// # Examples
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use runway_sdk::RunwayClient;
///
/// // From RUNWAYML_API_SECRET env var:
/// let client = RunwayClient::new()?;
///
/// // Or with an explicit key:
/// let client = RunwayClient::with_api_key("sk_test_...")?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct RunwayClient {
    /// Access to the inner client state (config, HTTP client).
    pub inner: Arc<ClientInner>,
}

impl RunwayClient {
    /// Create from RUNWAYML_API_SECRET env var.
    pub fn new() -> Result<Self, RunwayError> {
        let api_key =
            std::env::var("RUNWAYML_API_SECRET").map_err(|_| RunwayError::MissingApiKey)?;
        Self::with_api_key(api_key)
    }

    /// Create with explicit API key.
    pub fn with_api_key(key: impl Into<String>) -> Result<Self, RunwayError> {
        Self::with_config(Config::new(key))
    }

    /// Create with full config.
    pub fn with_config(config: Config) -> Result<Self, RunwayError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Runway-Version",
            HeaderValue::from_str(&config.api_version).map_err(|_| RunwayError::Validation {
                message: format!(
                    "API version contains invalid header characters: {:?}",
                    config.api_version
                ),
            })?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key)).map_err(|_| {
                RunwayError::Validation {
                    message: "API key contains invalid header characters".into(),
                }
            })?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .map_err(RunwayError::Http)?;

        Ok(Self {
            inner: Arc::new(ClientInner { http, config }),
        })
    }

    // ── Resource accessors ──────────────────────────────────────────────

    pub fn image_to_video(&self) -> ImageToVideoResource {
        ImageToVideoResource {
            client: self.clone(),
        }
    }

    pub fn text_to_video(&self) -> TextToVideoResource {
        TextToVideoResource {
            client: self.clone(),
        }
    }

    pub fn video_to_video(&self) -> VideoToVideoResource {
        VideoToVideoResource {
            client: self.clone(),
        }
    }

    pub fn text_to_image(&self) -> TextToImageResource {
        TextToImageResource {
            client: self.clone(),
        }
    }

    pub fn character_performance(&self) -> CharacterPerformanceResource {
        CharacterPerformanceResource {
            client: self.clone(),
        }
    }

    pub fn sound_effect(&self) -> SoundEffectResource {
        SoundEffectResource {
            client: self.clone(),
        }
    }

    pub fn speech_to_speech(&self) -> SpeechToSpeechResource {
        SpeechToSpeechResource {
            client: self.clone(),
        }
    }

    pub fn text_to_speech(&self) -> TextToSpeechResource {
        TextToSpeechResource {
            client: self.clone(),
        }
    }

    pub fn voice_dubbing(&self) -> VoiceDubbingResource {
        VoiceDubbingResource {
            client: self.clone(),
        }
    }

    pub fn voice_isolation(&self) -> VoiceIsolationResource {
        VoiceIsolationResource {
            client: self.clone(),
        }
    }

    pub fn tasks(&self) -> TasksResource {
        TasksResource {
            client: self.clone(),
        }
    }

    pub fn uploads(&self) -> UploadsResource {
        UploadsResource {
            client: self.clone(),
        }
    }

    pub fn workflows(&self) -> WorkflowsResource {
        WorkflowsResource {
            client: self.clone(),
        }
    }

    pub fn workflow_invocations(&self) -> WorkflowInvocationsResource {
        WorkflowInvocationsResource {
            client: self.clone(),
        }
    }

    pub fn avatars(&self) -> AvatarsResource {
        AvatarsResource {
            client: self.clone(),
        }
    }

    pub fn documents(&self) -> DocumentsResource {
        DocumentsResource {
            client: self.clone(),
        }
    }

    pub fn realtime_sessions(&self) -> RealtimeSessionsResource {
        RealtimeSessionsResource {
            client: self.clone(),
        }
    }

    pub fn organization(&self) -> OrganizationResource {
        OrganizationResource {
            client: self.clone(),
        }
    }

    pub fn voices(&self) -> VoicesResource {
        VoicesResource {
            client: self.clone(),
        }
    }

    pub fn lip_sync(&self) -> LipSyncResource {
        LipSyncResource {
            client: self.clone(),
        }
    }

    pub fn image_upscale(&self) -> ImageUpscaleResource {
        ImageUpscaleResource {
            client: self.clone(),
        }
    }

    // ── Internal HTTP helpers ───────────────────────────────────────────

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.inner.config.base_url, path)
    }

    /// Check a response for errors. Returns the response if successful.
    async fn check_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<reqwest::Response, RunwayError> {
        let status = resp.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(RunwayError::Unauthorized);
        }

        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(RunwayError::Api {
                status: status.as_u16(),
                message: text,
                code: None,
            });
        }

        Ok(resp)
    }

    /// Send a request with retry logic for rate limiting (HTTP 429) and
    /// transient server errors (HTTP 502, 503, 504).
    ///
    /// Uses exponential backoff with jitter to avoid thundering-herd effects.
    async fn send_with_retry(
        &self,
        request_builder: impl Fn() -> reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, RunwayError> {
        let mut retries = 0;
        loop {
            let resp = request_builder().send().await?;
            let status = resp.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if retries >= self.inner.config.max_retries {
                    let retry_after = resp
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .map(Duration::from_secs);
                    return Err(RunwayError::RateLimited { retry_after });
                }
                let wait = Self::backoff_with_jitter(retries);
                tracing::warn!("Rate limited, retrying in {:?}", wait);
                tokio::time::sleep(wait).await;
                retries += 1;
                continue;
            }

            // Retry on transient server errors (502 Bad Gateway, 503 Service
            // Unavailable, 504 Gateway Timeout).
            if matches!(
                status,
                reqwest::StatusCode::BAD_GATEWAY
                    | reqwest::StatusCode::SERVICE_UNAVAILABLE
                    | reqwest::StatusCode::GATEWAY_TIMEOUT
            ) {
                if retries >= self.inner.config.max_retries {
                    let text = resp.text().await.unwrap_or_default();
                    return Err(RunwayError::Api {
                        status: status.as_u16(),
                        message: text,
                        code: None,
                    });
                }
                let wait = Self::backoff_with_jitter(retries);
                tracing::warn!("Server error {}, retrying in {:?}", status.as_u16(), wait);
                tokio::time::sleep(wait).await;
                retries += 1;
                continue;
            }

            return self.check_response(resp).await;
        }
    }

    /// Calculate exponential backoff with jitter: base * 2^retries + random jitter.
    fn backoff_with_jitter(retries: u32) -> Duration {
        let base_ms = 1000u64 * 2u64.pow(retries);
        // Simple jitter: add 0-500ms using a time-based seed to avoid
        // requiring a random number generator dependency.
        let jitter_ms = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64)
            % 500;
        Duration::from_millis(base_ms + jitter_ms)
    }

    pub(crate) async fn post<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("POST {}", url);
        let body_bytes = serde_json::to_vec(body)?;

        let resp = self
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(&url)
                    .header(CONTENT_TYPE, "application/json")
                    .body(body_bytes.clone())
            })
            .await?;

        Ok(resp.json::<Resp>().await?)
    }

    pub(crate) async fn get<Resp: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("GET {}", url);

        let resp = self.send_with_retry(|| self.inner.http.get(&url)).await?;
        Ok(resp.json::<Resp>().await?)
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), RunwayError> {
        let url = self.url(path);
        tracing::debug!("DELETE {}", url);

        self.send_with_retry(|| self.inner.http.delete(&url))
            .await?;
        Ok(())
    }

    pub(crate) async fn patch<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("PATCH {}", url);
        let body_bytes = serde_json::to_vec(body)?;

        let resp = self
            .send_with_retry(|| {
                self.inner
                    .http
                    .patch(&url)
                    .header(CONTENT_TYPE, "application/json")
                    .body(body_bytes.clone())
            })
            .await?;
        Ok(resp.json::<Resp>().await?)
    }
}

impl std::fmt::Debug for RunwayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunwayClient")
            .field("base_url", &self.inner.config.base_url)
            .finish()
    }
}
