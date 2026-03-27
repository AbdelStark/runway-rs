# Runway Rust SDK Production Gap Analysis

Date: 2026-03-27

Compared against the official Node SDK cloned from `https://github.com/runwayml/sdk-node` into `/tmp/runway-sdk-node-fgCeMJ` at version `@runwayml/sdk 3.17.0`.

Implementation spec: [docs/specs/production-parity-spec.md](specs/production-parity-spec.md)

## Verdict

The Rust crate is not production-ready as an official-equivalent SDK yet.

What is good:

- The crate already has broad endpoint intent coverage.
- `cargo test` passes locally: 73 mock integration tests, 72 type tests, 8 doc tests.
- CI exists for `check`, `test`, `clippy`, `fmt`, and `doc`.

What blocks release:

- Several core resource schemas do not match the official API contract.
- The upload flow is implemented against a different wire format than the official SDK.
- Important runtime behaviors from the official SDK are missing: cursor pagination, typed union responses, per-request options, workflow polling, richer retries, raw response access, and typed error taxonomy.
- The crate also exposes extra unofficial surface area that should be separated from official parity work.

## High-Level Parity Matrix

| Area | Official Node SDK | Rust crate | Release assessment |
| --- | --- | --- | --- |
| Generation resources | Present | Present | Endpoint names mostly match, request schemas often do not |
| Tasks | `retrieve`, `delete` | `list`, `get`, `delete`, `cancel` | Surface diverges from official semantics |
| Uploads | `createEphemeral(file, fileMetadata)` | `create(filename)`, `upload_file(path)` | Wire contract mismatch, likely broken against production API |
| Avatars | Cursor pagination, rich union states | Flat list, thin struct | Schema mismatch |
| Documents | Cursor pagination, `data/nextCursor` | Flat list, thin struct | Schema mismatch |
| Voices | Cursor pagination, rich union states | Flat list, thin struct | Schema mismatch |
| Realtime sessions | Rich union retrieve response | Thin struct with optional strings | Schema mismatch |
| Organization | `creditBalance`, `tier`, `usage` | `id`, `name`, `created_at` | Schema mismatch |
| Workflows | Typed retrieve/list/run contracts | Simplified structs | Schema mismatch |
| Workflow invocations | Rich union + wait helper | Thin struct only | Schema mismatch |
| Polling | Task and workflow await helpers with abort support | Task-only polling, no abort | Missing behavior |
| Client runtime | Per-request overrides, raw response access, logging, richer retry policy | Global config only | Missing behavior |
| Extra endpoints | No `lip_sync`, `image_upscale`, no `webhookUrl` fields in SDK types | Those are exposed | Must be clearly separated from official parity |

## P0 Blockers

### 1. Resource schemas drift from the official API

These are not cosmetic differences. They are real contract mismatches that can cause deserialization failures or incorrect requests.

- Tasks:
  `TaskStatus` in Rust omits `CANCELLED`, but the official SDK models it as a first-class terminal state.
- Uploads:
  the official SDK first requests an upload placeholder, receives `runwayUri`, `uploadUrl`, and form `fields`, then performs a multipart `POST`.
  The Rust crate expects `id` and `uploadUrl`, then performs a `PUT`.
- Avatars:
  the official SDK uses cursor pagination and rich `PROCESSING | READY | FAILED` responses with voice unions, document IDs, personality, scripts, and timestamps.
  The Rust crate models a flat `Avatar` with only `id`, `name`, `description`, and `created_at`.
- Documents:
  the official SDK returns cursor pages and richer document objects including `content`, `type`, `updatedAt`, and `usedBy`.
  The Rust crate expects a flat `documents` array and a much smaller object.
- Voices:
  the official SDK returns cursor pages and `PROCESSING | READY | FAILED` unions with preview URLs and failure reasons.
  The Rust crate models a thin `Voice` and a flat `voices` array.
- Realtime sessions:
  the official SDK returns `NOT_READY | READY | RUNNING | COMPLETED | FAILED | CANCELLED` unions with fields like `sessionKey`, `expiresAt`, `duration`, and `failureCode`.
  The Rust crate uses one struct with optional `status` and `created_at`.
- Organization:
  the official SDK returns `creditBalance`, `tier`, and `usage`.
  The Rust crate expects `id`, `name`, and `created_at`.
- Workflows:
  the official SDK `list` shape is `{ data: [...] }` with versions grouped by workflow name.
  The Rust crate expects `{ workflows: [...] }`.
- Workflow invocations:
  the official SDK returns union states with `output`, `progress`, `failure`, `failureCode`, and `nodeErrors`.
  The Rust crate uses a thin struct with `status: Option<String>` and `output: Option<Value>`.

### 2. Several generation request types do not match the official contract

- `text_to_video`:
  the official SDK has model-specific unions and constraints.
  Rust accepts one generic `VideoModel` plus one generic request shape.
- `image_to_video`:
  the official SDK supports model-specific prompt-image formats, including arrays with positional images.
  Rust only supports a single generic `MediaInput`.
- `video_to_video`:
  the official SDK uses model `gen4_aleph`, `videoUri`, and optional image references.
  Rust models this as a generic video-generation request.
- `text_to_image`:
  the official SDK supports many image aspect ratios and optional `referenceImages`.
  Rust reuses `VideoRatio` and has no reference image support.
- `character_performance`:
  the official SDK uses model `act_two`, `character`, `reference`, `bodyControl`, and `expressionIntensity`.
  Rust uses `prompt_text`, `prompt_image`, and `prompt_video`.
- `sound_effect`, `text_to_speech`, `speech_to_speech`, `voice_dubbing`, and `voice_isolation`:
  the official SDK requires explicit model fields and typed preset voice or language objects.
  Rust uses simplified custom request shapes that do not line up with the official SDK.

### 3. Task semantics differ from the official SDK

- Official behavior:
  `DELETE /v1/tasks/{id}` handles both cancellation of active tasks and deletion of completed ones.
- Rust behavior:
  exposes a separate `cancel()` that posts to `/v1/tasks/{id}/cancel`, plus a `list()` endpoint that the official SDK does not expose.

This is an official-parity problem even if the custom endpoints happen to work.

### 4. Upload flow must be rewritten before release

This is the highest-risk transport issue because it affects all media-first workflows.

Required official behavior:

1. `POST /v1/uploads` with `{ filename, type: "ephemeral" }`
2. Receive `runwayUri`, `uploadUrl`, and form `fields`
3. Multipart `POST` file data to storage
4. Return the `uri`

Current Rust behavior:

1. `POST /v1/uploads` with `{ filename }`
2. Expect `{ id, uploadUrl }`
3. Raw `PUT` bytes to storage
4. Synthesize `runway://{id}`

## P1 Gaps

### 1. Missing runtime features from the official SDK

- No per-request request options:
  headers, query, timeout, retries, idempotency key, abort signal, and base URL override are not exposed.
- No raw response access:
  the official SDK supports `.asResponse()` and `.withResponse()`.
- No `withOptions()` cloning pattern.
- No default headers or default query parameters at client construction.
- No SDK-controlled request logging interface.

### 2. Retry and error handling are materially weaker

Official Node behavior retries:

- connection failures
- timeouts
- HTTP `408`
- HTTP `409`
- HTTP `429`
- HTTP `>=500`
- `x-should-retry`
- `retry-after-ms`
- date or numeric `retry-after`

Current Rust behavior retries only:

- HTTP `429`
- HTTP `502`
- HTTP `503`
- HTTP `504`

Current Rust behavior does not retry:

- request transport errors
- `408`
- `409`
- `500`
- `501`
- arbitrary `>=500`
- `retry-after-ms`
- `x-should-retry`
- date-valued `retry-after`

Error typing is also much coarser in Rust.
The official SDK exposes distinct classes such as `BadRequestError`, `AuthenticationError`, `ConflictError`, `RateLimitError`, and `InternalServerError`, each with headers attached.

### 3. Polling behavior is incomplete

- The official SDK provides task polling and workflow invocation polling.
- The official SDK supports aborting polling through `AbortSignal`.
- The official SDK treats `CANCELLED` as a terminal failure path.
- Rust polling is task-only, has no abort support, and currently cannot represent cancelled tasks.

### 4. Pagination support is incomplete

The official SDK uses cursor-based pagination for avatars, documents, and voices, with `PagePromise`, `hasNextPage()`, `getNextPage()`, and async iteration.

The Rust crate currently only has custom streaming pagination for tasks, and that task list surface is not part of the official SDK surface being matched.

## P2 Gaps

### 1. Extra unofficial surface area should be isolated

The Rust crate currently exposes functionality that is not present in the official Node SDK snapshot:

- `lip_sync`
- `image_upscale`
- request-level `webhook_url` fields across generation types
- `tasks.list`
- `tasks.cancel`

This may still be valuable, but it should not be mixed into "official-equivalent" claims without clear documentation and compatibility guarantees.

### 2. Builder ergonomics are convenient but not spec-safe

The current Rust builders are easy to use, but because they flatten model-specific request variants into one struct, they cannot enforce official model-specific rules such as:

- required versus optional `duration`
- legal ratios per model
- allowed voice object shapes
- typed media unions
- typed workflow node outputs

For production-grade parity, ergonomics should come after correctness, or be built on top of a correct typed core.

## Strategic Recommendation

Do not continue hand-maintaining the Rust surface by intuition.

The official Node SDK is generated from a single API description. The fastest path to a durable production-grade Rust SDK is:

1. Use the same OpenAPI source if available.
2. Rebuild request and response types from that contract.
3. Regenerate or mechanically derive resource methods where possible.
4. Layer Rust-native ergonomics on top of a spec-faithful transport and type layer.

Manual drift is already visible across schemas, behaviors, and retries.

## Recommended Release Sequence

### Phase 1: Contract correctness

- Replace all mismatched request and response types with official-contract equivalents.
- Rewrite uploads to match the official placeholder plus multipart flow.
- Add missing task and workflow invocation terminal states.
- Align workflows, organization, avatars, documents, voices, and realtime session schemas.

### Phase 2: Runtime parity

- Add per-request options.
- Add richer retry logic and typed HTTP errors.
- Add raw response access APIs.
- Add workflow invocation wait helpers and cancellable polling.
- Add cursor pagination abstractions for official paginated resources.

### Phase 3: Product hardening

- Add live integration tests against a real Runway account in a gated workflow.
- Add compatibility fixtures copied from the official SDK schema examples.
- Validate upload, polling, cancellation, and pagination against real endpoints.
- Split unofficial or beta endpoints behind features or clearly labeled modules.

### Phase 4: Release readiness

- Audit docs and examples for official-only claims.
- Publish a parity matrix in the README.
- Define semver expectations and backwards-compatibility policy.
- Only then cut a `1.0` or "production-ready" release.

## Verification Performed

- Cloned official SDK into `/tmp/runway-sdk-node-fgCeMJ`
- Inspected official source and README/API surface
- Ran `cargo test` successfully in the Rust crate
- Ran `cargo fmt -- --check` successfully
- Ran `cargo clippy --all-targets --all-features -- -D warnings`
  and fixed one failing lint in the test suite

## Bottom Line

This crate has a useful foundation, but it is currently a partially compatible unofficial SDK, not an official-equivalent production SDK.

The primary work now is not adding more endpoints.
It is correcting the wire contract, type system, pagination model, upload flow, polling semantics, and runtime behavior so the crate matches the real API and the official SDK's expectations.
