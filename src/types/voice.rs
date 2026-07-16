use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::RunwayError;
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
    pub description: Option<Option<String>>,
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
        self.description = Some(Some(description.into()));
        self
    }

    /// Explicitly clear the voice description by sending JSON `null`.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    pub fn from(mut self, from: VoiceFrom) -> Self {
        self.from = from;
        self
    }

    /// Create a custom voice by cloning a source audio URL.
    pub fn from_audio(name: impl Into<String>, audio: impl Into<String>) -> Self {
        Self {
            from: VoiceFrom::audio(audio),
            name: name.into(),
            description: None,
        }
    }

    /// Validate the selected source before transport.
    pub fn validate(&self) -> Result<(), RunwayError> {
        self.from.validate()
    }
}

/// Mutable fields accepted by the custom-voice update endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVoiceRequest {
    /// Description update. `Some(None)` explicitly clears the description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// New display name, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateVoiceRequest {
    /// Create an update with no fields set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the voice description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    /// Explicitly clear the voice description by sending JSON `null`.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Replace the voice display name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Source used to create a custom voice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum VoiceFrom {
    /// Clone a voice from an HTTPS audio URL.
    Audio { audio: String },
    /// Design a voice from a textual description.
    Text {
        model: VoiceDesignModel,
        prompt: String,
    },
}

impl VoiceFrom {
    /// Create an audio-cloning source.
    pub fn audio(audio: impl Into<String>) -> Self {
        Self::Audio {
            audio: audio.into(),
        }
    }

    /// Create a text-designed source using the latest preferred model.
    pub fn text(prompt: impl Into<String>) -> Self {
        Self::Text {
            model: VoiceDesignModel::ElevenTtvV3,
            prompt: prompt.into(),
        }
    }

    /// Override the design model for a text source.
    ///
    /// Audio sources have no model field and are returned unchanged.
    pub fn model(mut self, model: VoiceDesignModel) -> Self {
        if let Self::Text {
            model: current_model,
            ..
        } = &mut self
        {
            *current_model = model;
        }
        self
    }

    /// Return the source discriminator.
    pub fn source_type(&self) -> VoiceFromType {
        match self {
            Self::Audio { .. } => VoiceFromType::Audio,
            Self::Text { .. } => VoiceFromType::Text,
        }
    }

    /// Validate documented source constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if let Self::Text { prompt, .. } = self {
            validate_voice_design_prompt(prompt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VoiceFromType {
    Audio,
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
            model: VoiceDesignModel::ElevenTtvV3,
            prompt: prompt.into(),
        }
    }

    pub fn model(mut self, model: VoiceDesignModel) -> Self {
        self.model = model;
        self
    }

    /// Validate the documented prompt minimum.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_voice_design_prompt(&self.prompt)
    }
}

fn validate_voice_design_prompt(prompt: &str) -> Result<(), RunwayError> {
    if prompt.chars().count() < 20 {
        return Err(RunwayError::Validation {
            message: "Voice design prompt must contain at least 20 characters".into(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewVoiceResponse {
    pub duration_secs: f64,
    pub url: String,
}
