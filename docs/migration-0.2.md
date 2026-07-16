# Migrating from 0.1.x to 0.2.0

Version 0.2.0 is an intentionally source-breaking pre-1.0 release. It aligns
the public Rust types and wire payloads with the upstream SDK snapshot recorded
in the [compatibility contract](compatibility.md).

This guide covers the changes most likely to require application edits. The
release is backed by deterministic contract and mock-server tests. No
successful billable generation against the live API is claimed by this audit.

## Upgrade the toolchain and dependency

The minimum supported Rust version is now 1.89. First make the direct dependency
requirement explicit:

```toml
[dependencies]
runway-sdk = "0.2"
```

Then install the tested minimum toolchain and resolve the new crate version:

```console
rustup toolchain install 1.89.0
cargo +1.89.0 update -p runway-sdk --precise 0.2.0
cargo +1.89.0 check --all-targets --all-features
```

## Review changed runtime defaults

Three client defaults changed to match the current upstream SDKs:

| Setting | 0.1.x | 0.2.0 |
| --- | ---: | ---: |
| HTTP request timeout | 300 seconds | 60 seconds |
| Retry budget | 3 retries | 2 retries |
| Poll interval | 5 seconds | 6 seconds |

The ten-minute polling deadline is unchanged. Poll waits are now jittered by up
to 25%, and one absolute deadline covers sleeps, requests, response bodies, and
retry backoff.

If the old timing values are required, set them explicitly:

```rust
use std::time::Duration;
use runway_sdk::Config;

let config = Config::new("api-secret")
    .timeout(Duration::from_secs(300))
    .max_retries(3)
    .poll_interval(Duration::from_secs(5));
```

This restores the numeric values, not the old mutation retry behavior described
below.

## Construct configuration with the API key

`Config::api_key` is now private so application debug and configuration code
cannot read or replace the credential accidentally.

Replace direct field access:

```rust,ignore
// 0.1.x
let mut config = Config::new(old_key);
config.api_key = rotated_key;
```

with construction of a new configuration or client:

```rust
use runway_sdk::{Config, RunwayClient};

fn client_for(rotated_key: String) -> Result<RunwayClient, runway_sdk::RunwayError> {
    RunwayClient::with_config(Config::new(rotated_key))
}
```

There is intentionally no public API-key getter. Keep the source secret in the
application's secret store and create a new client when rotating it.

## Opt mutation requests into retries deliberately

In 0.1.x, the configured retry budget could retry POST and PATCH requests.
Version 0.2.0 automatically retries eligible GET and DELETE requests, but a POST
or PATCH is retryable only when either:

- the request has an idempotency key; or
- the application explicitly opts into non-idempotent retries.

A retry budget alone no longer makes a mutation retryable. Prefer a unique,
stable key for each logical submission:

```rust,ignore
use runway_sdk::RequestOptions;

let options = RequestOptions::new()
    .max_retries(2)
    .idempotency_key("generation-42");

let submitted = client
    .text_to_video()
    .create_with_options(request, options)
    .await?;
```

If an endpoint cannot use an idempotency key, opt in only after accepting that
an ambiguous failure may duplicate billable work:

```rust
let options = runway_sdk::RequestOptions::new()
    .retry_non_idempotent(true);
```

The same unsafe policy can be set client-wide with
`Config::retry_non_idempotent(true)`, but per-request scope is easier to audit.
`RunwayClient::with_options` rejects idempotency keys and cancellation tokens:
those controls belong to one logical request and must not become reusable
client defaults.

## Update error pattern matches

`RunwayError::Unauthorized` changed from a unit variant to a structured
variant that retains the server message, optional code, and redacted response
headers.

```rust,ignore
use runway_sdk::RunwayError;

match error {
    RunwayError::Unauthorized { message, code, .. } => {
        eprintln!("authentication failed: {message} ({code:?})");
    }
    other => eprintln!("{other}"),
}
```

Code that destructures transport errors must also use
`ConnectionError { source }` and `ConnectionTimeout { source }` instead of
the old message/unit forms. For API-derived errors, the stable accessors
`status()`, `api_kind()`, and `headers()` avoid coupling to variant fields.

## Supply document sort semantics

`DocumentListQuery` is no longer the generic zero-argument
`CursorPageQuery` alias. The current endpoint requires both `order` and
`sort`.

For the common case, replace:

```rust,ignore
// 0.1.x
let query = DocumentListQuery::new().limit(25);
```

with:

```rust
use runway_sdk::DocumentListQuery;

let query = DocumentListQuery::newest_first().limit(25);
```

For an explicit ordering:

```rust
use runway_sdk::{DocumentListQuery, DocumentSortField, DocumentSortOrder};

let query = DocumentListQuery::new(
    DocumentSortOrder::Asc,
    DocumentSortField::UpdatedAt,
)
.limit(25);
```

## Move to the current typed API surface

Version 0.2.0 models upstream discriminated unions directly. Prefer the
model-specific request builders and enums over hand-built JSON or free-form
model strings. The complete supported model list is maintained in the
[compatibility matrix](compatibility.md#current-model-matrix).

### Realtime sessions

Preset avatar IDs are typed. Replace a string preset:

```rust,ignore
// 0.1.x
RealtimeAvatarInput::runway_preset("game-character")
```

with the corresponding enum variant:

```rust
use runway_sdk::{RealtimeAvatarInput, RealtimePresetAvatarId};

let avatar =
    RealtimeAvatarInput::runway_preset(RealtimePresetAvatarId::GameCharacter);
```

Realtime integrations and tool schemas now use `RealtimeIntegration`,
`RealtimeTool`, and `RealtimeToolParameter`. Use their constructors so the
serialized discriminator, credentials, and parameter schema match the current
contract. Custom avatar IDs remain strings through
`RealtimeAvatarInput::custom(...)`.

### Image upscaling

The obsolete feature-gated `ImageUpscaleRequest` payload used an image
generation model, `promptImage`, pixel `resolution`, seed, and moderation
fields. The stable endpoint now accepts `ImageUpscaleCreateRequest` with the
Magnific model discriminator, `imageUri`, flavor, scale factor, and enhancement
controls.

Replace:

```rust,ignore
// 0.1.x
ImageUpscaleRequest::new(
    ImageModel::Gen4ImageTurbo,
    MediaInput::from_url("https://example.com/input.png"),
)
.resolution(4096)
```

with:

```rust
use runway_sdk::{
    ImageUpscaleCreateRequest, ImageUpscaleFlavor, ImageUpscaleScaleFactor,
};

let request = ImageUpscaleCreateRequest::new("https://example.com/input.png")
    .flavor(ImageUpscaleFlavor::Photo)
    .scale_factor(ImageUpscaleScaleFactor::X4);
```

Image upscaling no longer requires `unstable-endpoints`. Video upscaling is
available through `VideoUpscaleCreateRequest` and `client.video_upscale()`.

### Generation and management models

The current surface adds dedicated request variants for the pinned video,
image, and audio models, including Seedance, Gemini, GPT Image, Seedream, and
Seed Audio families. Existing request types still convert into the new
`*CreateRequest` unions where the upstream shape remains valid; use the
compiler error and the compatibility matrix to select a replacement for an
obsolete model or field.

Management coverage now includes avatar usage, avatar conversations and videos,
voice updates, seven recipe operations, and page/item streams for cursor
resources. Response types follow the current upstream discriminated unions.
When code directly constructs or exhaustively destructures management payloads,
switch to the exported constructors, accessors, and enum variants rather than
assuming the 0.1.x JSON shape.

Voice creation now models `from` as the tagged
[`VoiceFrom`](https://docs.rs/runway-sdk/0.2.0/runway_sdk/enum.VoiceFrom.html)
audio-or-text union. `CreateVoiceRequest::from_audio` represents cloning from
an audio URL, while `CreateVoiceRequest::new` remains the text-design
convenience constructor and now defaults to the upstream-preferred
`eleven_ttv_v3`. Text design and preview prompts are rejected locally when
shorter than 20 characters. Voice-create descriptions are tri-state: omit them
by default, set them with `description`, or emit JSON `null` with
`clear_description`.

## Verify the migration

Run deterministic checks before any live request:

```console
cargo fmt --all --check
cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
```

The `live-tests` feature only compiles ignored real-account tests. Running
those tests explicitly can consume credentials, storage, and credits. Consult
the [compatibility contract](compatibility.md#verification-boundary) for what
the normal release gate proves and the [changelog](../CHANGELOG.md) for the full
0.2.0 release summary.
