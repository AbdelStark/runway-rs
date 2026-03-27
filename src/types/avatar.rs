use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::common::{CursorPage, CursorPageQuery};

pub type AvatarList = CursorPage<Avatar>;
pub type AvatarListQuery = CursorPageQuery;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvatarStatus {
    Processing,
    Ready,
    Failed,
}

impl fmt::Display for AvatarStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Processing => write!(f, "PROCESSING"),
            Self::Ready => write!(f, "READY"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AvatarVoice {
    #[serde(rename = "runway-live-preset")]
    RunwayLivePreset {
        #[serde(rename = "presetId")]
        preset_id: String,
        name: String,
        description: String,
    },
    #[serde(rename = "custom")]
    Custom {
        id: String,
        deleted: bool,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AvatarVoiceInput {
    #[serde(rename = "runway-live-preset")]
    RunwayLivePreset {
        #[serde(rename = "presetId")]
        preset_id: String,
    },
    #[serde(rename = "custom")]
    Custom { id: String },
}

impl AvatarVoiceInput {
    pub fn runway_live_preset(preset_id: impl Into<String>) -> Self {
        Self::RunwayLivePreset {
            preset_id: preset_id.into(),
        }
    }

    pub fn custom(id: impl Into<String>) -> Self {
        Self::Custom { id: id.into() }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AvatarImageProcessing {
    Optimize,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Avatar {
    Processing {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "documentIds")]
        document_ids: Vec<String>,
        name: String,
        personality: String,
        #[serde(rename = "processedImageUri", default)]
        processed_image_uri: Option<String>,
        #[serde(rename = "referenceImageUri", default)]
        reference_image_uri: Option<String>,
        #[serde(rename = "startScript", default)]
        start_script: Option<String>,
        #[serde(rename = "updatedAt")]
        updated_at: String,
        voice: AvatarVoice,
    },
    Ready {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "documentIds")]
        document_ids: Vec<String>,
        name: String,
        personality: String,
        #[serde(rename = "processedImageUri", default)]
        processed_image_uri: Option<String>,
        #[serde(rename = "referenceImageUri", default)]
        reference_image_uri: Option<String>,
        #[serde(rename = "startScript", default)]
        start_script: Option<String>,
        #[serde(rename = "updatedAt")]
        updated_at: String,
        voice: AvatarVoice,
    },
    Failed {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "documentIds")]
        document_ids: Vec<String>,
        #[serde(rename = "failureReason")]
        failure_reason: String,
        name: String,
        personality: String,
        #[serde(rename = "processedImageUri", default)]
        processed_image_uri: Option<String>,
        #[serde(rename = "referenceImageUri", default)]
        reference_image_uri: Option<String>,
        #[serde(rename = "startScript", default)]
        start_script: Option<String>,
        #[serde(rename = "updatedAt")]
        updated_at: String,
        voice: AvatarVoice,
    },
}

impl Avatar {
    pub fn id(&self) -> &str {
        match self {
            Self::Processing { id, .. } | Self::Ready { id, .. } | Self::Failed { id, .. } => id,
        }
    }

    pub fn status(&self) -> AvatarStatus {
        match self {
            Self::Processing { .. } => AvatarStatus::Processing,
            Self::Ready { .. } => AvatarStatus::Ready,
            Self::Failed { .. } => AvatarStatus::Failed,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Processing { name, .. }
            | Self::Ready { name, .. }
            | Self::Failed { name, .. } => name,
        }
    }

    pub fn failure_reason(&self) -> Option<&str> {
        match self {
            Self::Failed { failure_reason, .. } => Some(failure_reason),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateAvatarRequest {
    pub name: String,
    pub personality: String,
    pub reference_image: String,
    pub voice: AvatarVoiceInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_processing: Option<AvatarImageProcessing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_script: Option<String>,
}

impl CreateAvatarRequest {
    pub fn new(
        name: impl Into<String>,
        personality: impl Into<String>,
        reference_image: impl Into<String>,
        voice: AvatarVoiceInput,
    ) -> Self {
        Self {
            name: name.into(),
            personality: personality.into(),
            reference_image: reference_image.into(),
            voice,
            document_ids: None,
            image_processing: None,
            start_script: None,
        }
    }

    pub fn document_ids(mut self, document_ids: impl Into<Vec<String>>) -> Self {
        self.document_ids = Some(document_ids.into());
        self
    }

    pub fn image_processing(mut self, image_processing: AvatarImageProcessing) -> Self {
        self.image_processing = Some(image_processing);
        self
    }

    pub fn start_script(mut self, start_script: impl Into<String>) -> Self {
        self.start_script = Some(start_script.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAvatarRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_processing: Option<AvatarImageProcessing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_script: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<AvatarVoiceInput>,
}

impl UpdateAvatarRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn document_ids(mut self, document_ids: impl Into<Vec<String>>) -> Self {
        self.document_ids = Some(document_ids.into());
        self
    }

    pub fn image_processing(mut self, image_processing: AvatarImageProcessing) -> Self {
        self.image_processing = Some(image_processing);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn personality(mut self, personality: impl Into<String>) -> Self {
        self.personality = Some(personality.into());
        self
    }

    pub fn reference_image(mut self, reference_image: impl Into<String>) -> Self {
        self.reference_image = Some(reference_image.into());
        self
    }

    pub fn start_script(mut self, start_script: impl Into<String>) -> Self {
        self.start_script = Some(Some(start_script.into()));
        self
    }

    pub fn clear_start_script(mut self) -> Self {
        self.start_script = Some(None);
        self
    }

    pub fn voice(mut self, voice: AvatarVoiceInput) -> Self {
        self.voice = Some(voice);
        self
    }
}
