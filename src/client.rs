use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, HeaderValue as ReqHeaderValue, AUTHORIZATION, CONTENT_TYPE,
};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::config::Config;
use crate::error::{ApiErrorKind, RunwayError};
use crate::resources::*;

/// Response metadata exposed alongside parsed bodies.
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    /// HTTP status code returned by the Runway API.
    pub status: u16,
    /// Response headers returned by the Runway API.
    pub headers: HeaderMap,
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
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// Extra headers merged into the request.
    pub headers: HeaderMap,
    /// Query string pairs appended to the request URL.
    pub query: Vec<(String, String)>,
    /// Request-specific timeout override.
    pub timeout: Option<Duration>,
    /// Request-specific retry budget override.
    pub max_retries: Option<u32>,
    /// Idempotency key sent as `Idempotency-Key` when present.
    pub idempotency_key: Option<String>,
    /// Base URL override for this request.
    pub base_url: Option<String>,
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
}

#[derive(Clone)]
struct ResolvedRequestOptions {
    headers: HeaderMap,
    query: Vec<(String, String)>,
    timeout: Option<Duration>,
    max_retries: u32,
    idempotency_key: Option<String>,
    base_url: Option<String>,
}

/// Inner client state shared via `Arc`.
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
        headers.extend(config.default_headers.clone());

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .map_err(RunwayError::Http)?;

        Ok(Self {
            inner: Arc::new(ClientInner { http, config }),
        })
    }

    /// Clone this client with per-request defaults applied to the derived instance.
    ///
    /// This is useful when a subset of requests should share alternate headers,
    /// query parameters, retry limits, or a different base URL.
    pub fn with_options(&self, options: RequestOptions) -> Result<Self, RunwayError> {
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

        config.default_headers.extend(options.headers);
        config.default_query.extend(options.query);

        Self::with_config(config)
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

    #[cfg(feature = "unstable-endpoints")]
    pub fn lip_sync(&self) -> LipSyncResource {
        LipSyncResource {
            client: self.clone(),
        }
    }

    #[cfg(feature = "unstable-endpoints")]
    pub fn image_upscale(&self) -> ImageUpscaleResource {
        ImageUpscaleResource {
            client: self.clone(),
        }
    }

    // ── Internal HTTP helpers ───────────────────────────────────────────

    fn url_with_base(&self, path: &str, base_url_override: Option<&str>) -> String {
        format!(
            "{}{}",
            base_url_override.unwrap_or(&self.inner.config.base_url),
            path
        )
    }

    fn resolve_request_options(&self, options: Option<&RequestOptions>) -> ResolvedRequestOptions {
        let mut headers = self.inner.config.default_headers.clone();
        let mut query = self.inner.config.default_query.clone();
        let mut timeout = None;
        let mut max_retries = self.inner.config.max_retries;
        let mut idempotency_key = None;
        let mut base_url = None;

        if let Some(options) = options {
            headers.extend(options.headers.clone());
            query.extend(options.query.clone());
            timeout = options.timeout;
            max_retries = options.max_retries.unwrap_or(max_retries);
            idempotency_key = options.idempotency_key.clone();
            base_url = options.base_url.clone();
        }

        ResolvedRequestOptions {
            headers,
            query,
            timeout,
            max_retries,
            idempotency_key,
            base_url,
        }
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
                ReqHeaderValue::from_str(idempotency_key)
                    .unwrap_or_else(|_| ReqHeaderValue::from_static("invalid-idempotency-key")),
            );
        }
        builder
    }

    async fn parse_json_response<Resp: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let response = ResponseMetadata {
            status: resp.status().as_u16(),
            headers: resp.headers().clone(),
        };
        let data = resp.json::<Resp>().await?;
        Ok(WithResponse { data, response })
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

    async fn error_from_response(&self, resp: reqwest::Response) -> RunwayError {
        let status = resp.status();
        let headers = resp.headers().clone();
        let retry_after = Self::parse_retry_after(&headers);
        let text = resp.text().await.unwrap_or_default();

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
            reqwest::StatusCode::UNAUTHORIZED => RunwayError::Unauthorized,
            reqwest::StatusCode::TOO_MANY_REQUESTS => RunwayError::RateLimited {
                retry_after,
                message,
                code,
                headers: Box::new(headers),
            },
            _ => RunwayError::Api {
                status: status.as_u16(),
                kind: Self::classify_status(status),
                message,
                code,
                headers: Box::new(headers),
            },
        }
    }

    async fn check_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<reqwest::Response, RunwayError> {
        if resp.status().is_success() {
            Ok(resp)
        } else {
            Err(self.error_from_response(resp).await)
        }
    }

    async fn send_with_retry(
        &self,
        request_builder: impl Fn() -> reqwest::RequestBuilder,
        max_retries: u32,
    ) -> Result<reqwest::Response, RunwayError> {
        let mut retries = 0;

        loop {
            let resp = match request_builder().send().await {
                Ok(resp) => resp,
                Err(err) => {
                    let retryable = err.is_connect() || err.is_timeout();
                    if retryable && retries < max_retries {
                        let wait = Self::backoff_with_jitter(retries);
                        tracing::warn!("Request transport error, retrying in {:?}", wait);
                        tokio::time::sleep(wait).await;
                        retries += 1;
                        continue;
                    }

                    if err.is_timeout() {
                        return Err(RunwayError::ConnectionTimeout);
                    }

                    let message = err.to_string();
                    let lowercase = message.to_ascii_lowercase();
                    if lowercase.contains("abort") || lowercase.contains("cancel") {
                        return Err(RunwayError::RequestAborted);
                    }

                    if err.is_connect() {
                        return Err(RunwayError::ConnectionError { message });
                    }

                    return Err(RunwayError::Http(err));
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
            if should_retry && retries < max_retries {
                let wait = Self::retry_delay(resp.headers(), retries);
                tracing::warn!("Retrying {} response in {:?}", resp.status().as_u16(), wait);
                drop(resp);
                tokio::time::sleep(wait).await;
                retries += 1;
                continue;
            }

            return self.check_response(resp).await;
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
            .and_then(|value| match value {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            })
    }

    fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
        if let Some(value) = headers.get("retry-after-ms") {
            if let Ok(value) = value.to_str() {
                if let Ok(milliseconds) = value.parse::<u64>() {
                    return Some(Duration::from_millis(milliseconds));
                }
            }
        }

        if let Some(value) = headers.get("retry-after") {
            if let Ok(value) = value.to_str() {
                if let Ok(seconds) = value.parse::<u64>() {
                    return Some(Duration::from_secs(seconds));
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
        Self::parse_retry_after(headers).unwrap_or_else(|| Self::backoff_with_jitter(retries))
    }

    fn backoff_with_jitter(retries: u32) -> Duration {
        let base_ms = 500u64 * 2u64.pow(retries.min(6));
        let jitter_ms = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64)
            % 500;
        Duration::from_millis(base_ms + jitter_ms)
    }

    pub(crate) async fn post_with_options<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());
        let body_bytes = serde_json::to_vec(body)?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::POST, &url)
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
            )
            .await?;

        self.parse_json_response(resp).await
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
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());

        let resp = self
            .send_with_retry(
                || {
                    let builder = self.inner.http.request(Method::GET, &url);
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
            )
            .await?;

        self.parse_json_response(resp).await
    }

    pub(crate) async fn get<Resp: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Resp, RunwayError> {
        Ok(self
            .get_with_options(path, &RequestOptions::default())
            .await?
            .data)
    }

    pub(crate) async fn get_with_query_with_options<Query: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        query: &Query,
        options: &RequestOptions,
    ) -> Result<WithResponse<Resp>, RunwayError> {
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());

        let resp = self
            .send_with_retry(
                || {
                    let builder = self.inner.http.request(Method::GET, &url).query(query);
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
            )
            .await?;

        self.parse_json_response(resp).await
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
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());

        let resp = self
            .send_with_retry(
                || {
                    let builder = self.inner.http.request(Method::DELETE, &url);
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
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
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());
        let body_bytes = serde_json::to_vec(body)?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::PATCH, &url)
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
            )
            .await?;

        self.parse_json_response(resp).await
    }

    pub(crate) async fn patch_empty_with_options<Req: Serialize>(
        &self,
        path: &str,
        body: &Req,
        options: &RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        let options = self.resolve_request_options(Some(options));
        let url = self.url_with_base(path, options.base_url.as_deref());
        let body_bytes = serde_json::to_vec(body)?;

        let resp = self
            .send_with_retry(
                || {
                    let builder = self
                        .inner
                        .http
                        .request(Method::PATCH, &url)
                        .header(CONTENT_TYPE, "application/json")
                        .body(body_bytes.clone());
                    self.apply_request_options(builder, &options)
                },
                options.max_retries,
            )
            .await?;

        self.parse_empty_response(resp).await
    }
}

impl std::fmt::Debug for RunwayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunwayClient")
            .field("base_url", &self.inner.config.base_url)
            .finish()
    }
}
