---
name: testing
description: Writing and running tests for the Runway SDK. Activate when adding tests, debugging test failures, or setting up mocks. Covers both unit tests (serde) and integration tests (wiremock HTTP mocking).
prerequisites: wiremock 0.6, tokio-test, assert_matches
---

# Testing

<purpose>
Guides writing unit and integration tests for the SDK.
Two test files: `tests/types_test.rs` (serialization) and `tests/mock_server.rs` (HTTP).
</purpose>

<context>
— Unit tests verify serde serialization/deserialization of API types
— Integration tests use wiremock to simulate the Runway API
— `test_config()` helper in mock_server.rs creates a Config pointing to the mock server
— Tests use short poll intervals (100ms) to avoid slow test runs
— Live API tests are behind the `live-tests` feature flag (not run in CI)
</context>

<procedure>
1. For new types: add serialization test in `tests/types_test.rs`
   — Test `serde_json::to_value` for requests (verify camelCase field names)
   — Test `serde_json::from_str` for responses (verify deserialization)
2. For new resources: add wiremock test in `tests/mock_server.rs`
   — Set up `MockServer::start().await`
   — Mount mock with `Mock::given(method("POST")).and(path("/v1/..."))...`
   — Create client with `test_config(&mock_server.uri())`
   — Call the resource method and assert the result
3. Run: `cargo test`
4. For a specific test: `cargo test test_name`
</procedure>

<patterns>
<do>
  — Use `serde_json::to_value` (not `to_string`) for readable assertions on request JSON
  — Use `serde_json::from_str` with raw JSON strings for response deserialization tests
  — Set `.expect(1)` on mocks to verify the endpoint was called exactly once
  — Use short timeouts in test configs: `poll_interval(Duration::from_millis(100))`
  — Test error paths: 401 → Unauthorized, 400 → Api error, 429 → rate limiting
</do>
<dont>
  — Don't hit the real Runway API in standard tests — use wiremock
  — Don't use `#[ignore]` without the `live-tests` feature gate
  — Don't hardcode UUIDs differently across related tests — reuse the same UUID string for consistency
</dont>
</patterns>

<examples>
Example: Wiremock integration test

```rust
#[tokio::test]
async fn test_resource_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/endpoint"))
        .and(header("Authorization", "Bearer test-api-key-12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));
    let pending = client.resource().create(Request::new(...)).await.unwrap();
    assert_eq!(pending.id().to_string(), "550e8400-e29b-41d4-a716-446655440000");
}
```
</examples>

<troubleshooting>

| Symptom | Cause | Fix |
|---------|-------|-----|
| Test hangs indefinitely | Polling loop with no matching mock | Add mock for `GET /v1/tasks/{id}` returning terminal status |
| `Unexpected request` from wiremock | Mock path/method doesn't match | Check exact path, method, and headers |
| Deserialization panic in test | JSON field names don't match serde renames | Compare JSON keys with `#[serde(rename_all)]` output |

</troubleshooting>

<references>
— tests/types_test.rs: Unit test examples
— tests/mock_server.rs: Integration test examples with test_config helper
— src/types/task.rs: Task/TaskStatus types used in polling tests
</references>
