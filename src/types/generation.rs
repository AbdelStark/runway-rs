use serde::{Deserialize, Serialize};

use super::common::ContentModeration;
use super::media::MediaInput;
use super::models::{ImageModel, VideoModel, VideoRatio};

// ── Image to Video ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub prompt_image: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

impl ImageToVideoRequest {
    pub fn new(model: VideoModel, prompt: impl Into<String>, image: MediaInput) -> Self {
        Self {
            model,
            prompt_text: prompt.into(),
            prompt_image: image,
            ratio: None,
            duration: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn duration(mut self, secs: u8) -> Self {
        self.duration = Some(secs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn content_moderation(mut self, cm: ContentModeration) -> Self {
        self.content_moderation = Some(cm);
        self
    }
}

// ── Text to Video ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

impl TextToVideoRequest {
    pub fn new(model: VideoModel, prompt: impl Into<String>) -> Self {
        Self {
            model,
            prompt_text: prompt.into(),
            ratio: None,
            duration: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn duration(mut self, secs: u8) -> Self {
        self.duration = Some(secs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn content_moderation(mut self, cm: ContentModeration) -> Self {
        self.content_moderation = Some(cm);
        self
    }
}

// ── Video to Video ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub prompt_video: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

impl VideoToVideoRequest {
    pub fn new(model: VideoModel, prompt: impl Into<String>, video: MediaInput) -> Self {
        Self {
            model,
            prompt_text: prompt.into(),
            prompt_video: video,
            ratio: None,
            duration: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn duration(mut self, secs: u8) -> Self {
        self.duration = Some(secs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn content_moderation(mut self, cm: ContentModeration) -> Self {
        self.content_moderation = Some(cm);
        self
    }
}

// ── Text to Image ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageRequest {
    pub model: ImageModel,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

impl TextToImageRequest {
    pub fn new(model: ImageModel, prompt: impl Into<String>) -> Self {
        Self {
            model,
            prompt_text: prompt.into(),
            ratio: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Character Performance ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPerformanceRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub prompt_image: MediaInput,
    pub prompt_video: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl CharacterPerformanceRequest {
    pub fn new(
        model: VideoModel,
        prompt: impl Into<String>,
        image: MediaInput,
        video: MediaInput,
    ) -> Self {
        Self {
            model,
            prompt_text: prompt.into(),
            prompt_image: image,
            prompt_video: video,
            ratio: None,
            duration: None,
            seed: None,
        }
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn duration(mut self, secs: u8) -> Self {
        self.duration = Some(secs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Sound Effect ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundEffectRequest {
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl SoundEffectRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt_text: prompt.into(),
            duration: None,
            seed: None,
        }
    }

    pub fn duration(mut self, secs: u8) -> Self {
        self.duration = Some(secs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Speech to Speech ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechToSpeechRequest {
    pub audio: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl SpeechToSpeechRequest {
    pub fn new(audio: MediaInput) -> Self {
        Self {
            audio,
            voice_id: None,
            seed: None,
        }
    }

    pub fn voice_id(mut self, id: impl Into<String>) -> Self {
        self.voice_id = Some(id.into());
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Text to Speech ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextToSpeechRequest {
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl TextToSpeechRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt_text: prompt.into(),
            voice_id: None,
            seed: None,
        }
    }

    pub fn voice_id(mut self, id: impl Into<String>) -> Self {
        self.voice_id = Some(id.into());
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Voice Dubbing ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceDubbingRequest {
    pub audio: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl VoiceDubbingRequest {
    pub fn new(audio: MediaInput) -> Self {
        Self {
            audio,
            target_language: None,
            seed: None,
        }
    }

    pub fn target_language(mut self, lang: impl Into<String>) -> Self {
        self.target_language = Some(lang.into());
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Voice Isolation ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceIsolationRequest {
    pub audio: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl VoiceIsolationRequest {
    pub fn new(audio: MediaInput) -> Self {
        Self { audio, seed: None }
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ── Upload ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadRequest {
    pub filename: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadResponse {
    pub id: String,
    pub upload_url: String,
}
