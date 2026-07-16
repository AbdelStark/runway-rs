# runway-sdk

An unofficial, async Rust SDK for the Runway API. It provides model-specific
request types, all current public API operations, streaming uploads, bounded
and cancellable polling, per-request controls, and structured errors.

[![crates.io](https://img.shields.io/crates/v/runway-sdk.svg)](https://crates.io/crates/runway-sdk)
[![docs.rs](https://img.shields.io/docsrs/runway-sdk)](https://docs.rs/runway-sdk)
[![CI](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AbdelStark/runway-rs/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.89-blue)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/runway-sdk)](LICENSE-MIT)

> [!IMPORTANT]
> `runway-sdk` is community maintained. It is not affiliated with or endorsed
> by Runway AI, Inc. Use [Runway's official documentation](https://docs.dev.runwayml.com/)
> for service availability, pricing, and policy.

## Why this crate

- Typed, model-specific request variants reject many invalid combinations
  before a billable request is sent.
- The stable resource surface tracks all 52 operations in the official SDK
  snapshot documented in [Compatibility](docs/compatibility.md).
- POST and PATCH requests are not retried by default, avoiding accidental
  duplicate generation charges after an ambiguous failure.
- Polling has one absolute deadline across sleeps, HTTP requests, response
  bodies, and retry backoff, with cancellation through `CancellationToken`.
- File uploads stream from disk and use a separate unauthenticated HTTP client
  for presigned storage URLs.
- Response bodies are bounded, errors retain status and headers, and common
  debug output redacts credentials, header values, query values, and signed
  URL details.

## Quick start

Add the crate and Tokio:

```console
cargo add runway-sdk
cargo add tokio --features macros,rt-multi-thread
```

Set your API secret:

```console
export RUNWAYML_API_SECRET=your_secret_here
```

Start with a read-only account check:

```rust
use runway_sdk::RunwayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;
    let organization = client.organization().retrieve().await?;

    println!("credit balance: {}", organization.credit_balance);
    Ok(())
}
```

### Generate a video safely

Generation creates billable work. Give each logical submission a unique
idempotency key if you want transport retries:

```rust
use std::time::Duration;

use runway_sdk::{
    RequestOptions, RunwayClient, TextToVideoGen45Request, VideoRatio, WaitOptions,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;
    let cancellation = CancellationToken::new();

    let submitted = client
        .text_to_video()
        .create_with_options(
            TextToVideoGen45Request::new(
                "A slow aerial shot over a glacier at sunrise",
                VideoRatio::Landscape,
                5,
            ),
            RequestOptions::new().idempotency_key("my-unique-generation-id"),
        )
        .await?;

    let task = submitted
        .data
        .wait_with_options(
            WaitOptions::new()
                .timeout(Duration::from_secs(15 * 60))
                .cancellation_token(cancellation),
        )
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("{url}");
    }
    Ok(())
}
```

Methods ending in `_with_options` return `WithResponse<T>`, pairing parsed data
with the HTTP status and response headers. Convenience methods return `T`
directly.

### Stream an upload from disk

```rust
use std::path::Path;

use runway_sdk::RunwayClient;

# async fn upload() -> Result<(), Box<dyn std::error::Error>> {
let client = RunwayClient::new()?;
let runway_uri = client.uploads().upload_file(Path::new("input.png")).await?;
println!("{runway_uri}");
# Ok(())
# }
```

The SDK creates an official ephemeral upload placeholder, sends a multipart
form to the returned storage URL without Runway authorization headers, and
returns the server-provided `runway://` URI.

## API surface

All resources are created from `RunwayClient`; resource structs do not need to
be constructed directly.

| Resource | Stable operations |
| --- | --- |
| `tasks()` | `retrieve`, `delete` |
| `text_to_video()` | `create` |
| `image_to_video()` | `create` |
| `video_to_video()` | `create` |
| `text_to_image()` | `create` |
| `image_upscale()` / `video_upscale()` | `create` |
| `character_performance()` | `create` |
| `text_to_speech()` / `speech_to_speech()` | `create` |
| `sound_effect()` / `voice_isolation()` / `voice_dubbing()` | `create` |
| `recipes()` | seven typed production-recipe methods |
| `organization()` | `retrieve`, `retrieve_usage` |
| `avatars()` | `create`, `retrieve`, `update`, `list`, `delete`, `get_usage` |
| `avatar_conversations()` | `retrieve`, `list`, `delete` |
| `avatar_videos()` | `create` |
| `documents()` | `create`, `retrieve`, `update`, `list`, `delete` |
| `realtime_sessions()` | `create`, `retrieve`, `cancel` |
| `voices()` | `create`, `retrieve`, `update`, `preview`, `list`, `delete` |
| `workflows()` | `retrieve`, `list`, `run` |
| `workflow_invocations()` | `retrieve` |
| `uploads()` | `create_ephemeral` plus streaming file helpers |

Cursor resources also expose page and item streams. Task and workflow creation
returns pending handles with `wait_for_output`, explicit `WaitOptions`, and
status streams.

## Runtime behavior

| Setting | Default | Notes |
| --- | ---: | --- |
| Base URL | `https://api.dev.runwayml.com` | Must be an absolute HTTP(S) URL without credentials, query, or fragment. |
| API version | `2024-11-06` | Sent as `X-Runway-Version`. |
| Request timeout | 60 seconds | Applies to connect, response headers, and body reads. |
| Retry budget | 2 retries | Used only when the request is eligible for automatic retry. |
| Poll interval | 6 seconds | Each interval is jittered by up to 25%. |
| Poll deadline | 10 minutes | One deadline covers every polling phase. |
| JSON response limit | 16 MiB | Oversized bodies return `ResponseTooLarge`. |
| Error response limit | 1 MiB | Prevents unbounded diagnostic buffering. |

GET and DELETE requests automatically retry retryable connection failures,
timeouts, HTTP 408, 409, 429, and 5xx responses. POST and PATCH requests require
an idempotency key or an explicit `retry_non_idempotent(true)` opt-in; the latter
can duplicate billable work. `x-should-retry` and `Retry-After` headers are
honored within that eligibility rule; a server header never overrides the
mutation-safety requirement. Retry waits are cancellable.

Configure client-wide defaults with `Config` and override a single call with
`RequestOptions`:

```rust
use std::time::Duration;

use runway_sdk::{Config, RequestOptions, RunwayClient};

# fn configure() -> Result<(), Box<dyn std::error::Error>> {
let client = RunwayClient::with_config(
    Config::new("api-secret")
        .timeout(Duration::from_secs(90))
        .max_retries(4),
)?;

let request = RequestOptions::new()
    .timeout(Duration::from_secs(30))
    .max_retries(0);
# let _ = (client, request);
# Ok(())
# }
```

## Errors and diagnostics

Every fallible method returns `RunwayError`. API errors expose their HTTP
status, classified `ApiErrorKind`, parsed code/message, response headers, and
rate-limit delay when available. Transport timeouts, cancellation, validation,
task failures, workflow failures, malformed JSON, and oversized responses have
distinct variants.

Malformed-response excerpts are redacted from `Debug`; access one deliberately
with `ResponseBodyExcerpt::expose()`. `Config`, `RequestOptions`, response
metadata, realtime credentials, and error headers also use redacting debug
implementations. Transport errors strip request URLs before storage so secret
query values and presigned storage credentials cannot leak through error
formatting. Applications should still avoid logging generated content or API
error display strings unless their own data-handling policy permits it.

## Compatibility and versioning

Version 0.2.0 is source-aligned with the official Node SDK 4.10.0 and Python
SDK 5.10.0 snapshots from 2026-07-16. The exact commits, model matrix, known
server-side constraints, and verification boundary are recorded in
[docs/compatibility.md](docs/compatibility.md).

This crate follows semantic versioning. Before 1.0, minor releases may contain
intentional public API changes. Applications upgrading from 0.1.x should follow
the [0.2.0 migration guide](docs/migration-0.2.md); all release notes remain in
[CHANGELOG.md](CHANGELOG.md).

## Feature flags

| Feature | Purpose |
| --- | --- |
| `unstable-endpoints` | Enables community extensions not present in the pinned official SDK: lip sync and task list/cancel helpers. |
| `live-tests` | Compiles the ignored real-account smoke tests. It does not run them automatically. |

Live tests can consume credentials and incur charges. They are excluded from
normal CI and require an explicit manual confirmation in the dedicated GitHub
Actions workflow. To run them locally with your own account:

```console
cargo test --all-features --test live_api -- --ignored --test-threads=1 --nocapture
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for the MSRV, full local quality gate,
contract-update policy, and release procedure. Security issues belong in a
[private advisory](SECURITY.md), not a public issue.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at
your option.
