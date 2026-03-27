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
