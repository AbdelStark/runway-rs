use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::RunwayError;

/// Flexible media input — accepts URLs, Runway upload URIs, or data URIs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MediaInput {
    Uri(String),
}

impl MediaInput {
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::Uri(url.into())
    }

    pub fn from_runway_uri(uri: impl Into<String>) -> Self {
        Self::Uri(uri.into())
    }

    pub fn from_base64(mime_type: &str, data: &[u8]) -> Self {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        Self::Uri(format!("data:{};base64,{}", mime_type, encoded))
    }

    pub fn from_file(path: &Path) -> Result<Self, RunwayError> {
        let data = std::fs::read(path)?;
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        Ok(Self::from_base64(&mime, &data))
    }

    pub fn as_uri(&self) -> &str {
        match self {
            Self::Uri(uri) => uri,
        }
    }

    pub fn into_uri(self) -> String {
        match self {
            Self::Uri(uri) => uri,
        }
    }
}

impl<T: Into<String>> From<T> for MediaInput {
    fn from(value: T) -> Self {
        Self::Uri(value.into())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TypedMediaInput {
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub uri: String,
}

impl TypedMediaInput {
    pub fn image(uri: impl Into<String>) -> Self {
        Self {
            media_type: MediaType::Image,
            uri: uri.into(),
        }
    }

    pub fn video(uri: impl Into<String>) -> Self {
        Self {
            media_type: MediaType::Video,
            uri: uri.into(),
        }
    }

    pub fn audio(uri: impl Into<String>) -> Self {
        Self {
            media_type: MediaType::Audio,
            uri: uri.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PromptFramePosition {
    First,
    Last,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PromptFrame {
    pub position: PromptFramePosition,
    pub uri: String,
}

impl PromptFrame {
    pub fn first(uri: impl Into<String>) -> Self {
        Self {
            position: PromptFramePosition::First,
            uri: uri.into(),
        }
    }

    pub fn last(uri: impl Into<String>) -> Self {
        Self {
            position: PromptFramePosition::Last,
            uri: uri.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PromptImageInput {
    Uri(String),
    Frames(Vec<PromptFrame>),
}

impl PromptImageInput {
    pub fn first_frame(uri: impl Into<String>) -> Self {
        Self::Frames(vec![PromptFrame::first(uri)])
    }

    pub fn with_last_frame(self, uri: impl Into<String>) -> Self {
        let last_uri = uri.into();
        match self {
            Self::Uri(first_uri) => Self::Frames(vec![
                PromptFrame::first(first_uri),
                PromptFrame::last(last_uri),
            ]),
            Self::Frames(mut frames) => {
                frames.retain(|frame| frame.position != PromptFramePosition::Last);
                frames.push(PromptFrame::last(last_uri));
                Self::Frames(frames)
            }
        }
    }

    pub fn as_uri(&self) -> Option<&str> {
        match self {
            Self::Uri(uri) => Some(uri),
            Self::Frames(_) => None,
        }
    }
}

impl From<MediaInput> for PromptImageInput {
    fn from(value: MediaInput) -> Self {
        Self::Uri(value.into_uri())
    }
}

impl From<String> for PromptImageInput {
    fn from(value: String) -> Self {
        Self::Uri(value)
    }
}

impl From<&str> for PromptImageInput {
    fn from(value: &str) -> Self {
        Self::Uri(value.to_string())
    }
}
