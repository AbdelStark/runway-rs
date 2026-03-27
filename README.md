# runway-sdk

Async Rust SDK for the Runway API with typed generation requests, uploads, task polling, and workflow invocations.

[![crates.io](https://img.shields.io/crates/v/runway-sdk.svg)](https://crates.io/crates/runway-sdk)
[![docs.rs](https://img.shields.io/docsrs/runway-sdk)](https://docs.rs/runway-sdk)
[![CI](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/runway-sdk)](./LICENSE-MIT)

## How It Works

```text
prompt / asset bytes
        |
        v
+-----------------------------+
| typed request builders      |
| TextToVideoGen45Request     |
| CreateEphemeralUploadRequest|
+-----------------------------+
        |
        v
+-----------------------------+
| RunwayClient                |
| reqwest + retries + headers |
+-----------------------------+
        |
        v
+-----------------------------+      +-----------------------------+
| /v1/tasks                   | <--> | PendingTask                 |
| /v1/workflows               |      | wait_for_output()           |
| /v1/uploads                 |      | stream_status()             |
+-----------------------------+      +-----------------------------+
        |
        v
output URLs / typed workflow results / response metadata
```

## Quick Start

1. Create a project and add the crate.

```bash
cargo new runway-smoke
cd runway-smoke
cargo add runway-sdk
cargo add tokio --features macros,rt-multi-thread
```

2. Set your Runway API secret.

```bash
export RUNWAYML_API_SECRET=your_secret_here
```

3. Run a safe organization smoke test.

```rust
use runway_sdk::RunwayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;
    let org = client.organization().retrieve().await?;

    println!("Credit balance: {}", org.credit_balance);
    Ok(())
}
```

```text
Credit balance: <number>
```

## The Good Stuff

### 1. Submit text-to-video and wait for the final asset

```rust
use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_video()
        .create(TextToVideoGen45Request::new(
            "Aerial shot of a glacier at sunrise",
            VideoRatio::Landscape,
            5,
        ))
        .await?
        .wait_for_output()
        .await?;

    println!("{}", task.output_urls().unwrap()[0]);
    Ok(())
}
```

- `TextToVideoGen45Request` keeps request fields model-specific instead of flattening everything into one permissive struct.
- `VideoRatio::Landscape` maps to `1280:720`.
- `wait_for_output()` exits on `SUCCEEDED`, `FAILED`, or `CANCELLED`.

### 2. Upload media once and reuse the returned `runway://` URI

```rust
use runway_sdk::{
    CreateEphemeralUploadRequest, ImageToVideoGen4TurboRequest, RunwayClient, VideoRatio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;
    let bytes = std::fs::read("input.png")?;

    let upload = client
        .uploads()
        .create_ephemeral(
            CreateEphemeralUploadRequest::new("input.png", bytes).content_type("image/png"),
        )
        .await?;

    let task = client
        .image_to_video()
        .create(
            ImageToVideoGen4TurboRequest::new(upload.uri, VideoRatio::Landscape)
                .prompt_text("Animate the uploaded image"),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("{}", task.output_urls().unwrap()[0]);
    Ok(())
}
```

- `create_ephemeral()` follows the placeholder-creation plus multipart upload flow used by the official SDK.
- `upload.uri` is the `runway://...` handle you pass to downstream generation endpoints.
- Upload creation can be blocked by Runway billing rules on unfunded accounts.

### 3. Launch workflows with typed node output values

```rust
use runway_sdk::{PrimitiveNodeValue, RunWorkflowRequest, RunwayClient, WorkflowNodeOutputValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;
    let workflows = client.workflows().list().await?;

    let version = workflows
        .data
        .first()
        .and_then(|workflow| workflow.versions.first())
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "no workflow versions available")
        })?;

    let invocation = client
        .workflows()
        .run_pending(
            &version.id,
            RunWorkflowRequest::new().node_output(
                "prompt-node",
                "prompt",
                WorkflowNodeOutputValue::Primitive {
                    value: PrimitiveNodeValue::from("hello world"),
                },
            ),
        )
        .await?;

    println!("{}", invocation.id());
    Ok(())
}
```

- `node_output()` builds the `nodeOutputs` map without hand-rolled JSON.
- `run_pending()` returns a `PendingWorkflowInvocation` that supports the same polling API as generation tasks.
- `workflow_invocations().pending(id)` is the direct entry point when you already have an invocation ID.

## Configuration / API

| Knob | Default | Description |
| --- | --- | --- |
| `RUNWAYML_API_SECRET` | none | Bearer secret read by `RunwayClient::new()`. |
| `Config::base_url` | `https://api.dev.runwayml.com` | API host override. |
| `Config::api_version` | `2024-11-06` | Sent as `X-Runway-Version`. |
| `Config::timeout` | `300s` | Default HTTP timeout for each request. |
| `Config::max_retries` | `3` | Retries `408`, `409`, `429`, `5xx`, and retryable transport failures. |
| `Config::poll_interval` | `5s` | Delay between task and workflow polls. |
| `Config::max_poll_duration` | `600s` | Max total wait time for `wait_for_output()`. |
| `RequestOptions` | none | Per-request headers, query params, timeout, retries, idempotency key, and base URL override. |
| `unstable-endpoints` | off | Enables `lip_sync`, `image_upscale`, and task list/cancel helpers. |

## Deployment / Integration

Run a live smoke test in GitHub Actions with a real Runway secret:

```yaml
name: runway-live-smoke

on:
  workflow_dispatch:

jobs:
  live:
    runs-on: ubuntu-latest
    env:
      RUNWAYML_API_SECRET: ${{ secrets.RUNWAYML_API_SECRET }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --features live-tests --test live_api -- --nocapture --test-threads=1
```

## License

[MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE).
