# Security policy

`runway-rs` is an independent, unofficial community SDK. It is not affiliated
with or endorsed by Runway AI, Inc.

## Supported versions

Security fixes are provided for the latest released `0.2.x` version. Older
pre-release snapshots and unreleased Git commits are supported only on a
best-effort basis. This policy will be updated when another release line is
maintained.

| Version | Supported |
| ------- | --------- |
| 0.2.x   | Yes       |
| < 0.2   | No        |

## Report a vulnerability privately

Use a [private GitHub security advisory](https://github.com/AbdelStark/runway-rs/security/advisories/new)
to report a suspected vulnerability. Please do not open a public issue or pull
request before a fix and disclosure plan are agreed.

Include the affected version, impact, reproduction steps, and any suggested
mitigation. Do not include real API secrets, customer content, or personal data.
If a Runway API key may have been exposed, revoke it immediately and create a
replacement.

We aim to acknowledge reports within three business days and provide an initial
assessment within seven business days. Resolution timing depends on severity
and coordination needs. We will keep reporters informed and credit them unless
they prefer to remain anonymous.

This channel covers vulnerabilities in this Rust SDK. For issues in Runway's
hosted service, account security, abuse, or billing, contact
[Runway support](https://help.runwayml.com/hc/en-us) through its official
channels.

Please allow a reasonable remediation window before public disclosure. The
maintainer will coordinate a release and advisory when appropriate.
