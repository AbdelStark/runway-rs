# Runway Rust SDK Production Parity Specification

Status: Draft
Date: 2026-03-27
Owner: `runway-rs`
Related: [../production-gap-analysis.md](../production-gap-analysis.md)

## Problem

The current crate has broad endpoint intent coverage but does not match the official Runway SDK contract closely enough to be released as a production-grade equivalent SDK.

The current failures are structural:

- request and response schemas diverge from the official contract
- uploads use the wrong storage handoff flow
- several resources model thin objects where the official SDK models tagged unions
- runtime behavior is missing per-request overrides, raw-response access, cursor pagination, workflow polling, richer retries, and typed HTTP errors
- unofficial endpoints are mixed into the same surface without clear stability boundaries

## Goal

Ship a Rust SDK whose stable public API is contract-faithful to the current official Runway SDK, production-safe in transport behavior, and explicit about anything unofficial or experimental.

## Non-Goals

- Preserve source compatibility with `0.1.0` at all costs
- Add new speculative endpoints beyond the official SDK snapshot
- Guarantee browser support
- Replace the whole crate architecture unless required for parity

## Source Of Truth

Use sources in this order:

1. Official Runway OpenAPI or Stainless source, if obtainable
2. Official Node SDK source in `/tmp/runway-sdk-node-fgCeMJ`
3. Official Runway API docs
4. Real server behavior from live integration tests
5. The current Rust crate only for migration planning, never as the canonical contract

If two sources disagree, implementation must document the discrepancy in the PR and prefer the highest source in the list unless live behavior proves otherwise.

## Product Decisions

### 1. Stable Surface

The stable surface of the crate must correspond to the official SDK contract.

- resource accessor names may remain idiomatic Rust snake_case
- resource method names should align with official semantics:
  `create`, `retrieve`, `list`, `update`, `delete`, `preview`, `run`
- methods currently named `get` or `usage` should either be renamed or kept as deprecated aliases during the migration window

### 2. Unofficial Surface

Endpoints or options that are not present in the official SDK snapshot must not remain mixed into the stable parity layer.

They must be moved into one of these buckets:

- `experimental` module
- `extensions` module
- feature-gated API such as `unstable-endpoints`

This applies to:

- `lip_sync`
- `image_upscale`
- `tasks.list`
- `tasks.cancel`
- request-level `webhook_url` fields, unless live API evidence and docs confirm they are official and stable

### 3. Breaking Changes

Breaking changes are acceptable before `1.0`, but they must be deliberate and documented.

- every removed or renamed public item must be called out in the changelog
- compatibility shims may be added if they do not distort the new stable design
- deprecated aliases are acceptable for one release cycle if they reduce migration pain

## Required Architecture

### Contract Layer

The crate needs a spec-faithful contract layer first, with ergonomics layered on top.

Requirements:

- tagged unions must be represented as Rust enums or equivalent typed state models
- cursor-paginated resources must use typed page containers
- model-specific generation requests must be represented explicitly, not flattened into one permissive struct
- field names and optionality must match the official contract exactly

Recommended file layout:

- split the current monolithic [generation.rs](/Users/abdel/dev/me/world-models/runway/runway-rs/src/types/generation.rs) into resource-scoped type files
- keep resource modules thin and move schema complexity into `types/`
- add dedicated modules for request options, pagination, and response metadata

### Runtime Layer

The runtime layer must support official-equivalent request behavior.

Requirements:

- per-request headers, query, timeout, retries, idempotency key, and base URL override
- raw response access in addition to parsed bodies
- richer retry policy
- typed HTTP errors
- task polling and workflow polling with cancellation support

### Testing Layer

The verification model must expand beyond the current mock-only schema coverage.

Requirements:

- fixture-driven serialization and deserialization tests
- transport behavior tests for retries and uploads
- live gated tests against a real Runway account
- examples and doc tests aligned to the new stable surface

## Workstreams

## Workstream 1: Public API And Naming Alignment

### Objective

Make the stable Rust API mirror official resource semantics without losing idiomatic Rust naming where it does not create ambiguity.

### Required Changes

- keep client accessors like `text_to_video()` and `image_to_video()`
- standardize resource methods on official names:
  `retrieve` instead of `get`
  `retrieve_usage` instead of `usage`
- decide whether aliases stay temporarily:
  `get -> retrieve`
  `usage -> retrieve_usage`

### Acceptance Criteria

- all stable resources expose the same operation set as the official SDK, adjusted only for Rust naming conventions
- method names do not hide materially different semantics
- docs and examples use the stable names only

## Workstream 2: Upload Flow Rewrite

### Objective

Implement the official ephemeral upload flow exactly.

### Required Behavior

1. `POST /v1/uploads` with `{ filename, type: "ephemeral" }`
2. deserialize `{ runwayUri, uploadUrl, fields }`
3. upload with multipart `POST`, including storage fields and optional metadata
4. return the `uri`

### Module Changes

- replace the current flow in [uploads.rs](/Users/abdel/dev/me/world-models/runway/runway-rs/src/resources/uploads.rs)
- introduce typed upload placeholder response types
- add multipart upload helper logic
- preserve the current "no auth headers to storage" safety behavior

### Public API

Stable API should expose:

- `uploads().create_ephemeral(params, options)`
- convenience helpers for file paths may be added on top, but only if they wrap the official flow rather than defining a different contract

### Acceptance Criteria

- multipart body includes storage `fields`, file, and optional metadata
- storage upload uses `POST`, not `PUT`
- the returned value is the server-provided `uri`
- tests verify that no `Authorization` header is sent to storage

## Workstream 3: Task And Workflow State Models

### Objective

Model task and workflow invocation lifecycle states exactly, including cancellation.

### Required Changes

- replace the flat task model in [task.rs](/Users/abdel/dev/me/world-models/runway/runway-rs/src/types/task.rs) with typed state-aware models
- add `CANCELLED` handling for tasks
- replace the thin workflow invocation model in [workflow.rs](/Users/abdel/dev/me/world-models/runway/runway-rs/src/types/workflow.rs) with a typed union
- include `output`, `progress`, `failure`, `failureCode`, and `nodeErrors` where the official contract includes them

### Acceptance Criteria

- deserialization succeeds for every official task and workflow invocation state
- polling treats `FAILED` and `CANCELLED` as terminal error paths
- success states expose typed output without `serde_json::Value` catch-alls where stronger typing is available

## Workstream 4: Management Resource Contract Alignment

### Objective

Bring avatars, documents, voices, realtime sessions, organization, and workflows into contract parity.

### Required Changes

- avatars:
  add cursor pagination and rich `PROCESSING | READY | FAILED` unions
- documents:
  align create, retrieve, update, and list response shapes with `content`, `type`, `updatedAt`, and `usedBy`
- voices:
  align create, preview, retrieve, and list types with processing states and preview URLs
- realtime sessions:
  model `NOT_READY | READY | RUNNING | COMPLETED | FAILED | CANCELLED`
- organization:
  replace the current schema with `creditBalance`, `tier`, and `usage`
- workflows:
  replace `WorkflowList { workflows: ... }` with the official grouped `data` shape and typed `nodeOutputs`

### Acceptance Criteria

- the Rust types deserialize official example payloads with no field loss
- list resources use cursor pagination where official
- no stable management resource relies on an underspecified `Option<String>` status field

## Workstream 5: Generation Request Contract Alignment

### Objective

Replace permissive generic generation structs with model-specific request types.

### Required Changes

- `text_to_video`
  model per-model request variants and legal ratios and durations
- `image_to_video`
  support model-specific prompt image formats, including positional arrays where official
- `video_to_video`
  align to official `gen4_aleph`, `videoUri`, and references shape
- `text_to_image`
  use official image ratios and `referenceImages`
- `character_performance`
  replace current prompt-style shape with `character`, `reference`, `bodyControl`, and `expressionIntensity`
- audio resources
  add explicit model fields and official voice or language object shapes

### Design Rule

Rust ergonomics must not weaken the contract.

If helper builders are added, they must build one of the official variants rather than collapsing them into an invalid superset.

### Acceptance Criteria

- every generation endpoint serializes exactly to official request bodies
- model-specific invalid states become unrepresentable or are rejected with clear validation errors
- snapshot tests cover representative payloads for every model variant

## Workstream 6: Runtime Parity

### Objective

Add the client capabilities needed for production-grade behavior.

### Required Changes

- add a `RequestOptions` type for per-request overrides
- add raw response access analogous to the official SDK
- add client cloning with option overrides
- add default headers and default query support
- add optional SDK logging hooks

### Proposed Rust Surface

The exact naming can be Rust-native, but the capabilities must exist.

One acceptable shape is:

- `RequestOptions`
- `ResponseMetadata`
- `WithResponse<T> { data, response }`
- `client.with_options(...)`
- `resource.create_with_options(params, options)`

If a different naming scheme is chosen, it must be documented and used consistently.

### Acceptance Criteria

- every stable resource method can accept per-request overrides
- callers can access status code and headers without reparsing the body
- request defaults and per-request overrides merge predictably

## Workstream 7: Retry And Error Taxonomy

### Objective

Raise transport behavior to production grade and official parity.

### Required Retry Behavior

Retry on:

- connection failures
- connection timeouts
- HTTP `408`
- HTTP `409`
- HTTP `429`
- HTTP `>=500`
- `x-should-retry: true`
- `retry-after-ms`
- numeric `retry-after`
- date-valued `retry-after`

Do not retry when:

- `x-should-retry: false`
- the request is non-retryable and no override says otherwise
- retries are exhausted

### Required Error Taxonomy

Add typed variants or structs equivalent to:

- bad request
- authentication
- permission denied
- not found
- conflict
- unprocessable entity
- rate limit
- internal server error
- connection error
- connection timeout
- user abort

### Acceptance Criteria

- retry logic is covered by transport tests for every supported trigger
- typed errors expose status and headers where available
- timeouts and user-driven cancellation are distinguishable

## Workstream 8: Pagination

### Objective

Implement official cursor pagination for avatars, documents, and voices.

### Required Changes

- add a generic cursor page type
- add iteration helpers for pages and items
- make `list` methods return typed pages rather than ad hoc vectors

### Acceptance Criteria

- callers can iterate page-by-page and item-by-item
- `next_cursor` handling matches server behavior
- pagination tests cover empty, single-page, and multi-page results

## Workstream 9: Polling

### Objective

Provide official-equivalent waiting behavior for tasks and workflow invocations.

### Required Changes

- extend [polling.rs](/Users/abdel/dev/me/world-models/runway/runway-rs/src/polling.rs) to support workflow invocations
- add wait options including timeout and cancellation
- treat cancelled terminal states as failure paths
- reuse shared polling logic where possible

### Acceptance Criteria

- task waiting works for `SUCCEEDED`, `FAILED`, and `CANCELLED`
- workflow invocation waiting works for `SUCCEEDED`, `FAILED`, and `CANCELLED`
- callers can stop polling cleanly

## Workstream 10: Documentation, Migration, And Release Policy

### Objective

Make the parity story explicit and safe for users.

### Required Changes

- update README examples to the new stable API
- add a parity matrix to the README
- add migration notes from `0.1.0`
- label unofficial features clearly
- document feature flags and stability levels

### Acceptance Criteria

- README does not claim official equivalence until the release gates pass
- docs show which modules are stable and which are experimental
- changelog or release notes enumerate breaking changes

## Test Strategy

## Fixture Tests

Add JSON fixtures for:

- task states
- workflow invocation states
- avatars
- documents
- voices
- realtime sessions
- organization
- workflows
- generation request bodies

Fixtures should come from official docs, official SDK examples, or captured live responses.

## Transport Tests

Add or expand mock-server tests for:

- upload placeholder request and multipart storage upload
- retry handling for `408`, `409`, `429`, `500`, `502`, `503`, `504`
- `retry-after-ms`
- date-valued `retry-after`
- `x-should-retry`
- raw response access
- per-request header and query overrides

## Live Tests

Add gated live tests behind a feature and environment variables.

Required env vars should include:

- `RUNWAYML_API_SECRET`
- optional per-test asset URIs or local asset paths where needed

Live tests should cover at minimum:

- ephemeral upload
- one representative generation task
- one management resource retrieval
- workflow invocation retrieval if the account has a published workflow

## CI Gates

The release branch must pass:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo doc --no-deps`
- gated live tests in a protected workflow or manual release workflow

## Implementation Order

Execute in this order:

1. stable API naming and public-surface decisions
2. upload rewrite
3. task and workflow state models
4. management resource schemas
5. generation request schemas
6. request options, raw responses, retries, and typed errors
7. pagination
8. polling unification
9. docs and migration
10. live verification and release checklist

## Release Gates

Do not label the crate production-ready until all of the following are true:

- stable public types are contract-faithful to the official SDK snapshot
- upload flow matches the official server contract
- retries and errors meet the spec above
- official paginated resources use cursor pagination
- task and workflow polling support cancellation and terminal failure states
- unofficial endpoints are isolated from the stable parity surface
- fixture, transport, and live tests pass
- README parity matrix is published

## Open Questions

- Can the upstream OpenAPI or Stainless source be obtained to eliminate manual drift?
- Are `webhookUrl` fields officially supported but omitted from the Node SDK, or are they private or legacy?
- Should deprecated aliases be kept for one pre-`1.0` release, or should the crate cut directly to the new stable names?
- Should unofficial endpoints live behind one feature flag or separate per-endpoint flags?

## Immediate Next Step

Start with Workstreams 1 through 3 in one branch:

- finalize stable naming
- rewrite uploads
- replace task and workflow invocation lifecycle models

Those three changes remove the highest-risk contract failures and give the rest of the migration a stable foundation.
