# runway-rs — Rust SDK for the Runway API

## Full Technical Specification

**Author:** Abdel (@AbdelStark)
**Version:** 1.0.0
**Status:** Ready to Build
**Estimated Build Time:** 5–7 days
**Crate Name:** `runway-sdk`

-----

## 1. Project Overview

### 1.1 Mission

Build the first production-grade Rust SDK for the Runway API. No official or community Rust SDK exists today — Runway only ships Python and Node SDKs. This fills a real ecosystem gap and demonstrates systems-level engineering to Runway’s team.

### 1.2 Strategic Value

- **Unique in market**: Zero Rust SDKs for Runway exist. First-mover advantage.
- **Signal to hiring team**: Demonstrates you can ship production Rust, understand their API deeply, and think about developer experience.
- **Ecosystem contribution**: Directly addresses the gap identified in the ecosystem analysis — Runway’s developer tooling lags behind OpenAI (which has Python, Node, .NET, Go, Java SDKs).
- **Blog post fuel**: “Why I Built a Rust SDK for Runway’s API” is a natural conversation starter.

### 1.3 Design Philosophy

Mirror the ergonomics of the official Node/Python SDKs while being idiomatically Rust. The developer should feel like they’re using a first-party SDK, not a wrapper.

-----

## 2. API Surface Coverage

Based on the official Runway API (version `2024-11-06`), the SDK must cover all endpoint groups:

### 2.1 Generation Endpoints (Task-based)

|Endpoint             |Method|Path                       |Rust Module                     |
|---------------------|------|---------------------------|--------------------------------|
|Image to Video       |POST  |`/v1/image_to_video`       |`client.image_to_video()`       |
|Text to Video        |POST  |`/v1/text_to_video`        |`client.text_to_video()`        |
|Video to Video       |POST  |`/v1/video_to_video`       |`client.video_to_video()`       |
|Text/Image to Image  |POST  |`/v1/text_to_image`        |`client.text_to_image()`        |
|Character Performance|POST  |`/v1/character_performance`|`client.character_performance()`|
|Sound Effects        |POST  |`/v1/sound_effect`         |`client.sound_effect()`         |
|Speech to Speech     |POST  |`/v1/speech_to_speech`     |`client.speech_to_speech()`     |
|Text to Speech       |POST  |`/v1/text_to_speech`       |`client.text_to_speech()`       |
|Voice Dubbing        |POST  |`/v1/voice_dubbing`        |`client.voice_dubbing()`        |
|Voice Isolation      |POST  |`/v1/voice_isolation`      |`client.voice_isolation()`      |

### 2.2 Task Management

|Endpoint          |Method|Path            |Rust Module                |
|------------------|------|----------------|---------------------------|
|Get Task Detail   |GET   |`/v1/tasks/{id}`|`client.tasks().get(id)`   |
|Cancel/Delete Task|DELETE|`/v1/tasks/{id}`|`client.tasks().delete(id)`|

### 2.3 Uploads

|Endpoint     |Method|Path         |Rust Module                        |
|-------------|------|-------------|-----------------------------------|
|Create Upload|POST  |`/v1/uploads`|`client.uploads().create(filename)`|

### 2.4 Workflows

|Endpoint      |Method|Path                           |Rust Module                            |
|--------------|------|-------------------------------|---------------------------------------|
|List Workflows|GET   |`/v1/workflows`                |`client.workflows().list()`            |
|Run Workflow  |POST  |`/v1/workflows/{id}`           |`client.workflows().run(id, params)`   |
|Get Workflow  |GET   |`/v1/workflows/{id}`           |`client.workflows().get(id)`           |
|Get Invocation|GET   |`/v1/workflow_invocations/{id}`|`client.workflow_invocations().get(id)`|

### 2.5 Avatars (Characters API)

|Endpoint     |Method|Path              |Rust Module                          |
|-------------|------|------------------|-------------------------------------|
|List Avatars |GET   |`/v1/avatars`     |`client.avatars().list()`            |
|Create Avatar|POST  |`/v1/avatars`     |`client.avatars().create(params)`    |
|Get Avatar   |GET   |`/v1/avatars/{id}`|`client.avatars().get(id)`           |
|Update Avatar|PATCH |`/v1/avatars/{id}`|`client.avatars().update(id, params)`|
|Delete Avatar|DELETE|`/v1/avatars/{id}`|`client.avatars().delete(id)`        |

### 2.6 Knowledge (Documents)

|Endpoint       |Method|Path                |Rust Module                            |
|---------------|------|--------------------|---------------------------------------|
|Create Document|POST  |`/v1/documents`     |`client.documents().create(params)`    |
|List Documents |GET   |`/v1/documents`     |`client.documents().list()`            |
|Get Document   |GET   |`/v1/documents/{id}`|`client.documents().get(id)`           |
|Update Document|PATCH |`/v1/documents/{id}`|`client.documents().update(id, params)`|
|Delete Document|DELETE|`/v1/documents/{id}`|`client.documents().delete(id)`        |

### 2.7 Realtime Sessions

|Endpoint      |Method|Path                        |Rust Module                                |
|--------------|------|----------------------------|-------------------------------------------|
|Create Session|POST  |`/v1/realtime_sessions`     |`client.realtime_sessions().create(params)`|
|Get Session   |GET   |`/v1/realtime_sessions/{id}`|`client.realtime_sessions().get(id)`       |
|Cancel Session|DELETE|`/v1/realtime_sessions/{id}`|`client.realtime_sessions().cancel(id)`    |

### 2.8 Organization

|Endpoint    |Method|Path                    |Rust Module                          |
|------------|------|------------------------|-------------------------------------|
|Get Org Info|GET   |`/v1/organization`      |`client.organization().get()`        |
|Query Usage |POST  |`/v1/organization/usage`|`client.organization().usage(params)`|

### 2.9 Voices

|Endpoint     |Method|Path                |Rust Module                      |
|-------------|------|--------------------|---------------------------------|
|List Voices  |GET   |`/v1/voices`        |`client.voices().list()`         |
|Create Voice |POST  |`/v1/voices`        |`client.voices().create(params)` |
|Get Voice    |GET   |`/v1/voices/{id}`   |`client.voices().get(id)`        |
|Delete Voice |DELETE|`/v1/voices/{id}`   |`client.voices().delete(id)`     |
|Preview Voice|POST  |`/v1/voices/preview`|`client.voices().preview(params)`|

-----

## 3. Architecture

### 3.1 Crate Structure

```
runway-sdk/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── examples/
│   ├── text_to_video.rs
│   ├── image_to_video.rs
│   ├── video_to_video.rs
│   ├── avatars.rs
│   ├── workflows.rs
│   └── poll_task.rs
├── src/
│   ├── lib.rs                  # Re-exports, RunwayClient constructor
│   ├── client.rs               # Core HTTP client, auth, retry, headers
│   ├── config.rs               # Configuration (api key, base url, timeouts)
│   ├── error.rs                # Error types (RunwayError, ApiError, TaskFailedError)
│   ├── types/
│   │   ├── mod.rs
│   │   ├── task.rs             # Task, TaskStatus, TaskOutput
│   │   ├── models.rs           # Model enums (Gen45, Gen4Turbo, etc.)
│   │   ├── media.rs            # PromptImage, VideoUri, AudioUri, DataUri
│   │   ├── generation.rs       # All generation request/response types
│   │   ├── avatar.rs           # Avatar types
│   │   ├── workflow.rs         # Workflow types
│   │   ├── voice.rs            # Voice types
│   │   ├── document.rs         # Document types
│   │   ├── organization.rs     # Org, usage types
│   │   ├── realtime.rs         # Realtime session types
│   │   └── common.rs           # Shared types (ContentModeration, Ratio, etc.)
│   ├── resources/
│   │   ├── mod.rs
│   │   ├── image_to_video.rs
│   │   ├── text_to_video.rs
│   │   ├── video_to_video.rs
│   │   ├── text_to_image.rs
│   │   ├── character_performance.rs
│   │   ├── sound_effect.rs
│   │   ├── speech_to_speech.rs
│   │   ├── text_to_speech.rs
│   │   ├── voice_dubbing.rs
│   │   ├── voice_isolation.rs
│   │   ├── tasks.rs
│   │   ├── uploads.rs
│   │   ├── workflows.rs
│   │   ├── avatars.rs
│   │   ├── documents.rs
│   │   ├── realtime_sessions.rs
│   │   ├── organization.rs
│   │   └── voices.rs
│   └── polling.rs              # Task polling with configurable backoff
└── tests/
    ├── integration/
    │   └── mock_server.rs      # Integration tests with wiremock
    └── unit/
        ├── types_test.rs
        └── polling_test.rs
```

### 3.2 Dependencies

```toml
[package]
name = "runway-sdk"
version = "0.1.0"
edition = "2021"
description = "Unofficial Rust SDK for the Runway API — AI video generation, world models, and more"
license = "MIT OR Apache-2.0"
repository = "https://github.com/AbdelStark/runway-rs"
keywords = ["runway", "ai", "video-generation", "generative-ai", "sdk"]
categories = ["api-bindings", "multimedia::video"]

[dependencies]
reqwest = { version = "0.12", features = ["json", "multipart", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
uuid = { version = "1", features = ["serde", "v4"] }
url = "2"
tracing = "0.1"
backoff = { version = "0.4", features = ["tokio"] }
base64 = "0.22"
mime_guess = "2"

[dev-dependencies]
wiremock = "0.6"
tokio-test = "0.4"
assert_matches = "1"
```

### 3.3 Core Client Design

```rust
use std::time::Duration;

pub struct RunwayClient {
    http: reqwest::Client,
    config: Config,
}

pub struct Config {
    pub api_key: String,
    pub base_url: String,           // default: "https://api.dev.runwayml.com"
    pub api_version: String,        // default: "2024-11-06"
    pub timeout: Duration,          // default: 300s (video gen is slow)
    pub max_retries: u32,           // default: 3
    pub poll_interval: Duration,    // default: 5s (API doc says no faster than 5s)
    pub max_poll_duration: Duration,// default: 600s
}

impl RunwayClient {
    /// Create from RUNWAYML_API_SECRET env var
    pub fn new() -> Result<Self, RunwayError>;

    /// Create with explicit API key
    pub fn with_api_key(key: impl Into<String>) -> Self;

    /// Create with full config
    pub fn with_config(config: Config) -> Self;

    // Resource accessors
    pub fn image_to_video(&self) -> ImageToVideoResource<'_>;
    pub fn text_to_video(&self) -> TextToVideoResource<'_>;
    pub fn video_to_video(&self) -> VideoToVideoResource<'_>;
    pub fn text_to_image(&self) -> TextToImageResource<'_>;
    pub fn character_performance(&self) -> CharacterPerformanceResource<'_>;
    pub fn sound_effect(&self) -> SoundEffectResource<'_>;
    pub fn speech_to_speech(&self) -> SpeechToSpeechResource<'_>;
    pub fn text_to_speech(&self) -> TextToSpeechResource<'_>;
    pub fn voice_dubbing(&self) -> VoiceDubbingResource<'_>;
    pub fn voice_isolation(&self) -> VoiceIsolationResource<'_>;
    pub fn tasks(&self) -> TasksResource<'_>;
    pub fn uploads(&self) -> UploadsResource<'_>;
    pub fn workflows(&self) -> WorkflowsResource<'_>;
    pub fn workflow_invocations(&self) -> WorkflowInvocationsResource<'_>;
    pub fn avatars(&self) -> AvatarsResource<'_>;
    pub fn documents(&self) -> DocumentsResource<'_>;
    pub fn realtime_sessions(&self) -> RealtimeSessionsResource<'_>;
    pub fn organization(&self) -> OrganizationResource<'_>;
    pub fn voices(&self) -> VoicesResource<'_>;
}
```

-----

## 4. Type System

### 4.1 Model Enums

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoModel {
    #[serde(rename = "gen4.5")]
    Gen45,
    #[serde(rename = "gen4_turbo")]
    Gen4Turbo,
    #[serde(rename = "gen3a_turbo")]
    Gen3aTurbo,
    #[serde(rename = "veo3.1")]
    Veo31,
    #[serde(rename = "veo3.1_fast")]
    Veo31Fast,
    #[serde(rename = "veo3")]
    Veo3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageModel {
    #[serde(rename = "gen4_image_turbo")]
    Gen4ImageTurbo,
    #[serde(rename = "gen4_image")]
    Gen4Image,
    #[serde(rename = "gemini_2.5_flash")]
    Gemini25Flash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoRatio {
    #[serde(rename = "1280:720")]
    Landscape,
    #[serde(rename = "720:1280")]
    Portrait,
    #[serde(rename = "1104:832")]
    Wide,
    #[serde(rename = "960:960")]
    Square,
    #[serde(rename = "832:1104")]
    Tall,
    #[serde(rename = "1584:672")]
    Ultrawide,
}
```

### 4.2 Task Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Pending,
    Throttled,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub status: TaskStatus,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<String>>,    // URLs to generated media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}
```

### 4.3 Media Input Types

```rust
/// Flexible media input — accepts URLs, Runway upload URIs, or data URIs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaInput {
    Url(String),           // https://...
    RunwayUri(String),     // runway://...
    DataUri(String),       // data:image/..., data:video/..., data:audio/...
}

impl MediaInput {
    pub fn from_url(url: impl Into<String>) -> Self;
    pub fn from_runway_uri(uri: impl Into<String>) -> Self;
    pub fn from_base64(mime_type: &str, data: &[u8]) -> Self;
    pub fn from_file(path: &std::path::Path) -> Result<Self, RunwayError>;
}
```

### 4.4 Generation Request Builders

```rust
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub prompt_image: MediaInput,
    pub ratio: VideoRatio,
    pub duration: u8,                       // 2..=10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

impl ImageToVideoRequest {
    pub fn new(model: VideoModel, prompt: impl Into<String>, image: MediaInput) -> Self;
    pub fn ratio(mut self, ratio: VideoRatio) -> Self;
    pub fn duration(mut self, secs: u8) -> Self;
    pub fn seed(mut self, seed: u32) -> Self;
}
```

-----

## 5. Task Polling System

The killer feature. All generation endpoints return a task ID. The SDK must poll until completion.

```rust
pub struct PendingTask {
    client: RunwayClient,
    task_id: Uuid,
}

impl PendingTask {
    /// Poll until task succeeds or fails. Returns completed Task with output URLs.
    pub async fn wait_for_output(self) -> Result<Task, RunwayError>;

    /// Poll with custom interval and timeout
    pub async fn wait_with_config(
        self,
        poll_interval: Duration,
        max_duration: Duration,
    ) -> Result<Task, RunwayError>;

    /// Get the task ID without waiting
    pub fn id(&self) -> Uuid;

    /// Stream task status updates
    pub fn stream_status(self) -> impl Stream<Item = Result<Task, RunwayError>>;
}

// Usage:
let task = client.image_to_video()
    .create(request)
    .await?                          // Returns PendingTask
    .wait_for_output()
    .await?;                         // Returns completed Task

println!("Video URL: {}", task.output.unwrap()[0]);
```

### 5.1 Polling Implementation

- Initial delay: 2 seconds
- Poll interval: 5 seconds (API docs say no faster)
- Max duration: configurable, default 600s
- Exponential backoff on HTTP 429 (rate limit)
- Returns `RunwayError::TaskFailed` with failure code and message on `FAILED` status
- Returns `RunwayError::Timeout` if max duration exceeded

-----

## 6. Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum RunwayError {
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String, code: Option<String> },

    #[error("Task failed: {message} (code: {code})")]
    TaskFailed { task_id: Uuid, message: String, code: String },

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
```

-----

## 7. Authentication & Headers

Every request must include:

```
Authorization: Bearer <RUNWAYML_API_SECRET>
X-Runway-Version: 2024-11-06
Content-Type: application/json
```

The API key is read from `RUNWAYML_API_SECRET` env var by default, matching the official SDK behavior.

-----

## 8. Examples

### 8.1 Text to Video (Minimal)

```rust
use runway_sdk::RunwayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client.text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "A serene mountain landscape at sunrise with mist rolling through the valleys",
        ).ratio(VideoRatio::Landscape).duration(5))
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output.unwrap()[0]);
    Ok(())
}
```

### 8.2 Image to Video with File Upload

```rust
use runway_sdk::{RunwayClient, MediaInput};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Upload a local file first
    let upload_uri = client.uploads()
        .upload_file(Path::new("./my-image.jpg"))
        .await?;

    let task = client.image_to_video()
        .create(ImageToVideoRequest::new(
            VideoModel::Gen4Turbo,
            "A gentle zoom into the scene with soft ambient lighting",
            MediaInput::from_runway_uri(upload_uri),
        ).duration(10))
        .await?
        .wait_for_output()
        .await?;

    println!("Video: {}", task.output.unwrap()[0]);
    Ok(())
}
```

-----

## 9. Testing Strategy

### 9.1 Unit Tests

- Serialization/deserialization of all request/response types
- Builder pattern validation
- Error type mapping from HTTP status codes
- Media input construction (URL, data URI, file path)

### 9.2 Integration Tests (wiremock)

- Mock server returning task IDs
- Mock polling sequences (PENDING → RUNNING → SUCCEEDED)
- Mock failure scenarios (FAILED with code)
- Mock rate limiting (429 + retry)
- Mock upload flow (two-step presigned URL)

### 9.3 Live Tests (behind feature flag)

```toml
[features]
live-tests = []
```

Run with actual API key for end-to-end validation. Gated behind `cargo test --features live-tests`.

-----

## 10. Documentation & Publishing

### 10.1 README.md Structure

- Hero banner with Runway logo
- “Unofficial Rust SDK” badge
- Installation (`cargo add runway-sdk`)
- Quick start (text to video in 10 lines)
- Feature matrix (which endpoints are supported)
- Full examples list
- Configuration options
- Error handling guide
- Link to Runway API docs
- License (MIT OR Apache-2.0)
- “Built by @AbdelStark as an open-source contribution to the Runway developer ecosystem”

### 10.2 Publishing

- Publish to crates.io with `cargo publish`
- Create GitHub release with changelog
- Post on Twitter tagging @runwayml, @agermanidis, @c_valenzuelab
- Write blog post: “Why I Built a Rust SDK for Runway’s API”

-----

## 11. Future Roadmap (v0.2+)

- Streaming task status via `async Stream`
- Automatic file type detection for uploads
- Webhook support (when Runway adds it)
- CLI companion tool (`runway-cli`)
- WebAssembly target for browser usage
- OpenTelemetry tracing integration
