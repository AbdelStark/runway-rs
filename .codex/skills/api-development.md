---
name: api-development
description: Adding new Runway API resources to the SDK. Activate when creating new endpoints, resource types, or extending the client with new API capabilities. Also relevant when the Runway API adds new features.
prerequisites: Runway API documentation for the target endpoint
---

# API Development

<purpose>
Guides the full process of adding a new Runway API resource — from types to tests.
Every Runway endpoint follows the same pattern: request type → resource struct → client accessor → tests.
</purpose>

<context>
— All API types use `#[serde(rename_all = "camelCase")]` to match Runway's JSON format
— Resources return `PendingTask` for async generation endpoints, direct types for CRUD
— The client handles auth headers, retry, and rate limiting automatically
— API version header `X-Runway-Version: 2024-11-06` is set globally in client.rs
</context>

<procedure>
1. Define request/response types in `src/types/generation.rs` (or new file for new domain)
   — Required fields in `new()`, optional fields as builder methods returning `Self`
   — Use `#[serde(skip_serializing_if = "Option::is_none")]` on all optional fields
2. If new type file created: add `pub mod` + `pub use` in `src/types/mod.rs`
3. Create `src/resources/{name}.rs`:
   ```rust
   pub struct {Name}Resource {
       pub(crate) client: RunwayClient,
   }
   impl {Name}Resource {
       pub async fn create(&self, request: {Name}Request) -> Result<PendingTask, RunwayError> {
           let resp: TaskCreateResponse = self.client.post("/v1/{endpoint}", &request).await?;
           Ok(PendingTask::new(self.client.clone(), resp.id))
       }
   }
   ```
4. Add `pub mod` + `pub use` in `src/resources/mod.rs`
5. Add accessor on `RunwayClient`:
   ```rust
   pub fn {name}(&self) -> {Name}Resource {
       {Name}Resource { client: self.clone() }
   }
   ```
6. Add serialization test in `tests/types_test.rs`
7. Add wiremock test in `tests/mock_server.rs`
8. Verify: `cargo test && cargo clippy -- -D warnings && cargo fmt -- --check`
</procedure>

<patterns>
<do>
  — Follow existing naming: `{Name}Request`, `{Name}Resource`, `{Name}Response`
  — Use `impl Into<String>` for string parameters in constructors
  — Return `PendingTask` for generation endpoints, direct response types for queries
  — Match the exact Runway API endpoint path (e.g., `/v1/text_to_video`)
</do>
<dont>
  — Don't add retry logic in resources — it's handled in `client.rs`
  — Don't create separate builder structs — use the consuming-self pattern on the request type
  — Don't add `Default` impl on request types with required fields
</dont>
</patterns>

<examples>
Example: Adding a text-to-video resource (existing pattern)

```rust
// src/types/generation.rs
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
}

impl TextToVideoRequest {
    pub fn new(model: VideoModel, prompt: impl Into<String>) -> Self {
        Self { model, prompt_text: prompt.into(), duration: None }
    }
    pub fn duration(mut self, secs: u8) -> Self { self.duration = Some(secs); self }
}
```
</examples>

<troubleshooting>

| Symptom | Cause | Fix |
|---------|-------|-----|
| Compile error: "not found in resources" | Missing `pub mod`/`pub use` in `src/resources/mod.rs` | Add both lines |
| JSON field name wrong in test | Serde rename not applied | Ensure `#[serde(rename_all = "camelCase")]` on struct |
| Test returns 404 | Wiremock path doesn't match resource path | Verify mock path matches `/v1/{endpoint}` exactly |

</troubleshooting>

<references>
— src/resources/text_to_video.rs: Canonical resource implementation pattern
— src/types/generation.rs: All generation request types
— src/client.rs: HTTP helpers (post, get, delete, patch)
— tests/mock_server.rs: Integration test patterns
</references>
