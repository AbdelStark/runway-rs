use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::common::{CursorPage, CursorPageQuery};

pub type VoiceList = CursorPage<Voice>;
pub type VoiceListQuery = CursorPageQuery;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoiceCreateResponse {
    pub id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceStatus {
    Processing,
    Ready,
    Failed,
}

impl fmt::Display for VoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Processing => write!(f, "PROCESSING"),
            Self::Ready => write!(f, "READY"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Voice {
    Processing {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(default)]
        description: Option<String>,
        name: String,
    },
    Ready {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(default)]
        description: Option<String>,
        name: String,
        #[serde(rename = "previewUrl", default)]
        preview_url: Option<String>,
    },
    Failed {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(rename = "failureReason")]
        failure_reason: String,
        name: String,
    },
}

impl Voice {
    pub fn id(&self) -> &str {
        match self {
            Self::Processing { id, .. } | Self::Ready { id, .. } | Self::Failed { id, .. } => id,
        }
    }

    pub fn status(&self) -> VoiceStatus {
        match self {
            Self::Processing { .. } => VoiceStatus::Processing,
            Self::Ready { .. } => VoiceStatus::Ready,
            Self::Failed { .. } => VoiceStatus::Failed,
        }
    }

    pub fn preview_url(&self) -> Option<&str> {
        match self {
            Self::Ready { preview_url, .. } => preview_url.as_deref(),
            _ => None,
        }
    }

    pub fn failure_reason(&self) -> Option<&str> {
        match self {
            Self::Failed { failure_reason, .. } => Some(failure_reason),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceDesignModel {
    #[serde(rename = "eleven_multilingual_ttv_v2")]
    ElevenMultilingualTtvV2,
    #[serde(rename = "eleven_ttv_v3")]
    ElevenTtvV3,
}

impl fmt::Display for VoiceDesignModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenMultilingualTtvV2 => write!(f, "eleven_multilingual_ttv_v2"),
            Self::ElevenTtvV3 => write!(f, "eleven_ttv_v3"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateVoiceRequest {
    pub from: VoiceFrom,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateVoiceRequest {
    pub fn new(name: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            from: VoiceFrom::text(prompt),
            name: name.into(),
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn from(mut self, from: VoiceFrom) -> Self {
        self.from = from;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoiceFrom {
    pub model: VoiceDesignModel,
    pub prompt: String,
    #[serde(rename = "type")]
    pub source_type: VoiceFromType,
}

impl VoiceFrom {
    pub fn text(prompt: impl Into<String>) -> Self {
        Self {
            model: VoiceDesignModel::ElevenMultilingualTtvV2,
            prompt: prompt.into(),
            source_type: VoiceFromType::Text,
        }
    }

    pub fn model(mut self, model: VoiceDesignModel) -> Self {
        self.model = model;
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VoiceFromType {
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewVoiceRequest {
    pub model: VoiceDesignModel,
    pub prompt: String,
}

impl PreviewVoiceRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            model: VoiceDesignModel::ElevenMultilingualTtvV2,
            prompt: prompt.into(),
        }
    }

    pub fn model(mut self, model: VoiceDesignModel) -> Self {
        self.model = model;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewVoiceResponse {
    pub duration_secs: f64,
    pub url: String,
}
