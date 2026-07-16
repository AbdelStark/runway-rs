# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-16

### Added

- Typed clients for avatar conversations, avatar videos, recipes, and the
  current upstream generation, management, and realtime API models.
- Audio-clone and text-designed voice creation, including explicit nullable
  descriptions and current voice-design validation.
- Per-request cancellation, idempotency, retry, timeout, header, query, and
  base-URL controls.

### Changed

- **Breaking:** management resources, public request and response types, and
  validation now match the current Runway API contract. See the
  [0.2.0 migration guide](docs/migration-0.2.md).
- Non-idempotent requests are no longer retried unless explicitly made safe,
  and polling deadlines now cover in-flight requests and retry backoff.
- File uploads now stream from disk instead of buffering the complete file.
- Pending task and workflow handles preserve submission routing context while
  excluding one-shot idempotency and cancellation controls.
- Hardened configuration, request validation, response-size limits, redaction,
  URL-stripped transport failures, and structured API error reporting.
- Expanded test, documentation, supply-chain, and release automation coverage.

### Security

- Updated `rustls-webpki` and `anyhow` to releases that resolve known RustSec
  advisories.

## [0.1.0] - 2026-03-27

### Added

- Initial unofficial async Rust client for typed Runway generation requests and
  responses.
- Task polling and status streaming for long-running generation jobs.
- Multipart file uploads, workflow invocation support, and runnable examples.
- Configurable retries and timeouts with structured API and transport errors.
- Mock-server integration tests plus opt-in live API tests.

[Unreleased]: https://github.com/AbdelStark/runway-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/AbdelStark/runway-rs/compare/96eac1264202063194a8d79189662ea01ed84a8e...v0.2.0
[0.1.0]: https://github.com/AbdelStark/runway-rs/tree/96eac1264202063194a8d79189662ea01ed84a8e
