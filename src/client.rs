use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::error::{ApiErrorKind, ResponseBodyExcerpt, RunwayError};
use crate::resources::*;

const MAX_JSON_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const MAX_ERROR_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_DECODE_EXCERPT_BYTES: usize = 2 * 1024;

/// Response metadata exposed alongside parsed bodies.
#[derive(Clone)]
pub struct ResponseMetadata {
    /// HTTP status code returned by the Runway API.
    pub status: u16,
    /// Response headers returned by the Runway API.
    pub headers: HeaderMap,
}

impl fmt::Debug for ResponseMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseMetadata")
            .field("status", &self.status)
            .field("headers", &RedactedHeaders(&self.headers))
            .finish()
    }
}

/// Parsed response data paired with transport metadata.
#[derive(Debug, Clone)]
pub struct WithResponse<T> {
    /// Parsed response body.
    pub data: T,
    /// Raw transport metadata for the request.
    pub response: ResponseMetadata,
}

/// Per-request overrides for headers, query params, timeout, retries, and base URL.
#[derive(Clone, Default)]
pub struct RequestOptions {
    /// Extra headers merged into the request.
    pub headers: HeaderMap,
    /// Query string pairs appended to the request URL.
    pub query: Vec<(String, String)>,
    /// Request-specific timeout override.
    pub timeout: Option<Duration>,
    /// Request-specific retry budget override.
    pub max_retries: Option<u32>,
    /// Whether this request may be retried without an idempotency key.
    pub retry_non_idempotent: Option<bool>,
    /// Idempotency key sent as `Idempotency-Key` when present.
    pub idempotency_key: Option<String>,
    /// Base URL override for this request.
    pub base_url: Option<String>,
    /// Cancellation token that aborts this request and any retry backoff.
    pub cancellation_token: Option<CancellationToken>,
}

impl RequestOptions {
    /// Create an empty set of request overrides.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header to this request.
    pub fn header(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self, RunwayError> {
        let name = HeaderName::try_from(name.as_ref()).map_err(|_| RunwayError::Validation {
            message: format!("Invalid header name: {}", name.as_ref()),
        })?;
        if name == "idempotency-key" {
            return Err(RunwayError::Validation {
                message: "Use RequestOptions::idempotency_key for Idempotency-Key".into(),
            });
        }
        let value = HeaderValue::from_str(value.as_ref()).map_err(|_| RunwayError::Validation {
            message: format!("Invalid header value for {}", name.as_str()),
        })?;
        self.headers.insert(name, value);
        Ok(self)
    }

    /// Append a query parameter to this request.
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    /// Override the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Override the retry budget for this request.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Allow or forbid retries when this mutation has no idempotency key.
    ///
    /// Opting in can duplicate billable work after an ambiguous timeout. Prefer
    /// [`idempotency_key`](Self::idempotency_key) whenever the endpoint supports it.
    pub fn retry_non_idempotent(mut self, enabled: bool) -> Self {
        self.retry_non_idempotent = Some(enabled);
        self
    }

    /// Attach an idempotency key to this request.
    pub fn idempotency_key(mut self, idempotency_key: impl Into<String>) -> Self {
        self.idempotency_key = Some(idempotency_key.into());
        self
    }

    /// Override the API base URL for this request.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Abort this request and its retry backoff when the token is cancelled.
    pub fn cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.cancellation_token = Some(cancellation_token);
        self
    }
}

impl fmt::Debug for RequestOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestOptions")
            .field("headers", &RedactedHeaders(&self.headers))
            .field("query", &RedactedQuery(&self.query))
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("retry_non_idempotent", &self.retry_non_idempotent)
            .field(
                "idempotency_key",
                &self.idempotency_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field("base_url", &self.base_url.as_deref().map(RedactedUrl))
            .field(
                "cancellation_token",
                &self
                    .cancellation_token
                    .as_ref()
                    .map(|_| "CancellationToken"),
            )
            .finish()
    }
}

pub(crate) struct RedactedHeaders<'a>(pub(crate) &'a HeaderMap);

impl fmt::Debug for RedactedHeaders<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for name in self.0.keys() {
            map.entry(&name.as_str(), &"[REDACTED]");
        }
        map.finish()
    }
}

pub(crate) struct RedactedQuery<'a>(pub(crate) &'a [(String, String)]);

impl fmt::Debug for RedactedQuery<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for (key, _) in self.0 {
            list.entry(&(key, "[REDACTED]"));
        }
        list.finish()
    }
}

pub(crate) struct RedactedUrl<'a>(pub(crate) &'a str);

impl fmt::Debug for RedactedUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match reqwest::Url::parse(self.0) {
            Ok(mut url) => {
                let _ = url.set_username("");
                let _ = url.set_password(None);
                url.set_query(None);
                url.set_fragment(None);
                f.debug_tuple("Url").field(&url.as_str()).finish()
            }
            Err(_) => f.write_str("[INVALID URL REDACTED]"),
        }
    }
}

#[derive(Clone)]
struct ResolvedRequestOptions {
    headers: HeaderMap,
    query: Vec<(String, String)>,
    timeout: Option<Duration>,
    max_retries: u32,
    retry_non_idempotent: bool,
    idempotency_key: Option<HeaderValue>,
    base_url: Option<String>,
    cancellation_token: Option<CancellationToken>,
}

/// Inner client state shared via `Arc`.
pub(crate) struct ClientInner {
    pub(crate) http: reqwest::Client,
    pub(crate) storage_http: reqwest::Client,
    pub config: Config,
}

/// The main entry point for interacting with the Runway API.
#[derive(Clone)]
pub struct RunwayClient {
    pub(crate) inner: Arc<ClientInner>,
}

impl RunwayClient {
    /// Create a client from the `RUNWAYML_API_SECRET` environment variable.
    pub fn new() -> Result<Self, RunwayError> {
        let api_key =
            std::env::var("RUNWAYML_API_SECRET").map_err(|_| RunwayError::MissingApiKey)?;
        Self::with_api_key(api_key)
    }

    /// Create a client with an explicit API key.
    pub fn with_api_key(key: impl Into<String>) -> Result<Self, RunwayError> {
        Self::with_config(Config::new(key))
    }

    /// Create a client from a fully customized [`Config`].
    pub fn with_config(config: Config) -> Result<Self, RunwayError> {
        if config.api_key().trim().is_empty() {
            return Err(RunwayError::MissingApiKey);
        }
        Self::validate_base_url(&config.base_url)?;
        if config.api_version.trim().is_empty() {
            return Err(RunwayError::Validation {
                message: "API version cannot be empty".into(),
            });
        }
        if config.timeout.is_zero() {
            return Err(RunwayError::Validation {
                message: "Request timeout must be greater than zero".into(),
            });
        }
        if config.poll_interval.is_zero() {
            return Err(RunwayError::Validation {
                message: "Poll interval must be greater than zero".into(),
            });
        }
        if config.max_poll_duration.is_zero() {
            return Err(RunwayError::Validation {
                message: "Maximum poll duration must be greater than zero".into(),
            });
        }
        if config.default_headers.contains_key("idempotency-key") {
            return Err(RunwayError::Validation {
                message: "Idempotency-Key cannot be a client default; set a unique key on each request with RequestOptions::idempotency_key".into(),
            });
        }

        let mut headers = HeaderMap::new();
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
            HeaderValue::from_str(&format!("Bearer {}", config.api_key())).map_err(|_| {
                RunwayError::Validation {
                    message: "API key contains invalid header characters".into(),
                }
            })?,
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(concat!("runway-sdk-rust/", env!("CARGO_PKG_VERSION"))),
        );
        headers.extend(config.default_headers.clone());

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .map_err(RunwayError::from)?;
        // Storage uploads use presigned URLs and must never inherit the Runway
        // authorization or custom API headers.
        let storage_http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(RunwayError::from)?;

        Ok(Self {
            inner: Arc::new(ClientInner {
                http,
                storage_http,
                config,
            }),
        })
    }

    /// Clone this client with per-request defaults applied to the derived instance.
    ///
    /// This is useful when a subset of requests should share alternate headers,
    /// query parameters, retry limits, or a different base URL.
    /// Idempotency keys and cancellation tokens are rejected because both are
    /// scoped to one logical request rather than a reusable client.
    pub fn with_options(&self, options: RequestOptions) -> Result<Self, RunwayError> {
        if options.idempotency_key.is_some() {
            return Err(RunwayError::Validation {
                message: "Idempotency keys are per logical request and cannot be client defaults"
                    .into(),
            });
        }
        if options.cancellation_token.is_some() {
            return Err(RunwayError::Validation {
                message: "Cancellation tokens are per request and cannot be client defaults".into(),
            });
        }
        let mut config = self.inner.config.clone();

        if let Some(base_url) = options.base_url {
            config.base_url = base_url;
        }
        if let Some(timeout) = options.timeout {
            config.timeout = timeout;
        }
        if let Some(max_retries) = options.max_retries {
            config.max_retries = max_retries;
        }
        if let Some(retry_non_idempotent) = options.retry_non_idempotent {
            config.retry_non_idempotent = retry_non_idempotent;
        }

        config.default_headers.extend(options.headers);
        config.default_query.extend(options.query);

        Self::with_config(config)
    }

    /// Derive the client context that a pending task or workflow must use.
    ///
    /// Routing, headers, query parameters, timeouts, and retry budgets carry
    /// forward. A one-shot idempotency key and cancellation token deliberately
    /// do not: polling is a separate read-only operation with its own deadline
    /// and cancellation controls.
    pub(crate) fn continuation_client(
        &self,
        options: &RequestOptions,
    ) -> Result<Self, RunwayError> {
        let continuation = RequestOptions {
            headers: options.headers.clone(),
            query: options.query.clone(),
            timeout: options.timeout,
            max_retries: options.max_retries,
            retry_non_idempotent: None,
            idempotency_key: None,
            base_url: options.base_url.clone(),
            cancellation_token: None,
        };
        if continuation.headers.is_empty()
            && continuation.query.is_empty()
            && continuation.timeout.is_none()
            && continuation.max_retries.is_none()
            && continuation.base_url.is_none()
        {
            Ok(self.clone())
        } else {
            self.with_options(continuation)
        }
    }

    /// Return the effective non-secret client configuration.
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    // ── Resource accessors ──────────────────────────────────────────────

    /// Access image-to-video generation operations.
    pub fn image_to_video(&self) -> ImageToVideoResource {
        ImageToVideoResource {
            client: self.clone(),
        }
    }

    /// Access text-to-video generation operations.
    pub fn text_to_video(&self) -> TextToVideoResource {
        TextToVideoResource {
            client: self.clone(),
        }
    }

    /// Access video-to-video transformation operations.
    pub fn video_to_video(&self) -> VideoToVideoResource {
        VideoToVideoResource {
            client: self.clone(),
        }
    }

    /// Access text-to-image generation operations.
    pub fn text_to_image(&self) -> TextToImageResource {
        TextToImageResource {
            client: self.clone(),
        }
    }

    /// Access character-performance generation operations.
    pub fn character_performance(&self) -> CharacterPerformanceResource {
        CharacterPerformanceResource {
            client: self.clone(),
        }
    }

    /// Access sound-effect generation operations.
    pub fn sound_effect(&self) -> SoundEffectResource {
        SoundEffectResource {
            client: self.clone(),
        }
    }

    /// Access speech-to-speech conversion operations.
    pub fn speech_to_speech(&self) -> SpeechToSpeechResource {
        SpeechToSpeechResource {
            client: self.clone(),
        }
    }

    /// Access text-to-speech generation operations.
    pub fn text_to_speech(&self) -> TextToSpeechResource {
        TextToSpeechResource {
            client: self.clone(),
        }
    }

    /// Access voice-dubbing operations.
    pub fn voice_dubbing(&self) -> VoiceDubbingResource {
        VoiceDubbingResource {
            client: self.clone(),
        }
    }

    /// Access voice-isolation operations.
    pub fn voice_isolation(&self) -> VoiceIsolationResource {
        VoiceIsolationResource {
            client: self.clone(),
        }
    }

    /// Access task retrieval and deletion operations.
    pub fn tasks(&self) -> TasksResource {
        TasksResource {
            client: self.clone(),
        }
    }

    /// Access ephemeral upload operations and file helpers.
    pub fn uploads(&self) -> UploadsResource {
        UploadsResource {
            client: self.clone(),
        }
    }

    /// Access workflow discovery and invocation operations.
    pub fn workflows(&self) -> WorkflowsResource {
        WorkflowsResource {
            client: self.clone(),
        }
    }

    /// Access workflow-invocation retrieval and polling operations.
    pub fn workflow_invocations(&self) -> WorkflowInvocationsResource {
        WorkflowInvocationsResource {
            client: self.clone(),
        }
    }

    /// Access avatar management and usage operations.
    pub fn avatars(&self) -> AvatarsResource {
        AvatarsResource {
            client: self.clone(),
        }
    }

    /// Access real-time avatar conversation operations.
    pub fn avatar_conversations(&self) -> AvatarConversationsResource {
        AvatarConversationsResource {
            client: self.clone(),
        }
    }

    /// Access asynchronous avatar video generation operations.
    pub fn avatar_videos(&self) -> AvatarVideosResource {
        AvatarVideosResource {
            client: self.clone(),
        }
    }

    /// Access document management operations.
    pub fn documents(&self) -> DocumentsResource {
        DocumentsResource {
            client: self.clone(),
        }
    }

    /// Access realtime-session operations.
    pub fn realtime_sessions(&self) -> RealtimeSessionsResource {
        RealtimeSessionsResource {
            client: self.clone(),
        }
    }

    /// Access organization balance, tier, and usage operations.
    pub fn organization(&self) -> OrganizationResource {
        OrganizationResource {
            client: self.clone(),
        }
    }

    /// Access official generation recipe operations.
    pub fn recipes(&self) -> RecipesResource {
        RecipesResource {
            client: self.clone(),
        }
    }

    /// Access custom voice management operations.
    pub fn voices(&self) -> VoicesResource {
        VoicesResource {
            client: self.clone(),
        }
    }

    #[cfg(feature = "unstable-endpoints")]
    /// Access the experimental lip-sync endpoint.
    pub fn lip_sync(&self) -> LipSyncResource {
        LipSyncResource {
            client: self.clone(),
        }
    }

    /// Access image upscaling.
    pub fn image_upscale(&self) -> ImageUpscaleResource {
        ImageUpscaleResource {
            client: self.clone(),
        }
    }

    /// Access video upscaling.
    pub fn video_upscale(&self) -> VideoUpscaleResource {
        VideoUpscaleResource {
            client: self.clone(),
        }
    }

    // ── Internal HTTP helpers ───────────────────────────────────────────

    fn validate_base_url(base_url: &str) -> Result<(), RunwayError> {
        let parsed = reqwest::Url::parse(base_url).map_err(|error| RunwayError::Validation {
            message: format!("Invalid base URL: {error}"),
        })?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err(RunwayError::Validation {
                message: "Base URL must be an absolute http or https URL".into(),
            });
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(RunwayError::Validation {
                message: "Base URL cannot contain credentials".into(),
            });
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            return Err(RunwayError::Validation {
                message: "Base URL cannot contain a query string or fragment".into(),
            });
        }
        Ok(())
    }

    /// Build an API path from literal path segments, percent-encoding identifiers.
    pub(crate) fn path(segments: &[&str]) -> Result<String, RunwayError> {
        let mut url = reqwest::Url::parse("http://runway.invalid/").map_err(|error| {
            RunwayError::Validation {
                message: format!("Could not initialize request path: {error}"),
            }
        })?;
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| RunwayError::Validation {
                    message: "Could not construct request path".into(),
                })?;
            path.clear();
            path.extend(segments.iter().copied());
        }
        Ok(url.path().to_owned())
    }

    fn url_with_base(
        &self,
        path: &str,
        base_url_override: Option<&str>,
    ) -> Result<reqwest::Url, RunwayError> {
        let base_url = base_url_override.unwrap_or(&self.inner.config.base_url);
        Self::validate_base_url(base_url)?;
        let combined = format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        reqwest::Url::parse(&combined).map_err(|error| RunwayError::Validation {
            message: format!("Invalid request URL: {error}"),
        })
    }

    fn parse_idempotency_key(value: &str) -> Result<HeaderValue, RunwayError> {
        if value.trim().is_empty() {
            return Err(RunwayError::Validation {
                message: "Idempotency key cannot be empty".into(),
            });
        }
        HeaderValue::from_str(value).map_err(|_| RunwayError::Validation {
            message: "Idempotency key contains invalid header characters".into(),
        })
    }

    fn resolve_request_options(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<ResolvedRequestOptions, RunwayError> {
        let mut headers = self.inner.config.default_headers.clone();
        let mut query = self.inner.config.default_query.clone();
        let mut timeout = None;
        let mut max_retries = self.inner.config.max_retries;
        let mut retry_non_idempotent = self.inner.config.retry_non_idempotent;
        let mut idempotency_key = None;
        let mut base_url = None;
        let mut cancellation_token = None;

        if let Some(options) = options {
            if options.headers.contains_key("idempotency-key") {
                return Err(RunwayError::Validation {
                    message: "Use RequestOptions::idempotency_key for Idempotency-Key".into(),
                });
            }
            headers.extend(options.headers.clone());
            query.extend(options.query.clone());
            timeout = options.timeout;
            max_retries = options.max_retries.unwrap_or(max_retries);
            retry_non_idempotent = options.retry_non_idempotent.unwrap_or(retry_non_idempotent);
            idempotency_key = options
                .idempotency_key
                .as_deref()
                .map(Self::parse_idempotency_key)
                .transpose()?;
            base_url = options.base_url.clone();
            cancellation_token = options.cancellation_token.clone();
            if timeout.is_some_and(|duration| duration.is_zero()) {
                return Err(RunwayError::Validation {
                    message: "Request timeout must be greater than zero".into(),
                });
            }
        }

        Ok(ResolvedRequestOptions {
            headers,
            query,
            timeout,
            max_retries,
            retry_non_idempotent,
            idempotency_key,
            base_url,
            cancellation_token,
        })
    }

    fn apply_request_options(
        &self,
        mut builder: reqwest::RequestBuilder,
        options: &ResolvedRequestOptions,
    ) -> reqwest::RequestBuilder {
        if !options.query.is_empty() {
            builder = builder.query(&options.query);
        }
        if !options.headers.is_empty() {
            builder = builder.headers(options.headers.clone());
        }
        if let Some(timeout) = options.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(idempotency_key) = &options.idempotency_key {
            builder = builder.header(
                HeaderName::from_static("idempotency-key"),
                idempotency_key.clone(),
            );
        }
        builder
    }

    async fn parse_json_response<Resp: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
        cancellation_token: Option<&CancellationToken>,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let (response, body) =
            Self::read_body_limited(resp, MAX_JSON_RESPONSE_BYTES, cancellation_token).await?;
        let data = serde_json::from_slice::<Resp>(&body).map_err(|source| {
            let excerpt_len = body.len().min(MAX_DECODE_EXCERPT_BYTES);
            RunwayError::ResponseDecode {
                status: response.status,
                message: source.to_string(),
                body_excerpt: ResponseBodyExcerpt::new(
                    String::from_utf8_lossy(&body[..excerpt_len]).into_owned(),
                ),
                source,
            }
        })?;
        Ok(WithResponse { data, response })
    }

    async fn read_body_limited(
        mut resp: reqwest::Response,
        limit_bytes: usize,
        cancellation_token: Option<&CancellationToken>,
    ) -> Result<(ResponseMetadata, Vec<u8>), RunwayError> {
        let response = ResponseMetadata {
            status: resp.status().as_u16(),
            headers: resp.headers().clone(),
        };
        if resp
            .content_length()
            .is_some_and(|content_length| content_length > limit_bytes as u64)
        {
            return Err(RunwayError::ResponseTooLarge {
                status: response.status,
                limit_bytes,
                content_length: resp.content_length(),
            });
        }

        let mut body = Vec::with_capacity(
            resp.content_length()
                .and_then(|length| usize::try_from(length).ok())
                .unwrap_or_default()
                .min(limit_bytes),
        );
        loop {
            let next_chunk = resp.chunk();
            let chunk = if let Some(cancellation_token) = cancellation_token {
                tokio::select! {
                    chunk = next_chunk => chunk?,
                    _ = cancellation_token.cancelled() => return Err(RunwayError::RequestAborted),
                }
            } else {
                next_chunk.await?
            };
            let Some(chunk) = chunk else {
                break;
            };
            if body.len().saturating_add(chunk.len()) > limit_bytes {
                return Err(RunwayError::ResponseTooLarge {
                    status: response.status,
                    limit_bytes,
                    content_length: resp.content_length(),
                });
            }
            body.extend_from_slice(&chunk);
        }
        Ok((response, body))
    }

    async fn parse_empty_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<WithResponse<()>, RunwayError> {
        let response = ResponseMetadata {
            status: resp.status().as_u16(),
            headers: resp.headers().clone(),
        };
        Ok(WithResponse { data: (), response })
    }

    async fn error_from_response(
        &self,
        resp: reqwest::Response,
        cancellation_token: Option<&CancellationToken>,
    ) -> RunwayError {
        let status = resp.status();
        let headers = resp.headers().clone();
        let retry_after = Self::parse_retry_after(&headers);
        let text = match Self::read_body_limited(resp, MAX_ERROR_RESPONSE_BYTES, cancellation_token)
            .await
        {
            Ok((_, body)) => String::from_utf8_lossy(&body).into_owned(),
            Err(error) => return error,
        };

        let (message, code) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            let error_obj = json.get("error").unwrap_or(&json);
            let message = error_obj
                .get("message")
                .and_then(|value| value.as_str())
                .map(ToOwned::to_owned)
                .or_else(|| error_obj.as_str().map(ToOwned::to_owned))
                .unwrap_or_else(|| text.clone());
            let code = error_obj
                .get("code")
                .and_then(|value| value.as_str())
                .map(ToOwned::to_owned);
            (message, code)
        } else {
            (text, None)
        };

        match status {
            reqwest::StatusCode::UNAUTHORIZED => RunwayError::Unauthorized {
                message,
                code,
                headers: Box::new(headers.into()),
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => RunwayError::RateLimited {
                retry_after,
                message,
                code,
                headers: Box::new(headers.into()),
            },
            _ => RunwayError::Api {
                status: status.as_u16(),
                kind: Self::classify_status(status),
                message,
                code,
                headers: Box::new(headers.into()),
            },
        }
    }

    async fn check_response(
        &self,
        resp: reqwest::Response,
        cancellation_token: Option<&CancellationToken>,
    ) -> Result<reqwest::Response, RunwayError> {
        if resp.status().is_success() {
            Ok(resp)
        } else {
            Err(self.error_from_response(resp, cancellation_token).await)
        }
    }

    async fn send_with_retry(
        &self,
        request_builder: impl Fn() -> reqwest::RequestBuilder,
        max_retries: u32,
        allow_automatic_retries: bool,
        cancellation_token: Option<&CancellationToken>,
    ) -> Result<reqwest::Response, RunwayError> {
        let mut retries = 0;

        loop {
            let send = request_builder().send();
            let send_result = if let Some(cancellation_token) = cancellation_token {
                tokio::select! {
                    result = send => result,
                    _ = cancellation_token.cancelled() => return Err(RunwayError::RequestAborted),
                }
            } else {
                send.await
            };

            let resp = match send_result {
                Ok(resp) => resp,
                Err(err) => {
                    let retryable = err.is_connect() || err.is_timeout();
                    if retryable && allow_automatic_retries && retries < max_retries {
                        let wait = Self::backoff_with_jitter(retries);
                        tracing::warn!("Request transport error, retrying in {:?}", wait);
                        Self::sleep_or_cancel(wait, cancellation_token).await?;
                        retries += 1;
                        continue;
                    }

                    if err.is_timeout() {
                        return Err(RunwayError::ConnectionTimeout {
                            source: err.without_url(),
                        });
                    }

                    let lowercase = err.to_string().to_ascii_lowercase();
                    if lowercase.contains("abort") || lowercase.contains("cancel") {
                        return Err(RunwayError::RequestAborted);
                    }

                    if err.is_connect() {
                        return Err(RunwayError::ConnectionError {
                            source: err.without_url(),
                        });
                    }

                    return Err(RunwayError::from(err));
                }
            };

            let should_retry_header = Self::parse_should_retry(resp.headers());
            let retryable_status = matches!(
                resp.status(),
                reqwest::StatusCode::REQUEST_TIMEOUT
                    | reqwest::StatusCode::CONFLICT
                    | reqwest::StatusCode::TOO_MANY_REQUESTS
            ) || resp.status().is_server_error();

            let should_retry = should_retry_header.unwrap_or(retryable_status);
            if should_retry && allow_automatic_retries && retries < max_retries {
                let wait = Self::retry_delay(resp.headers(), retries);
                tracing::warn!("Retrying {} response in {:?}", resp.status().as_u16(), wait);
                drop(resp);
                Self::sleep_or_cancel(wait, cancellation_token).await?;
                retries += 1;
                continue;
            }

            return self.check_response(resp, cancellation_token).await;
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

    fn classify_status(status: reqwest::StatusCode) -> ApiErrorKind {
        match status {
            reqwest::StatusCode::BAD_REQUEST => ApiErrorKind::BadRequest,
            reqwest::StatusCode::UNAUTHORIZED => ApiErrorKind::Authentication,
            reqwest::StatusCode::FORBIDDEN => ApiErrorKind::PermissionDenied,
            reqwest::StatusCode::NOT_FOUND => ApiErrorKind::NotFound,
            reqwest::StatusCode::CONFLICT => ApiErrorKind::Conflict,
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => ApiErrorKind::UnprocessableEntity,
            reqwest::StatusCode::TOO_MANY_REQUESTS => ApiErrorKind::RateLimited,
            status if status.is_server_error() => ApiErrorKind::InternalServer,
            _ => ApiErrorKind::Unknown,
        }
    }

    fn parse_should_retry(headers: &HeaderMap) -> Option<bool> {
        headers
            .get("x-should-retry")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            })
    }

    fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
        if let Some(value) = headers.get("retry-after-ms") {
            if let Ok(value) = value.to_str() {
                if let Ok(milliseconds) = value.parse::<f64>() {
                    if let Ok(duration) = Duration::try_from_secs_f64(milliseconds / 1_000.0) {
                        return Some(duration);
                    }
                }
            }
        }

        if let Some(value) = headers.get("retry-after") {
            if let Ok(value) = value.to_str() {
                if let Ok(seconds) = value.parse::<f64>() {
                    if let Ok(duration) = Duration::try_from_secs_f64(seconds) {
                        return Some(duration);
                    }
                }

                if let Ok(datetime) = httpdate::parse_http_date(value) {
                    let now = SystemTime::now();
                    return Some(
                        datetime
                            .duration_since(now)
                            .unwrap_or(Duration::from_secs(0)),
                    );
                }
            }
        }

        None
    }

    fn retry_delay(headers: &HeaderMap, retries: u32) -> Duration {
        Self::parse_retry_after(headers)
            .filter(|delay| !delay.is_zero() && *delay <= Duration::from_secs(60))
            .unwrap_or_else(|| Self::backoff_with_jitter(retries))
    }

    fn backoff_with_jitter(retries: u32) -> Duration {
        let base_ms = (500u64.saturating_mul(2u64.pow(retries.min(4)))).min(8_000);
        let jitter_quarters = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64)
            % 251;
        Duration::from_millis(base_ms * (750 + jitter_quarters) / 1_000)
    }

    pub(crate) async fn post_with_options<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;
        let body_bytes = serde_json::to_vec(body)?;
        let allow_automatic_retries =
            options.retry_non_idempotent || options.idempotency_key.is_some();

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::POST, url.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                allow_automatic_retries,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_json_response(resp, options.cancellation_token.as_ref())
            .await
    }

    #[cfg(feature = "unstable-endpoints")]
    pub(crate) async fn post<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, RunwayError> {
        Ok(self
            .post_with_options(path, body, &RequestOptions::default())
            .await?
            .data)
    }

    pub(crate) async fn get_with_options<Resp: DeserializeOwned>(
        &self,
        path: &str,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self.inner.http.request(Method::GET, url.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                true,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_json_response(resp, options.cancellation_token.as_ref())
            .await
    }

    pub(crate) async fn get_with_query_with_options<Query: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        query: &Query,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::GET, url.clone())
                        .query(query);
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                true,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_json_response(resp, options.cancellation_token.as_ref())
            .await
    }

    #[cfg(feature = "unstable-endpoints")]
    pub(crate) async fn get_with_query<Query: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        query: &Query,
    ) -> Result<Resp, RunwayError> {
        Ok(self
            .get_with_query_with_options(path, query, &RequestOptions::default())
            .await?
            .data)
    }

    pub(crate) async fn delete_with_options(
        &self,
        path: &str,
        options: &RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self.inner.http.request(Method::DELETE, url.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                true,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_empty_response(resp).await
    }

    pub(crate) async fn patch_with_options<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;
        let body_bytes = serde_json::to_vec(body)?;
        let allow_automatic_retries =
            options.retry_non_idempotent || options.idempotency_key.is_some();

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::PATCH, url.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                allow_automatic_retries,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_json_response(resp, options.cancellation_token.as_ref())
            .await
    }

    pub(crate) async fn patch_empty_with_options<Req: Serialize>(
        &self,
        path: &str,
        body: &Req,
        options: &RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        let options = self.resolve_request_options(Some(options))?;
        let url = self.url_with_base(path, options.base_url.as_deref())?;
        let body_bytes = serde_json::to_vec(body)?;
        let allow_automatic_retries =
            options.retry_non_idempotent || options.idempotency_key.is_some();

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::PATCH, url.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
                allow_automatic_retries,
                options.cancellation_token.as_ref(),
            )
            .await?;

        self.parse_empty_response(resp).await
    }
}

impl std::fmt::Debug for RunwayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunwayClient")
            .field("base_url", &RedactedUrl(&self.inner.config.base_url))
            .finish()
    }
}
