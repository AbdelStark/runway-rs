# runway-rs

Unofficial Rust SDK for the [Runway API](https://docs.dev.runwayml.com/), aligned to the official SDK contract for the stable surface and explicit about any unofficial extensions.

[![CI](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Installation

```toml
[dependencies]
runway-sdk = "0.1"
```

Enable unofficial extensions only if you need them:

```toml
[dependencies]
runway-sdk = { version = "0.1", features = ["unstable-endpoints"] }
```

## Quick Start

```rust
use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_video()
        .create(TextToVideoGen45Request::new(
            "A serene mountain at sunrise",
            VideoRatio::Landscape,
            5,
        ))
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
```

## Authentication And Configuration

Set `RUNWAYML_API_SECRET`, or pass the key explicitly:

```rust
let client = runway_sdk::RunwayClient::with_api_key("your-api-key")?;
```

Use [`Config`](src/config.rs) for client-wide defaults:

```rust
use runway_sdk::{Config, RunwayClient};
use std::time::Duration;

let config = Config::new("your-api-key")
    .timeout(Duration::from_secs(120))
    .max_retries(5)
    .poll_interval(Duration::from_secs(5));

let client = RunwayClient::with_config(config)?;
```

## Stable API Coverage

### Generation Resources

All stable generation endpoints return a `PendingTask`.

| Resource | Endpoint |
|----------|----------|
| `text_to_video()` | `POST /v1/text_to_video` |
| `image_to_video()` | `POST /v1/image_to_video` |
| `video_to_video()` | `POST /v1/video_to_video` |
| `text_to_image()` | `POST /v1/text_to_image` |
| `character_performance()` | `POST /v1/character_performance` |
| `sound_effect()` | `POST /v1/sound_effect` |
| `text_to_speech()` | `POST /v1/text_to_speech` |
| `speech_to_speech()` | `POST /v1/speech_to_speech` |
| `voice_dubbing()` | `POST /v1/voice_dubbing` |
| `voice_isolation()` | `POST /v1/voice_isolation` |

Request bodies are model-specific. For example:

- `TextToVideoGen45Request`
- `ImageToVideoGen4TurboRequest`
- `TextToImageGen4ImageTurboRequest`
- `VideoToVideoRequest`

### Management Resources

| Resource | Stable methods |
|----------|----------------|
| `tasks()` | `retrieve`, `delete` |
| `uploads()` | `create_ephemeral`, `upload_file` |
| `avatars()` | `list`, `retrieve`, `create`, `update`, `delete` |
| `documents()` | `list`, `retrieve`, `create`, `update`, `delete` |
| `voices()` | `list`, `retrieve`, `create`, `delete`, `preview` |
| `workflows()` | `list`, `retrieve`, `run`, `run_pending` |
| `workflow_invocations()` | `retrieve`, `pending` |
| `realtime_sessions()` | `create`, `retrieve`, `cancel` |
| `organization()` | `retrieve`, `retrieve_usage` |

Deprecated-style aliases such as `get()` remain temporarily where needed, but docs use the stable names only.

## Per-Request Options

The runtime supports per-request headers, query params, timeout overrides, retry overrides, idempotency keys, and base URL overrides.

```rust
use runway_sdk::{RequestOptions, RunwayClient, TextToVideoGen45Request, VideoRatio};
use std::time::Duration;

let client = RunwayClient::new()?;
let request = TextToVideoGen45Request::new(
    "A cinematic drone shot over a glacier",
    VideoRatio::Landscape,
    5,
);

let response = client
    .text_to_video()
    .create_with_options(
        request,
        RequestOptions::new()
            .timeout(Duration::from_secs(90))
            .idempotency_key("job-123"),
    )
    .await?;

println!("HTTP status: {}", response.response.status);
println!("Task id: {}", response.data.id());
```

## Polling

Task and workflow polling both support timeout and cancellation controls.

```rust
use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio, WaitOptions};
use std::time::Duration;

let client = RunwayClient::new()?;
let pending = client
    .text_to_video()
    .create(TextToVideoGen45Request::new(
        "A cat running through fresh snow",
        VideoRatio::Landscape,
        5,
    ))
    .await?;

let task = pending
    .wait_with_options(WaitOptions::default().timeout(Duration::from_secs(300)))
    .await?;

println!("{}", task.output_urls().unwrap()[0]);
```

## Unofficial Extensions

The default crate surface tracks the official SDK contract. Unofficial endpoints are behind the `unstable-endpoints` feature:

- `lip_sync()`
- `image_upscale()`
- task `list`, `list_stream`, `list_all`, and `cancel`

Examples that depend on these extensions require the feature:

```sh
cargo run --example lip_sync --features unstable-endpoints
cargo run --example image_upscale --features unstable-endpoints
cargo run --example list_tasks --features unstable-endpoints
```

## Error Handling

All operations return `Result<T, RunwayError>`. The error type includes:

- typed API failures via `RunwayError::Api { status, kind, message, code, .. }`
- `RunwayError::RateLimited { retry_after, .. }`
- task and workflow terminal failures
- connection, timeout, abort, JSON, and IO errors
- local contract validation errors before a request is sent

## Examples

Stable examples:

```sh
export RUNWAYML_API_SECRET=your_key
cargo run --example text_to_video
cargo run --example image_to_video
cargo run --example text_to_image
cargo run --example uploads
cargo run --example workflows
```

See [`examples/`](examples/) for the full set.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

Built by [@AbdelStark](https://github.com/AbdelStark) as an open-source contribution to the Runway developer ecosystem.
