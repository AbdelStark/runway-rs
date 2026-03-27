#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
//! Async Rust SDK for the [Runway API](https://docs.dev.runwayml.com/).
//!
//! `runway-sdk` provides typed request models, multipart upload helpers,
//! task and workflow polling, and per-request overrides on top of `reqwest`.
//!
//! # Quick Start
//!
//! ```no_run
//! use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunwayClient::new()?;
//!
//! let task = client
//!     .text_to_video()
//!     .create(TextToVideoGen45Request::new(
//!         "A serene mountain at sunrise",
//!         VideoRatio::Landscape,
//!         5,
//!     ))
//!     .await?
//!     .wait_for_output()
//!     .await?;
//!
//! println!("Video URL: {}", task.output_urls().unwrap()[0]);
//! # Ok(())
//! # }
//! ```
//!
//! # Authentication
//!
//! Set `RUNWAYML_API_SECRET`, or pass the key explicitly:
//!
//! ```no_run
//! # use runway_sdk::RunwayClient;
//! // Read from RUNWAYML_API_SECRET
//! let client = RunwayClient::new().unwrap();
//!
//! // Or pass the key directly
//! let client = RunwayClient::with_api_key("sk_test_...").unwrap();
//! ```
//!
//! # Configuration
//!
//! Use [`Config`] to customize timeouts, retries, and polling behavior:
//!
//! ```no_run
//! use runway_sdk::{Config, RunwayClient};
//! use std::time::Duration;
//!
//! let config = Config::new("sk_test_...")
//!     .timeout(Duration::from_secs(120))
//!     .max_retries(5)
//!     .poll_interval(Duration::from_secs(10));
//!
//! let client = RunwayClient::with_config(config).unwrap();
//! ```
//!
//! # Per-Request Overrides
//!
//! Use [`RequestOptions`] when one call needs different headers, query params,
//! timeouts, or retry behavior:
//!
//! ```no_run
//! use runway_sdk::{RequestOptions, RunwayClient};
//! use std::time::Duration;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunwayClient::new()?;
//! let options = RequestOptions::new()
//!     .timeout(Duration::from_secs(30))
//!     .idempotency_key("req_123");
//!
//! let response = client
//!     .organization()
//!     .retrieve_with_options(options)
//!     .await?;
//! # let _ = response;
//! # Ok(())
//! # }
//! ```
//!
//! # Raw Responses
//!
//! Methods ending in `_with_options` return [`WithResponse`] so you can inspect
//! headers and status codes alongside the parsed payload:
//!
//! ```no_run
//! use runway_sdk::{RequestOptions, RunwayClient};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunwayClient::new()?;
//! let response = client
//!     .organization()
//!     .retrieve_with_options(RequestOptions::default())
//!     .await?;
//!
//! println!("status={}", response.response.status);
//! println!("credits={}", response.data.credit_balance);
//! # Ok(())
//! # }
//! ```
//!
//! # Resources
//!
//! All API resources are accessed via methods on [`RunwayClient`]:
//!
//! | Method | Description |
//! |--------|-------------|
//! | [`text_to_video()`](RunwayClient::text_to_video) | Generate video from text prompts |
//! | [`image_to_video()`](RunwayClient::image_to_video) | Animate images into video |
//! | [`video_to_video()`](RunwayClient::video_to_video) | Transform existing video |
//! | [`text_to_image()`](RunwayClient::text_to_image) | Generate images from text |
//! | [`text_to_speech()`](RunwayClient::text_to_speech) | Convert text to speech |
//! | [`speech_to_speech()`](RunwayClient::speech_to_speech) | Voice conversion |
//! | [`sound_effect()`](RunwayClient::sound_effect) | Generate sound effects |
//! | [`character_performance()`](RunwayClient::character_performance) | Animate characters |
//! | [`voice_dubbing()`](RunwayClient::voice_dubbing) | Dub audio to target language |
//! | [`voice_isolation()`](RunwayClient::voice_isolation) | Isolate voice from audio |
//! | [`tasks()`](RunwayClient::tasks) | Retrieve and delete tasks |
//! | [`uploads()`](RunwayClient::uploads) | Upload media files |
//! | [`avatars()`](RunwayClient::avatars) | Manage avatars |
//! | [`voices()`](RunwayClient::voices) | Manage voice clones |
//! | [`documents()`](RunwayClient::documents) | Manage documents |
//! | [`workflows()`](RunwayClient::workflows) | List and run workflows |
//! | [`organization()`](RunwayClient::organization) | Organization info and usage |
//!
//! # Feature Flags
//!
//! - `unstable-endpoints`: enables `lip_sync`, `image_upscale`, and task
//!   list/cancel helpers that are intentionally not part of the default surface.
//! - `live-tests`: enables the real API smoke suite in `tests/live_api.rs`.
//!
//! # Task Lifecycle
//!
//! Generation methods return a [`PendingTask`] that can be polled or streamed.
//! Workflow runs can return a [`PendingWorkflowInvocation`] via
//! [`WorkflowsResource::run_pending`](crate::resources::WorkflowsResource::run_pending):
//!
//! ```no_run
//! # use runway_sdk::*;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = RunwayClient::with_api_key("test")?;
//! let task = client
//!     .text_to_video()
//!     .create(TextToVideoGen45Request::new(
//!         "A cat",
//!         VideoRatio::Landscape,
//!         5,
//!     ))
//!     .await?
//!     .wait_for_output()
//!     .await?;
//!
//! if let Some(urls) = task.output_urls() {
//!     println!("Generated: {}", urls[0]);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Error Handling
//!
//! All methods return [`Result<T, RunwayError>`]. The error type covers API
//! errors, rate limiting, task or workflow failures, and transport issues:
//!
//! ```no_run
//! # use runway_sdk::*;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = RunwayClient::with_api_key("test")?;
//! match client.tasks().retrieve(uuid::Uuid::new_v4()).await {
//!     Ok(task) => println!("Task status: {}", task.status()),
//!     Err(RunwayError::Unauthorized) => eprintln!("Check your API key"),
//!     Err(RunwayError::RateLimited { retry_after, .. }) => {
//!         eprintln!("Rate limited, retry after {:?}", retry_after);
//!     }
//!     Err(RunwayError::Api { status, message, code, kind, .. }) => {
//!         eprintln!("API error {} ({:?}): {} (code: {:?})", status, kind, message, code);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod polling;
pub mod resources;
pub mod types;

pub use client::{RequestOptions, ResponseMetadata, RunwayClient, WithResponse};
pub use config::Config;
pub use error::RunwayError;
pub use polling::{PendingTask, PendingWorkflowInvocation, WaitOptions};
pub use types::*;
