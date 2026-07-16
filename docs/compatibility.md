# Compatibility contract

This document defines what “compatible with the Runway API” means for
`runway-sdk` 0.2.0. It separates source-level contract evidence from behavior
that can only be confirmed against a live account.

## Pinned upstream evidence

The 0.2.0 surface was audited on 2026-07-16 against:

| Upstream | Version | Commit |
| --- | --- | --- |
| [Official Node SDK](https://github.com/runwayml/sdk-node/tree/d636bc5cd47125fd79fb04398ba98d8946839195) | 4.10.0 | `d636bc5cd47125fd79fb04398ba98d8946839195` |
| [Official Python SDK](https://github.com/runwayml/sdk-python/tree/97de8ed7fac3f59f2a31077b229a7b0732acc2cf) | 5.10.0 | `97de8ed7fac3f59f2a31077b229a7b0732acc2cf` |
| [Runway API documentation](https://docs.dev.runwayml.com/api/) | live documentation viewed 2026-07-16 | not versioned |

The official generated SDK source is the canonical evidence for wire names,
discriminated unions, optionality, endpoint methods, and paths. This crate
sends `X-Runway-Version: 2024-11-06` by default.

## Operation parity

The stable Rust surface covers the 52 operations exposed by the pinned
official SDK, with idiomatic snake-case accessors and method names.

| Area | Operations | Rust surface |
| --- | ---: | --- |
| Tasks | 2 | retrieve, delete |
| Video generation | 3 | image-to-video, text-to-video, video-to-video create |
| Image generation | 3 | text-to-image, image-upscale, video-upscale create |
| Character and audio | 6 | character performance, text-to-speech, sound effect, voice isolation, voice dubbing, speech-to-speech create |
| Organization | 2 | retrieve, retrieve usage |
| Avatars | 10 | avatar CRUD/list/usage, conversation retrieve/list/delete, avatar-video create |
| Documents | 5 | create, retrieve, update, list, delete |
| Realtime sessions | 3 | create, retrieve, cancel |
| Recipes | 7 | all seven typed recipe submissions |
| Voices | 6 | create, retrieve, update, preview, list, delete |
| Workflows | 4 | workflow retrieve/list/run and invocation retrieve |
| Uploads | 1 | ephemeral upload handoff |
| **Total** | **52** | |

Cursor pagination is represented by typed pages plus Rust streams. Methods
ending in `_with_options` are the Rust equivalent of retaining parsed data and
the underlying response metadata.

## Current model matrix

Each model is represented by a dedicated request variant when its legal fields
or constraints differ from another model.

| Endpoint | Supported model discriminants |
| --- | --- |
| Text to video | `gen4.5`, `veo3.1`, `veo3.1_fast`, `happyhorse_1_0`, `seedance2`, `seedance2_fast`, `seedance2_mini`, `gemini_omni_flash`, `veo3` |
| Image to video | the nine text-to-video models plus `gen4_turbo` |
| Video to video | `aleph2`, `seedance2`, `seedance2_fast`, `seedance2_mini`, `gemini_omni_flash` |
| Text to image | `gen4_image_turbo`, `gen4_image`, `gpt_image_2`, `gemini_image3_pro`, `gemini_image3.1_flash`, `seedream5_pro`, `seedream5_lite`, `gemini_2.5_flash` |
| Image upscale | `magnific_precision_upscaler_v2` |
| Video upscale | `magnific_video_upscaler_creative` |
| Character performance | `act_two` |
| Text to speech | `eleven_multilingual_v2`, `seed_audio` |
| Sound effect | `eleven_text_to_sound_v2`, `seed_audio` |
| Speech to speech | `eleven_multilingual_sts_v2` |
| Voice dubbing | `eleven_voice_dubbing` |
| Voice isolation | `eleven_voice_isolation` |

Legacy constructors retained for migration are documented as compatibility
shims when their model is no longer present upstream. They are not a claim
that Runway still accepts the old model.

At audit time, the [live model guide](https://docs.dev.runwayml.com/guides/models/)
and generated SDK were not perfectly
synchronized: the guide still listed deprecated `gen4_aleph` with a 2026-07-30
sunset, while the current video-to-video SDK union had already moved to
`aleph2`; conversely, the generated SDK exposed `gemini_image3.1_flash` while
the model guide did not list it. The Rust contract follows the pinned generated
SDK and treats actual account/model availability as server-validated.

## Intentional differences

The goal is wire compatibility, not a byte-for-byte port of another language's
runtime.

- Official SDKs may retry mutation requests automatically. This crate requires
  an idempotency key or an explicit unsafe opt-in before retrying POST/PATCH,
  because an ambiguous retry can duplicate billable work.
- JavaScript abort signals map to `tokio_util::sync::CancellationToken`.
- JavaScript page promises map to typed page values and `Stream` helpers.
- Request unions are serialization-focused. The SDK does not promise that an
  arbitrary request JSON body can be deserialized back into the same Rust
  variant when upstream variants share structurally ambiguous fields.
- Lip sync and task list/cancel helpers are community extensions behind
  `unstable-endpoints`; they are excluded from the 52-operation parity count.

## Verification boundary

The release gate verifies:

- exact JSON discriminants, field names, omission behavior, query names, paths,
  methods, and percent-encoded identifiers;
- all documented request variants and local validation rules;
- pagination, response unions, error classification, bounded bodies, retry
  decisions, cancellation, absolute polling deadlines, and upload handoff;
- absence of Runway authorization headers on presigned storage requests;
- compilation and tests on the MSRV plus stable Linux, macOS, and Windows CI;
- docs.rs-style documentation, package contents, dependency advisories, license
  policy, and Clippy with warnings denied.

Normal CI never calls the live API. The real-account tests are ignored and can
incur charges; the manual workflow requires an explicit confirmation and a
repository secret. No successful billable generation is claimed solely from
the deterministic contract suite. Account tier, credits, moderation, remote
media dimensions/duration, model rollout, and service-side policy remain
server-validated.

## Updating the snapshot

An upstream-sync change must record the new official versions and commits,
diff every resource and discriminated union, update this matrix, and add a
deterministic test for every changed wire contract. A green compile alone is
not sufficient evidence of parity.
