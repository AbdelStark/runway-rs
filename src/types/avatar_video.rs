//! Request models for speech-driven avatar video generation.

use serde::{Deserialize, Serialize};

/// Runway preset avatars supported by the avatar-video endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum AvatarVideoPresetId {
    /// Game character preset.
    GameCharacter,
    /// Music superstar preset.
    MusicSuperstar,
    /// Male game character preset.
    GameCharacterMan,
    /// Cat character preset.
    CatCharacter,
    /// Influencer preset.
    Influencer,
    /// Tennis coach preset.
    TennisCoach,
    /// Human resources presenter preset.
    HumanResource,
    /// Fashion designer preset.
    FashionDesigner,
    /// Cooking teacher preset.
    CookingTeacher,
}

/// Runway preset voices supported by avatar text-to-speech.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AvatarVideoVoicePresetId {
    /// Victoria preset voice.
    Victoria,
    /// Vincent preset voice.
    Vincent,
    /// Clara preset voice.
    Clara,
    /// Drew preset voice.
    Drew,
    /// Skye preset voice.
    Skye,
    /// Max preset voice.
    Max,
    /// Morgan preset voice.
    Morgan,
    /// Felix preset voice.
    Felix,
    /// Mia preset voice.
    Mia,
    /// Marcus preset voice.
    Marcus,
    /// Summer preset voice.
    Summer,
    /// Ruby preset voice.
    Ruby,
    /// Aurora preset voice.
    Aurora,
    /// Jasper preset voice.
    Jasper,
    /// Leo preset voice.
    Leo,
    /// Adrian preset voice.
    Adrian,
    /// Nina preset voice.
    Nina,
    /// Emma preset voice.
    Emma,
    /// Blake preset voice.
    Blake,
    /// David preset voice.
    David,
    /// Maya preset voice.
    Maya,
    /// Nathan preset voice.
    Nathan,
    /// Sam preset voice.
    Sam,
    /// Georgia preset voice.
    Georgia,
    /// Petra preset voice.
    Petra,
    /// Adam preset voice.
    Adam,
    /// Zach preset voice.
    Zach,
    /// Violet preset voice.
    Violet,
    /// Roman preset voice.
    Roman,
    /// Luna preset voice.
    Luna,
}

/// Avatar input used for an avatar-video generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AvatarVideoAvatar {
    /// A Runway-provided preset avatar.
    #[serde(rename = "runway-preset")]
    RunwayPreset {
        /// Preset avatar identifier.
        #[serde(rename = "presetId")]
        preset_id: AvatarVideoPresetId,
    },
    /// A user-created avatar.
    #[serde(rename = "custom")]
    Custom {
        /// Custom avatar identifier.
        #[serde(rename = "avatarId")]
        avatar_id: String,
    },
}

impl AvatarVideoAvatar {
    /// Select a Runway-provided preset avatar.
    pub fn runway_preset(preset_id: AvatarVideoPresetId) -> Self {
        Self::RunwayPreset { preset_id }
    }

    /// Select a user-created avatar.
    pub fn custom(avatar_id: impl Into<String>) -> Self {
        Self::Custom {
            avatar_id: avatar_id.into(),
        }
    }
}

/// Optional text-to-speech voice override for an avatar video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AvatarVideoVoice {
    /// A Runway preset voice.
    Preset {
        /// Preset voice identifier.
        #[serde(rename = "presetId")]
        preset_id: AvatarVideoVoicePresetId,
    },
    /// A custom voice created through the Voices API.
    Custom {
        /// Custom voice identifier.
        id: String,
    },
}

impl AvatarVideoVoice {
    /// Select a Runway preset voice.
    pub fn preset(preset_id: AvatarVideoVoicePresetId) -> Self {
        Self::Preset { preset_id }
    }

    /// Select a custom voice.
    pub fn custom(id: impl Into<String>) -> Self {
        Self::Custom { id: id.into() }
    }
}

/// Speech source for avatar video generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AvatarVideoSpeech {
    /// An existing audio file for the avatar to speak.
    Audio {
        /// HTTPS URL of the source audio.
        audio: String,
    },
    /// A text script synthesized with the avatar's voice.
    Text {
        /// Text script to synthesize.
        text: String,
        /// Optional voice override; the avatar's configured voice is used by default.
        #[serde(skip_serializing_if = "Option::is_none")]
        voice: Option<AvatarVideoVoice>,
    },
}

impl AvatarVideoSpeech {
    /// Use an existing audio file as the speech source.
    pub fn audio(audio: impl Into<String>) -> Self {
        Self::Audio {
            audio: audio.into(),
        }
    }

    /// Synthesize a text script with the avatar's configured voice.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text {
            text: text.into(),
            voice: None,
        }
    }

    /// Synthesize a text script with an explicit voice override.
    pub fn text_with_voice(text: impl Into<String>, voice: AvatarVideoVoice) -> Self {
        Self::Text {
            text: text.into(),
            voice: Some(voice),
        }
    }
}

/// Model accepted by the avatar-video endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AvatarVideoModel {
    /// Runway's generative world model for avatars.
    #[serde(rename = "gwm1_avatars")]
    Gwm1Avatars,
}

/// Request to generate a video of an avatar speaking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvatarVideoCreateRequest {
    /// Avatar that will appear in the generated video.
    pub avatar: AvatarVideoAvatar,
    /// Avatar-video model to use.
    pub model: AvatarVideoModel,
    /// Audio or text speech source.
    pub speech: AvatarVideoSpeech,
}

impl AvatarVideoCreateRequest {
    /// Create an avatar-video request using the current avatar model.
    pub fn new(avatar: AvatarVideoAvatar, speech: AvatarVideoSpeech) -> Self {
        Self {
            avatar,
            model: AvatarVideoModel::Gwm1Avatars,
            speech,
        }
    }
}
