# Production readiness assessment

Date: 2026-07-16

Target: `runway-sdk` 0.2.0

Upstream baseline: official Node SDK 4.10.0 and Python SDK 5.10.0

## Verdict

The 0.2.0 codebase is a production-oriented community SDK release candidate.
Its stable operation and model surface is source-aligned with the pinned
official SDKs, and its runtime deliberately takes a more conservative position
on retries of billable mutations.

This is not an official Runway SDK, and deterministic contract tests are not a
substitute for a successful request from a funded live account. The exact
evidence and that boundary are documented in
[Compatibility](compatibility.md).

## Release gates

| Area | 0.2.0 requirement |
| --- | --- |
| API contract | All 52 pinned official operations and every current model variant have exact wire tests. |
| Invalid input | Model-specific local constraints and pagination limits fail before network I/O where the client has enough information. |
| Billing safety | POST/PATCH retries require an idempotency key or an explicit unsafe opt-in. |
| Transport | Retry headers, connection failures, cancellation, bounded bodies, response metadata, and typed errors are covered by mock-server tests. |
| Polling | One absolute deadline covers jitter, requests, response bodies, retry backoff, and later sleeps for both tasks and workflows. |
| Uploads | Files stream from disk; presigned storage requests never inherit Runway authorization headers. |
| Secrets | Client configuration, request options, response metadata, realtime credentials, error headers, queries, and decode excerpts have redacting debug output; transport errors strip request URLs. |
| Portability | CI checks stable Rust on Linux, macOS, and Windows and tests the declared MSRV separately. |
| Supply chain | `cargo audit`, `cargo deny`, locked builds, immutable action pins, and package-content checks are release gates. |
| Documentation | README, rustdoc examples, compatibility snapshot, changelog, contribution guide, security policy, and release procedure agree on the public contract. |

## Residual risks

- Runway can change a live API or model rollout after the pinned upstream
  snapshot. A release must not claim compatibility with later SDK versions
  without a new diff and contract tests.
- Some constraints require inspecting remote media, including dimensions,
  duration, face visibility, or moderation results. Those remain server-side.
- A valid live account can still reject a source-correct request because of
  tier, credits, regional availability, policy, or service state.
- Idempotency support is service-defined. Callers must generate one unique key
  per logical mutation and must not reuse a key for unrelated requests.
- Compatibility aliases for removed upstream models ease migration but do not
  imply that the service still accepts those models.

## Release decision

Publish only from a reviewed, tagged commit after the local and hosted gates in
[CONTRIBUTING.md](../CONTRIBUTING.md) pass. The optional live workflow must be
run deliberately with a maintainer-owned credential when a live release smoke
test is required; it is never part of ordinary pull-request CI because it can
incur charges.
