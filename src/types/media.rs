use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::RunwayError;

/// Flexible media input — accepts URLs, Runway upload URIs, or data URIs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}

impl<T: Into<String>> From<T> for MediaInput {
    fn from(value: T) -> Self {
        Self::Uri(value.into())
    }
}
