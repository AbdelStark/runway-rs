# Production parity specification

Status: Implemented for the 0.2.0 release candidate

Last updated: 2026-07-16

## Objective

Provide an idiomatic Rust client whose stable wire contract covers the current
official Runway SDK surface, while making billing, cancellation, secret
handling, and compatibility boundaries explicit.

The pinned evidence and complete model/operation matrices live in
[Compatibility](../compatibility.md). The current release decision is recorded
in [Production readiness](../production-gap-analysis.md).

## Contract rules

1. Official generated SDK source controls endpoint paths, methods, JSON/query
   names, optionality, discriminants, and response state unions.
2. Each model with a distinct legal field set has a distinct Rust request type.
3. Dynamic identifiers are path-segment encoded. Cursor limits and model rules
   are rejected locally when the client has enough information.
4. Every stable resource offers a convenience method and a `_with_options`
   form returning parsed data with HTTP metadata.
5. Cursor resources provide typed pages and stream helpers. Long-running work
   returns a pending handle with bounded, cancellable polling.
6. Community-only endpoints remain behind `unstable-endpoints` and are excluded
   from official parity counts.

## Runtime rules

- Default request timeout: 60 seconds.
- Default retry budget: two retries.
- Default poll interval: six seconds with bounded ±25% jitter.
- Default absolute polling deadline: ten minutes.
- GET and DELETE may retry retryable transport/status failures.
- POST and PATCH require an idempotency key or explicit unsafe opt-in before
  automatic retries; server retry headers never override that safety rule.
- Cancellation covers request execution, response reads, retry waits, and every
  polling phase.
- JSON and error bodies have hard size limits.
- Presigned storage requests use a dedicated client without API credentials.
- Debug representations of credential-bearing runtime values redact secrets by
  default; explicit accessors are required for retained diagnostic bodies.

## Verification rules

Every contract change needs deterministic evidence for:

- serialized bodies and queries, including omitted optional fields;
- HTTP method and exact path;
- response-state deserialization;
- local invalid-input rejection before I/O;
- request options and response metadata;
- retry eligibility, deadlines, cancellation, or upload isolation when the
  runtime behavior changes.

The release gate runs formatting, all targets/features, Clippy with warnings
denied, doc tests, MSRV tests, docs.rs-style docs, advisory/license policy, and
crate packaging. Live tests are ignored by default because they require a real
credential and can create billable work.

## Compatibility policy

Before 1.0, source-breaking changes require a minor-version bump and migration
notes. Removed upstream models may receive a documented legacy compatibility
shim, but stable constructors and documentation must prefer the current model
set.
No release may claim a newer upstream baseline until its resources and unions
have been diffed and the compatibility snapshot updated.
