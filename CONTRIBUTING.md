# Contributing to runway-rs

Thank you for helping improve `runway-rs`. This is an independent, unofficial
community SDK and is not affiliated with or endorsed by Runway AI, Inc.

## Before you start

- Install Git and a Rust toolchain through
  [rustup](https://rustup.rs/). Stable Rust is recommended; the minimum
  supported Rust version (MSRV) is 1.89.
- Search existing issues before opening a new one.
- For substantial API or behavior changes, open an issue first so the contract
  and compatibility impact can be agreed on before implementation.

Fork the repository, create a focused branch, and bootstrap the project with:

```console
cargo fetch --locked
cargo build --locked --all-features
```

## Development checks

Before submitting a pull request, run the checks that CI enforces:

```console
cargo fmt --all --check
npx --yes markdownlint-cli2@0.23.0 '**/*.md' '#target' '#.git'
cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --all-features --no-deps
cargo audit
cargo deny check
cargo package --locked
```

Install the two optional supply-chain tools with `cargo install cargo-audit
cargo-deny` if they are not already available. CI tests the MSRV separately;
contributors can reproduce that leg with `cargo +1.89 test --locked
--all-targets --all-features`.

Tests behind the `live-tests` feature contact the real Runway API and can incur
charges. CI never runs them. Only run a live test deliberately, with your own
short-lived `RUNWAYML_API_SECRET`, and never commit credentials or captured
customer data.

## API contract and compatibility

Treat the official [Runway API documentation](https://docs.dev.runwayml.com/)
as the primary contract. The official JavaScript and Python SDKs can provide
additional implementation evidence, but do not infer undocumented behavior
from examples alone. In a pull request that changes a request, response, model,
or endpoint, link the upstream evidence and add a deterministic mock-server or
serialization test.

Public API changes must preserve semantic-versioning expectations. New public
items need rustdoc examples or explanations, and user-visible changes belong in
the `[Unreleased]` section of `CHANGELOG.md`. Avoid exposing upstream dependency
types unless they are intentionally part of this crate's long-term contract.

## Pull requests

Keep each pull request reviewable and describe:

- the user-visible problem and solution;
- the upstream contract evidence, where relevant;
- the exact commands used to validate it; and
- compatibility, security, billing, or migration considerations.

Maintainers may ask for changes before merging. A pull request is not a promise
that a feature will be released on a particular schedule.

## Maintainer release process

Releases use the manual `Release` GitHub Actions workflow. Before the first
publish, configure the `crates-io` GitHub environment with required reviewer
approval, then configure a [crates.io trusted publisher](https://crates.io/docs/trusted-publishing)
for this repository, `.github/workflows/release.yml`, and that environment. No
long-lived registry token is stored in GitHub.

1. Update the crate version and add a dated changelog section in a reviewed
   commit on `main`.
2. Run every development check above, then create and push the matching tag
   `v<version>` from that reviewed commit.
3. Dispatch the workflow in `verify-only` mode with the explicit version and
   tag. Resolve every failure before continuing.
4. Dispatch it again in `publish` mode and enter the exact confirmation
   `publish runway-sdk <version>`.

The workflow checks out the existing tag, verifies that it matches the manifest
and changelog, audits dependencies, checks semantic-versioning compatibility,
and builds the package before the protected publish job requests a short-lived
OIDC token. It creates the GitHub release only after crates.io accepts the
package. Never publish from an unreviewed branch or a local dirty worktree.

## Reporting security issues

Do not open a public issue for a suspected vulnerability. Follow
[`SECURITY.md`](SECURITY.md) to report it privately.

## Licensing

Unless you state otherwise, any contribution intentionally submitted for
inclusion in this project is licensed under both the Apache License, Version
2.0 and the MIT License, without additional terms or conditions.
