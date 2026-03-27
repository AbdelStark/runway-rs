//! Unofficial Rust SDK for the Runway API.
//!
//! Provides an async client for AI video, image, and audio generation
//! via the [Runway API](https://docs.dev.runwayml.com/).
//!
//! # Quick start
//!
//! ```no_run
//! use runway_sdk::{RunwayClient, TextToVideoRequest, VideoModel};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RunwayClient::new()?;
//!
//! let task = client
//!     .text_to_video()
//!     .create(TextToVideoRequest::new(VideoModel::Gen45, "A serene mountain at sunrise"))
//!     .await?
//!     .wait_for_output()
//!     .await?;
//!
//! println!("Video URL: {}", task.output.unwrap()[0]);
//! # Ok(())
//! # }
//! ```
//!
//! # Authentication
//!
//! Set the `RUNWAYML_API_SECRET` environment variable, or pass the key explicitly:
//!
//! ```no_run
//! # use runway_sdk::RunwayClient;
//! // From environment variable:
//! let client = RunwayClient::new().unwrap();
//!
//! // With explicit key:
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
//! | [`lip_sync()`](RunwayClient::lip_sync) | Synchronize lip movements |
//! | [`image_upscale()`](RunwayClient::image_upscale) | Upscale images |
//! | [`voice_dubbing()`](RunwayClient::voice_dubbing) | Dub audio to target language |
//! | [`voice_isolation()`](RunwayClient::voice_isolation) | Isolate voice from audio |
//! | [`tasks()`](RunwayClient::tasks) | List, get, cancel, and delete tasks |
//! | [`uploads()`](RunwayClient::uploads) | Upload media files |
//! | [`avatars()`](RunwayClient::avatars) | Manage avatars |
//! | [`voices()`](RunwayClient::voices) | Manage voice clones |
//! | [`documents()`](RunwayClient::documents) | Manage documents |
//! | [`workflows()`](RunwayClient::workflows) | List and run workflows |
//! | [`organization()`](RunwayClient::organization) | Organization info and usage |
//!
//! # Task lifecycle
//!
//! Generation methods return a [`PendingTask`] that can be polled or streamed:
//!
//! ```no_run
//! # use runway_sdk::*;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = RunwayClient::with_api_key("test")?;
//! // Option 1: Wait for completion
//! let task = client
//!     .text_to_video()
//!     .create(TextToVideoRequest::new(VideoModel::Gen45, "A cat"))
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
//! # Error handling
//!
//! All methods return [`Result<T, RunwayError>`]. The error type covers API
//! errors, rate limiting, task failures, and network issues:
//!
//! ```no_run
//! # use runway_sdk::*;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = RunwayClient::with_api_key("test")?;
//! match client.tasks().get(uuid::Uuid::new_v4()).await {
//!     Ok(task) => println!("Task status: {}", task.status),
//!     Err(RunwayError::Unauthorized) => eprintln!("Check your API key"),
//!     Err(RunwayError::RateLimited { retry_after }) => {
//!         eprintln!("Rate limited, retry after {:?}", retry_after);
//!     }
//!     Err(RunwayError::Api { status, message, code }) => {
//!         eprintln!("API error {}: {} (code: {:?})", status, message, code);
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

pub use client::RunwayClient;
pub use config::Config;
pub use error::RunwayError;
pub use polling::PendingTask;
pub use types::*;
