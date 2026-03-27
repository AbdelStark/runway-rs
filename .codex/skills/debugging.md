---
name: debugging
description: Diagnosing and fixing issues in the Runway SDK. Activate when encountering errors, unexpected behavior, serialization mismatches, HTTP failures, or polling problems.
prerequisites: None
---

# Debugging

<purpose>
Systematic approach to diagnosing SDK issues. Most bugs fall into three categories:
serde mismatches, HTTP/auth errors, or polling logic problems.
</purpose>

<context>
— Error types are in `src/error.rs` — `RunwayError` enum covers all failure modes
— HTTP retry logic is in `client.rs` — handles 429 (rate limit) with exponential backoff
— Polling logic is in `polling.rs` — `PendingTask::wait_for_output` and `stream_status`
— All API types use `#[serde(rename_all = "camelCase")]`
</context>

<procedure>
1. Identify the error category:
   — Compile error → Check mod.rs re-exports, type imports, feature flags
   — Serde error → Compare Rust struct fields with actual API JSON (use tracing to log raw responses)
   — HTTP error → Check status code in RunwayError::Api, verify auth header
   — Polling hang → Check task status mock returns terminal state (Succeeded/Failed)
2. For serde issues:
   — Serialize the Rust type with `serde_json::to_value` and compare with expected JSON
   — Check `rename_all`, individual `#[serde(rename)]` attributes, `skip_serializing_if`
   — Verify enum variant serialization matches API strings exactly
3. For HTTP issues:
   — Enable tracing: `RUST_LOG=runway_sdk=debug cargo test`
   — Check if rate limit retry is exhausting max_retries
   — Verify base URL doesn't have trailing slash
4. For polling issues:
   — Check `max_poll_duration` isn't too short
   — Verify task status strings match `TaskStatus` enum (`SUCCEEDED`, `FAILED`, etc.)
   — Check the 2-second initial delay in polling.rs
5. Write a regression test reproducing the bug before fixing
</procedure>

<patterns>
<do>
  — Use `RUST_LOG=runway_sdk=debug` to enable tracing output during debugging
  — Write a failing test first, then fix
  — Check the Runway API docs for the exact JSON format when serde fails
</do>
<dont>
  — Don't add `println!` for debugging — use `tracing::debug!` which is already instrumented
  — Don't change error variants without considering downstream consumers
  — Don't increase timeouts to "fix" polling issues — find the root cause
</dont>
</patterns>

<troubleshooting>

| Symptom | Cause | Fix |
|---------|-------|-----|
| `RunwayError::Unauthorized` | Invalid or expired API key | Verify RUNWAYML_API_SECRET is correct |
| `RunwayError::RateLimited` even after retries | Exceeded max_retries (default 3) | Increase `Config::max_retries` or add backoff |
| `RunwayError::Timeout` during polling | Task taking longer than max_poll_duration | Increase via `Config::max_poll_duration` |
| Serde: "missing field" | API response has different structure than type | Add `Option<T>` for nullable fields, check rename |
| Serde: "unknown variant" | API returned status/enum value not in enum | Add new variant to enum with correct serde rename |

</troubleshooting>

<references>
— src/error.rs: All error variants
— src/client.rs:191-228: Retry logic for POST requests
— src/polling.rs:29-81: Polling loop with timeout
— src/config.rs: Default timeout and retry values
</references>
