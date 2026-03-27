<identity>
Unofficial Rust SDK for the Runway API — async client for AI video/image/audio generation.
</identity>

<stack>

| Layer       | Technology | Version   | Notes                                    |
|-------------|------------|-----------|------------------------------------------|
| Language    | Rust       | 2021 ed.  | Stable toolchain, no nightly features    |
| HTTP        | reqwest    | 0.12      | json, multipart, stream features         |
| Async       | tokio      | 1.x       | Full feature set                         |
| Serialization | serde   | 1.x       | derive feature, camelCase rename         |
| Errors      | thiserror  | 2.x       | Derive-based error types                 |
| Testing     | wiremock   | 0.6       | Mock HTTP server for integration tests   |
| Package mgr | cargo      | —         | Single crate, no workspace               |

</stack>

<structure>
src/
├── lib.rs              # Crate root — re-exports public API
├── client.rs           # RunwayClient — HTTP methods, shared retry (send_with_retry) & error handling (check_response)
├── config.rs           # Config builder — base URL, timeouts, poll intervals
├── error.rs            # RunwayError enum — all error variants
├── polling.rs          # PendingTask — poll-until-done and streaming status
├── types/              # Request/response types [agent: create/modify]
│   ├── mod.rs          # Re-exports all type modules
│   ├── common.rs       # Shared types (ContentModeration)
│   ├── media.rs        # MediaInput — URL, base64, file input
│   ├── models.rs       # VideoModel, ImageModel, VideoRatio enums
│   ├── task.rs         # Task, TaskStatus, TaskCreateResponse
│   ├── generation.rs   # All generation request types
│   ├── avatar.rs       # Avatar types
│   ├── document.rs     # Document types
│   ├── organization.rs # Organization types
│   ├── realtime.rs     # Realtime session types
│   ├── voice.rs        # Voice types
│   └── workflow.rs     # Workflow types
└── resources/          # API resource implementations [agent: create/modify]
    ├── mod.rs          # Re-exports all resource modules
    ├── text_to_video.rs
    ├── image_to_video.rs
    ├── video_to_video.rs
    ├── text_to_image.rs
    ├── text_to_speech.rs
    ├── speech_to_speech.rs
    ├── sound_effect.rs
    ├── character_performance.rs
    ├── voice_dubbing.rs
    ├── voice_isolation.rs
    ├── uploads.rs
    ├── tasks.rs
    ├── avatars.rs
    ├── documents.rs
    ├── voices.rs
    ├── workflows.rs
    ├── workflow_invocations.rs
    ├── realtime_sessions.rs
    └── organization.rs
tests/
├── types_test.rs       # Unit tests — serialization/deserialization
└── mock_server.rs      # Integration tests — wiremock-based API simulation
examples/               # Usage examples (require RUNWAYML_API_SECRET) [agent: create/modify]
</structure>

<commands>

| Task             | Command                              | Notes                           |
|------------------|--------------------------------------|---------------------------------|
| Build            | `cargo build`                        | ~5s clean, <1s incremental      |
| Test (all)       | `cargo test`                         | Unit + integration + doctests   |
| Test (specific)  | `cargo test test_name`               | Filter by test name             |
| Check            | `cargo check`                        | Type check without codegen      |
| Clippy           | `cargo clippy -- -D warnings`        | Lint — treat warnings as errors |
| Format           | `cargo fmt`                          | Rustfmt                         |
| Format (check)   | `cargo fmt -- --check`               | Verify formatting only          |
| Run example      | `cargo run --example text_to_video`  | Requires RUNWAYML_API_SECRET    |
| Doc              | `cargo doc --no-deps --open`         | Generate and open API docs      |

</commands>

<conventions>
<code_style>
  — Naming: snake_case for functions/variables, PascalCase for types/enums, SCREAMING_SNAKE for constants
  — Files: snake_case.rs everywhere
  — Serde: `#[serde(rename_all = "camelCase")]` on all API types — Runway API uses camelCase JSON
  — Optional fields: `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
  — Builder pattern: consuming self (`mut self`) for request builders, returning `Self`
  — Visibility: `pub(crate)` for internal helpers, `pub` for API surface
  — Imports: group std → external crates → crate-internal (`use crate::...`)
</code_style>

<patterns>
<do>
  — Use builder pattern for all request types: `Request::new(required).optional_field(val)`
  — Return `PendingTask` from resource `.create()` methods — caller decides to poll or not
  — Use `thiserror` derive for all error variants in `RunwayError`
  — Add new API endpoints as: type in `types/`, resource in `resources/`, accessor on `RunwayClient`
  — Use wiremock for testing HTTP interactions — never hit real API in tests
  — Keep resource structs minimal: just `client: RunwayClient` field
  — Serialize enums with serde rename attributes matching exact Runway API strings
</do>
<dont>
  — Don't use `unwrap()` in library code — return `RunwayError` instead
  — Don't add `async-trait` — use concrete types or `impl Trait` returns
  — Don't duplicate retry/rate-limit logic — it lives in `client.rs` HTTP helpers
  — Don't put business logic in resource files — they should only map to API endpoints
  — Don't use `println!` — use `tracing::debug!`/`tracing::warn!` for diagnostics
</dont>
</patterns>

<commit_conventions>
  Format: `type: description` — e.g. `feat: add lip sync resource`, `fix: handle 504 in polling`
  Types: feat, fix, refactor, test, docs, chore
</commit_conventions>
</conventions>

<workflows>
<new_resource>
  1. Add request/response types to `src/types/generation.rs` (or new type file if new domain)
  2. If new type file: add `pub mod` + `pub use` in `src/types/mod.rs`
  3. Create `src/resources/{resource_name}.rs` with struct + `create`/`list`/`get` methods
  4. Add `pub mod` + `pub use` in `src/resources/mod.rs`
  5. Add accessor method on `RunwayClient` in `src/client.rs`
  6. Add serialization tests in `tests/types_test.rs`
  7. Add wiremock integration test in `tests/mock_server.rs`
  8. Run `cargo test` — all must pass
  9. Run `cargo clippy -- -D warnings` — zero warnings
  10. Run `cargo fmt -- --check` — must be clean
</new_resource>

<bug_fix>
  1. Reproduce with a test (wiremock or unit)
  2. Fix the issue
  3. Verify test passes
  4. Run full `cargo test`
  5. Run `cargo clippy -- -D warnings`
</bug_fix>
</workflows>

<boundaries>
<forbidden>
  DO NOT modify:
  — .env, .env.* (API keys, secrets)
  — Cargo.lock (modified automatically by cargo — never hand-edit)
  — LICENSE-MIT, LICENSE-APACHE
</forbidden>

<gated>
  Modify ONLY with explicit approval:
  — Cargo.toml [dependencies] section (adding/removing/upgrading dependencies)
  — src/client.rs HTTP methods (shared retry/auth logic — changes affect all resources)
  — src/error.rs (adding error variants changes public API)
  — Public API surface (any `pub` item removal or signature change)
</gated>
</boundaries>

<troubleshooting>

| Symptom                                 | Cause                              | Fix                                    |
|-----------------------------------------|------------------------------------|----------------------------------------|
| `MissingApiKey` at runtime              | RUNWAYML_API_SECRET not set        | Export env var or use `with_api_key()`  |
| Serde deserialization failure           | API field name mismatch            | Check `rename_all` and field names match Runway API docs |
| Test hangs                              | Missing mock for polled endpoint   | Add wiremock `Mock::given` for `/v1/tasks/{id}` |
| `cargo test` compile error on new type  | Missing `pub mod` / `pub use`      | Update `mod.rs` in types/ and resources/ |

</troubleshooting>

<environment>
  — Harness: Claude Code
  — API version: 2024-11-06 (set in config.rs DEFAULT_API_VERSION)
  — Base URL: https://api.dev.runwayml.com
  — CI: GitHub Actions (.github/workflows/ci.yml) — check, test, clippy, fmt, doc
  — Dual license: MIT OR Apache-2.0
</environment>

<skills>
Modular skills in `.codex/skills/` (symlinked from `.claude/skills/` and `.agents/skills/`).

Available skills:
— api-development.md: Adding new Runway API resources end-to-end
— testing.md: Writing unit and integration tests with wiremock
— debugging.md: Diagnosing serialization, HTTP, and polling issues
</skills>

<decisions>
  2024-01-01 reqwest over hyper — Higher-level API, built-in JSON/multipart, less boilerplate — hyper (too low-level for SDK)
  2024-01-01 thiserror over anyhow — Library needs typed errors for consumers — anyhow (erases types)
  2024-01-01 Builder pattern (consuming self) — Ergonomic chaining, no &mut borrow issues — &mut self (less ergonomic), separate builder struct (over-engineered)
  2024-01-01 PendingTask abstraction — Decouples creation from polling, supports both wait and stream — Returning Task directly (forces polling in create)
  2024-01-01 wiremock over mockito — Better async support, more expressive matchers — mockito (sync-first), httpmock (less mature)
</decisions>
