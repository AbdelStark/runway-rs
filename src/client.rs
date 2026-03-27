use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::error::RunwayError;
use crate::resources::*;

/// Inner client state shared via Arc.
pub struct ClientInner {
    pub(crate) http: reqwest::Client,
    pub config: Config,
}

/// The main entry point for interacting with the Runway API.
#[derive(Clone)]
pub struct RunwayClient {
    pub inner: Arc<ClientInner>,
}

impl RunwayClient {
    /// Create from RUNWAYML_API_SECRET env var.
    pub fn new() -> Result<Self, RunwayError> {
        let api_key =
            std::env::var("RUNWAYML_API_SECRET").map_err(|_| RunwayError::MissingApiKey)?;
        Ok(Self::with_api_key(api_key))
    }

    /// Create with explicit API key.
    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self::with_config(Config::new(key))
    }

    /// Create with full config.
    pub fn with_config(config: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Runway-Version",
            HeaderValue::from_str(&config.api_version).expect("valid api version header"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .expect("valid auth header"),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .expect("failed to build HTTP client");

        Self {
            inner: Arc::new(ClientInner { http, config }),
        }
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

    // ── Internal HTTP helpers ───────────────────────────────────────────

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.inner.config.base_url, path)
    }

    pub(crate) async fn post<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("POST {}", url);

        let mut retries = 0;
        loop {
            let resp = self.inner.http.post(&url).json(body).send().await?;
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
                let wait = Duration::from_secs(2u64.pow(retries));
                tracing::warn!("Rate limited, retrying in {:?}", wait);
                tokio::time::sleep(wait).await;
                retries += 1;
                continue;
            }

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

            let result = resp.json::<Resp>().await?;
            return Ok(result);
        }
    }

    pub(crate) async fn get<Resp: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("GET {}", url);

        let mut retries = 0;
        loop {
            let resp = self.inner.http.get(&url).send().await?;
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
                let wait = Duration::from_secs(2u64.pow(retries));
                tracing::warn!("Rate limited, retrying in {:?}", wait);
                tokio::time::sleep(wait).await;
                retries += 1;
                continue;
            }

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

            let result = resp.json::<Resp>().await?;
            return Ok(result);
        }
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), RunwayError> {
        let url = self.url(path);
        tracing::debug!("DELETE {}", url);

        let resp = self.inner.http.delete(&url).send().await?;
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

        Ok(())
    }

    pub(crate) async fn patch<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, RunwayError> {
        let url = self.url(path);
        tracing::debug!("PATCH {}", url);

        let resp = self.inner.http.patch(&url).json(body).send().await?;
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

        let result = resp.json::<Resp>().await?;
        Ok(result)
    }
}

impl std::fmt::Debug for RunwayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunwayClient")
            .field("base_url", &self.inner.config.base_url)
            .finish()
    }
}
