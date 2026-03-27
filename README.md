# runway-rs

Unofficial Rust SDK for the [Runway API](https://docs.dev.runwayml.com/) — async client for AI video, image, and audio generation.

[![CI](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Installation

```toml
[dependencies]
runway-sdk = "0.1"
```

## Quick Start

```rust
use runway_sdk::{RunwayClient, TextToVideoRequest, VideoModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?; // reads RUNWAYML_API_SECRET env var

    let task = client
        .text_to_video()
        .create(TextToVideoRequest::new(VideoModel::Gen45, "A serene mountain at sunrise"))
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output.unwrap()[0]);
    Ok(())
}
```

## Authentication

Set the `RUNWAYML_API_SECRET` environment variable, or pass the key explicitly:

```rust
let client = RunwayClient::with_api_key("your-api-key")?;
```

For full control over timeouts, retries, and polling behavior:

```rust
use runway_sdk::Config;
use std::time::Duration;

let config = Config::new("your-api-key")
    .timeout(Duration::from_secs(120))
    .max_retries(5)
    .poll_interval(Duration::from_secs(10));

let client = RunwayClient::with_config(config)?;
```

## API Coverage

### Generation (Task-based)

All generation methods return a `PendingTask` that can be polled or streamed:

| Method | Endpoint | Usage |
|--------|----------|-------|
| Text to Video | `POST /v1/text_to_video` | `client.text_to_video().create(req)` |
| Image to Video | `POST /v1/image_to_video` | `client.image_to_video().create(req)` |
| Video to Video | `POST /v1/video_to_video` | `client.video_to_video().create(req)` |
| Text to Image | `POST /v1/text_to_image` | `client.text_to_image().create(req)` |
| Character Performance | `POST /v1/character_performance` | `client.character_performance().create(req)` |
| Sound Effect | `POST /v1/sound_effect` | `client.sound_effect().create(req)` |
| Text to Speech | `POST /v1/text_to_speech` | `client.text_to_speech().create(req)` |
| Speech to Speech | `POST /v1/speech_to_speech` | `client.speech_to_speech().create(req)` |
| Voice Dubbing | `POST /v1/voice_dubbing` | `client.voice_dubbing().create(req)` |
| Voice Isolation | `POST /v1/voice_isolation` | `client.voice_isolation().create(req)` |

### Management Resources

| Resource | Operations |
|----------|------------|
| Tasks | `get`, `delete` |
| Uploads | `create` |
| Avatars | `list`, `get`, `create`, `update`, `delete` |
| Documents | `list`, `get`, `create`, `update`, `delete` |
| Voices | `list`, `get`, `create`, `delete`, `preview` |
| Workflows | `list`, `get`, `run` |
| Workflow Invocations | `get` |
| Realtime Sessions | `create`, `get`, `cancel` |
| Organization | `get`, `usage` |

## Task Polling

Generation endpoints return a `PendingTask`. Poll until completion or stream status updates:

```rust
// Wait for completion (blocking poll)
let task = pending_task.wait_for_output().await?;

// Or stream status updates
use futures::StreamExt;
let mut stream = pending_task.stream_status();
while let Some(update) = stream.next().await {
    let task = update?;
    println!("Status: {:?}, Progress: {:?}", task.status, task.progress);
}
```

## Examples

See the [`examples/`](examples/) directory:

```sh
export RUNWAYML_API_SECRET=your_key
cargo run --example text_to_video
cargo run --example image_to_video
cargo run --example video_to_video
cargo run --example avatars
cargo run --example workflows
cargo run --example poll_task
```

## Error Handling

All operations return `Result<T, RunwayError>`. Error variants include:

- `Api` — non-2xx response with status code and message
- `TaskFailed` — generation task failed with failure code
- `RateLimited` — HTTP 429 (retried automatically up to `max_retries`)
- `Timeout` — polling exceeded `max_poll_duration`
- `Unauthorized` — invalid or missing API key
- `Validation` — invalid input
- `Http`, `Json`, `Io` — transport and serialization errors

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

Built by [@AbdelStark](https://github.com/AbdelStark) as an open-source contribution to the Runway developer ecosystem.
