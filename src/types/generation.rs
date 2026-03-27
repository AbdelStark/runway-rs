use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::RunwayError;

use super::common::ContentModeration;
#[cfg(feature = "unstable-endpoints")]
use super::media::MediaInput;
use super::media::{PromptFramePosition, PromptImageInput};
use super::models::{ImageModel, ImageRatio, VideoModel, VideoRatio};

fn validate_prompt_text(prompt_text: &str, field_name: &str) -> Result<(), RunwayError> {
    if prompt_text.trim().is_empty() {
        return Err(RunwayError::Validation {
            message: format!("{field_name} cannot be empty"),
        });
    }

    if prompt_text.encode_utf16().count() > 1000 {
        return Err(RunwayError::Validation {
            message: format!("{field_name} must be at most 1000 UTF-16 code units"),
        });
    }

    Ok(())
}

fn validate_duration_range(
    duration: u8,
    min: u8,
    max: u8,
    field_name: &str,
) -> Result<(), RunwayError> {
    if (min..=max).contains(&duration) {
        Ok(())
    } else {
        Err(RunwayError::Validation {
            message: format!("{field_name} must be between {min} and {max} seconds"),
        })
    }
}

fn validate_allowed_duration(
    duration: u8,
    allowed: &[u8],
    field_name: &str,
) -> Result<(), RunwayError> {
    if allowed.contains(&duration) {
        Ok(())
    } else {
        Err(RunwayError::Validation {
            message: format!("{field_name} must be one of {:?}", allowed),
        })
    }
}

fn validate_video_ratio(
    ratio: VideoRatio,
    allowed: &[VideoRatio],
    field_name: &str,
) -> Result<(), RunwayError> {
    if allowed.contains(&ratio) {
        Ok(())
    } else {
        let allowed = allowed
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Err(RunwayError::Validation {
            message: format!("{field_name} must be one of [{allowed}]"),
        })
    }
}

fn validate_image_ratio(
    ratio: ImageRatio,
    allowed: &[ImageRatio],
    field_name: &str,
) -> Result<(), RunwayError> {
    if allowed.contains(&ratio) {
        Ok(())
    } else {
        let allowed = allowed
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Err(RunwayError::Validation {
            message: format!("{field_name} must be one of [{allowed}]"),
        })
    }
}

fn validate_prompt_image_frames(
    prompt_image: &PromptImageInput,
    allow_last_frame: bool,
    require_first_if_frames: bool,
) -> Result<(), RunwayError> {
    match prompt_image {
        PromptImageInput::Uri(uri) => {
            if uri.trim().is_empty() {
                return Err(RunwayError::Validation {
                    message: "promptImage cannot be empty".into(),
                });
            }
        }
        PromptImageInput::Frames(frames) => {
            if frames.is_empty() {
                return Err(RunwayError::Validation {
                    message: "promptImage frames cannot be empty".into(),
                });
            }

            if frames.len() > 2 {
                return Err(RunwayError::Validation {
                    message: "promptImage may contain at most 2 frames".into(),
                });
            }

            let mut seen_first = false;
            let mut seen_last = false;

            for frame in frames {
                if frame.uri.trim().is_empty() {
                    return Err(RunwayError::Validation {
                        message: "promptImage frame URIs cannot be empty".into(),
                    });
                }

                match frame.position {
                    PromptFramePosition::First => {
                        if seen_first {
                            return Err(RunwayError::Validation {
                                message: "promptImage may contain only one first frame".into(),
                            });
                        }
                        seen_first = true;
                    }
                    PromptFramePosition::Last => {
                        if !allow_last_frame {
                            return Err(RunwayError::Validation {
                                message: "this model does not support a last prompt frame".into(),
                            });
                        }
                        if seen_last {
                            return Err(RunwayError::Validation {
                                message: "promptImage may contain only one last frame".into(),
                            });
                        }
                        seen_last = true;
                    }
                }
            }

            if require_first_if_frames && !seen_first {
                return Err(RunwayError::Validation {
                    message:
                        "this model requires a first prompt frame when promptImage is an array"
                            .into(),
                });
            }
        }
    }

    Ok(())
}

fn validate_media_uri(uri: &str, field_name: &str) -> Result<(), RunwayError> {
    if uri.trim().is_empty() {
        return Err(RunwayError::Validation {
            message: format!("{field_name} cannot be empty"),
        });
    }
    Ok(())
}

// ── Text to Video ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextToVideoRequest {
    Gen45(TextToVideoGen45Request),
    Veo31(TextToVideoVeo31Request),
    Veo31Fast(TextToVideoVeo31FastRequest),
    Veo3(TextToVideoVeo3Request),
}

impl TextToVideoRequest {
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Gen45(request) => request.validate(),
            Self::Veo31(request) => request.validate(),
            Self::Veo31Fast(request) => request.validate(),
            Self::Veo3(request) => request.validate(),
        }
    }
}

impl From<TextToVideoGen45Request> for TextToVideoRequest {
    fn from(value: TextToVideoGen45Request) -> Self {
        Self::Gen45(value)
    }
}

impl From<TextToVideoVeo31Request> for TextToVideoRequest {
    fn from(value: TextToVideoVeo31Request) -> Self {
        Self::Veo31(value)
    }
}

impl From<TextToVideoVeo31FastRequest> for TextToVideoRequest {
    fn from(value: TextToVideoVeo31FastRequest) -> Self {
        Self::Veo31Fast(value)
    }
}

impl From<TextToVideoVeo3Request> for TextToVideoRequest {
    fn from(value: TextToVideoVeo3Request) -> Self {
        Self::Veo3(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoGen45Request {
    pub duration: u8,
    pub model: VideoModel,
    pub prompt_text: String,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl TextToVideoGen45Request {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio, duration: u8) -> Self {
        Self {
            duration,
            model: VideoModel::Gen45,
            prompt_text: prompt_text.into(),
            ratio,
            content_moderation: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Gen45 {
            return Err(RunwayError::Validation {
                message: "TextToVideoGen45Request must use model gen4.5".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_duration_range(self.duration, 2, 10, "duration")?;
        validate_video_ratio(
            self.ratio,
            &[VideoRatio::Landscape, VideoRatio::Portrait],
            "ratio",
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoVeo31Request {
    pub model: VideoModel,
    pub prompt_text: String,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
}

impl TextToVideoVeo31Request {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31,
            prompt_text: prompt_text.into(),
            ratio,
            audio: None,
            duration: None,
        }
    }

    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo31 {
            return Err(RunwayError::Validation {
                message: "TextToVideoVeo31Request must use model veo3.1".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )?;
        if let Some(duration) = self.duration {
            validate_allowed_duration(duration, &[4, 6, 8], "duration")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoVeo31FastRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
}

impl TextToVideoVeo31FastRequest {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31Fast,
            prompt_text: prompt_text.into(),
            ratio,
            audio: None,
            duration: None,
        }
    }

    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo31Fast {
            return Err(RunwayError::Validation {
                message: "TextToVideoVeo31FastRequest must use model veo3.1_fast".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )?;
        if let Some(duration) = self.duration {
            validate_allowed_duration(duration, &[4, 6, 8], "duration")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoVeo3Request {
    pub duration: u8,
    pub model: VideoModel,
    pub prompt_text: String,
    pub ratio: VideoRatio,
}

impl TextToVideoVeo3Request {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            duration: 8,
            model: VideoModel::Veo3,
            prompt_text: prompt_text.into(),
            ratio,
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo3 {
            return Err(RunwayError::Validation {
                message: "TextToVideoVeo3Request must use model veo3".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        if self.duration != 8 {
            return Err(RunwayError::Validation {
                message: "veo3 text-to-video duration must be 8 seconds".into(),
            });
        }
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )
    }
}

// ── Image to Video ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ImageToVideoRequest {
    Gen45(ImageToVideoGen45Request),
    Gen4Turbo(ImageToVideoGen4TurboRequest),
    Gen3aTurbo(ImageToVideoGen3aTurboRequest),
    Veo31(ImageToVideoVeo31Request),
    Veo31Fast(ImageToVideoVeo31FastRequest),
    Veo3(ImageToVideoVeo3Request),
}

impl ImageToVideoRequest {
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Gen45(request) => request.validate(),
            Self::Gen4Turbo(request) => request.validate(),
            Self::Gen3aTurbo(request) => request.validate(),
            Self::Veo31(request) => request.validate(),
            Self::Veo31Fast(request) => request.validate(),
            Self::Veo3(request) => request.validate(),
        }
    }
}

impl From<ImageToVideoGen45Request> for ImageToVideoRequest {
    fn from(value: ImageToVideoGen45Request) -> Self {
        Self::Gen45(value)
    }
}

impl From<ImageToVideoGen4TurboRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoGen4TurboRequest) -> Self {
        Self::Gen4Turbo(value)
    }
}

impl From<ImageToVideoGen3aTurboRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoGen3aTurboRequest) -> Self {
        Self::Gen3aTurbo(value)
    }
}

impl From<ImageToVideoVeo31Request> for ImageToVideoRequest {
    fn from(value: ImageToVideoVeo31Request) -> Self {
        Self::Veo31(value)
    }
}

impl From<ImageToVideoVeo31FastRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoVeo31FastRequest) -> Self {
        Self::Veo31Fast(value)
    }
}

impl From<ImageToVideoVeo3Request> for ImageToVideoRequest {
    fn from(value: ImageToVideoVeo3Request) -> Self {
        Self::Veo3(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoGen45Request {
    pub duration: u8,
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub prompt_text: String,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl ImageToVideoGen45Request {
    pub fn new(
        prompt_text: impl Into<String>,
        prompt_image: impl Into<PromptImageInput>,
        ratio: VideoRatio,
        duration: u8,
    ) -> Self {
        Self {
            duration,
            model: VideoModel::Gen45,
            prompt_image: prompt_image.into(),
            prompt_text: prompt_text.into(),
            ratio,
            content_moderation: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Gen45 {
            return Err(RunwayError::Validation {
                message: "ImageToVideoGen45Request must use model gen4.5".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_duration_range(self.duration, 2, 10, "duration")?;
        validate_prompt_image_frames(&self.prompt_image, false, true)?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::Wide,
                VideoRatio::Square,
                VideoRatio::Tall,
                VideoRatio::Ultrawide,
            ],
            "ratio",
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoGen4TurboRequest {
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl ImageToVideoGen4TurboRequest {
    pub fn new(prompt_image: impl Into<PromptImageInput>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Gen4Turbo,
            prompt_image: prompt_image.into(),
            ratio,
            content_moderation: None,
            duration: None,
            prompt_text: None,
            seed: None,
        }
    }

    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Gen4Turbo {
            return Err(RunwayError::Validation {
                message: "ImageToVideoGen4TurboRequest must use model gen4_turbo".into(),
            });
        }
        if let Some(prompt_text) = &self.prompt_text {
            validate_prompt_text(prompt_text, "promptText")?;
        }
        validate_prompt_image_frames(&self.prompt_image, false, true)?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::Wide,
                VideoRatio::Tall,
                VideoRatio::Square,
                VideoRatio::Ultrawide,
            ],
            "ratio",
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoGen3aTurboRequest {
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl ImageToVideoGen3aTurboRequest {
    pub fn new(prompt_text: impl Into<String>, prompt_image: impl Into<PromptImageInput>) -> Self {
        Self {
            model: VideoModel::Gen3aTurbo,
            prompt_image: prompt_image.into(),
            prompt_text: prompt_text.into(),
            content_moderation: None,
            duration: None,
            ratio: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Gen3aTurbo {
            return Err(RunwayError::Validation {
                message: "ImageToVideoGen3aTurboRequest must use model gen3a_turbo".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_prompt_image_frames(&self.prompt_image, true, false)?;
        if let Some(duration) = self.duration {
            validate_allowed_duration(duration, &[5, 10], "duration")?;
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[VideoRatio::Gen3Landscape, VideoRatio::Gen3Portrait],
                "ratio",
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoVeo31Request {
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
}

impl ImageToVideoVeo31Request {
    pub fn new(prompt_image: impl Into<PromptImageInput>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31,
            prompt_image: prompt_image.into(),
            ratio,
            audio: None,
            duration: None,
            prompt_text: None,
        }
    }

    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo31 {
            return Err(RunwayError::Validation {
                message: "ImageToVideoVeo31Request must use model veo3.1".into(),
            });
        }
        if let Some(prompt_text) = &self.prompt_text {
            validate_prompt_text(prompt_text, "promptText")?;
        }
        validate_prompt_image_frames(&self.prompt_image, true, true)?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )?;
        if let Some(duration) = self.duration {
            validate_allowed_duration(duration, &[4, 6, 8], "duration")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoVeo31FastRequest {
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
}

impl ImageToVideoVeo31FastRequest {
    pub fn new(prompt_image: impl Into<PromptImageInput>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31Fast,
            prompt_image: prompt_image.into(),
            ratio,
            audio: None,
            duration: None,
            prompt_text: None,
        }
    }

    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo31Fast {
            return Err(RunwayError::Validation {
                message: "ImageToVideoVeo31FastRequest must use model veo3.1_fast".into(),
            });
        }
        if let Some(prompt_text) = &self.prompt_text {
            validate_prompt_text(prompt_text, "promptText")?;
        }
        validate_prompt_image_frames(&self.prompt_image, true, true)?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )?;
        if let Some(duration) = self.duration {
            validate_allowed_duration(duration, &[4, 6, 8], "duration")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoVeo3Request {
    pub duration: u8,
    pub model: VideoModel,
    pub prompt_image: PromptImageInput,
    pub ratio: VideoRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
}

impl ImageToVideoVeo3Request {
    pub fn new(prompt_image: impl Into<PromptImageInput>, ratio: VideoRatio) -> Self {
        Self {
            duration: 8,
            model: VideoModel::Veo3,
            prompt_image: prompt_image.into(),
            ratio,
            prompt_text: None,
        }
    }

    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Veo3 {
            return Err(RunwayError::Validation {
                message: "ImageToVideoVeo3Request must use model veo3".into(),
            });
        }
        if self.duration != 8 {
            return Err(RunwayError::Validation {
                message: "veo3 image-to-video duration must be 8 seconds".into(),
            });
        }
        if let Some(prompt_text) = &self.prompt_text {
            validate_prompt_text(prompt_text, "promptText")?;
        }
        validate_prompt_image_frames(&self.prompt_image, false, true)?;
        validate_video_ratio(
            self.ratio,
            &[
                VideoRatio::Landscape,
                VideoRatio::Portrait,
                VideoRatio::HdPortrait,
                VideoRatio::HdLandscape,
            ],
            "ratio",
        )
    }
}

// ── Video to Video ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoRequest {
    pub model: VideoModel,
    pub prompt_text: String,
    pub video_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<VideoToVideoReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl VideoToVideoRequest {
    pub fn new(prompt_text: impl Into<String>, video_uri: impl Into<String>) -> Self {
        Self {
            model: VideoModel::Gen4Aleph,
            prompt_text: prompt_text.into(),
            video_uri: video_uri.into(),
            content_moderation: None,
            ratio: None,
            references: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn references(mut self, references: Vec<VideoToVideoReference>) -> Self {
        self.references = Some(references);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Gen4Aleph {
            return Err(RunwayError::Validation {
                message: "VideoToVideoRequest must use model gen4_aleph".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_media_uri(&self.video_uri, "videoUri")?;
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[
                    VideoRatio::Landscape,
                    VideoRatio::Portrait,
                    VideoRatio::Wide,
                    VideoRatio::Square,
                    VideoRatio::Tall,
                    VideoRatio::Ultrawide,
                    VideoRatio::SdLandscape,
                    VideoRatio::SdClassic,
                ],
                "ratio",
            )?;
        }
        if let Some(references) = &self.references {
            if references.len() > 1 {
                return Err(RunwayError::Validation {
                    message: "video-to-video currently supports at most one reference image".into(),
                });
            }
            for reference in references {
                reference.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoReference {
    #[serde(rename = "type")]
    pub reference_type: VideoToVideoReferenceType,
    pub uri: String,
}

impl VideoToVideoReference {
    pub fn image(uri: impl Into<String>) -> Self {
        Self {
            reference_type: VideoToVideoReferenceType::Image,
            uri: uri.into(),
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "references[].uri")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VideoToVideoReferenceType {
    Image,
}

// ── Text to Image ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextToImageRequest {
    Gen4ImageTurbo(TextToImageGen4ImageTurboRequest),
    Gen4Image(TextToImageGen4ImageRequest),
    Gemini25Flash(TextToImageGemini25FlashRequest),
}

impl TextToImageRequest {
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Gen4ImageTurbo(request) => request.validate(),
            Self::Gen4Image(request) => request.validate(),
            Self::Gemini25Flash(request) => request.validate(),
        }
    }
}

impl From<TextToImageGen4ImageTurboRequest> for TextToImageRequest {
    fn from(value: TextToImageGen4ImageTurboRequest) -> Self {
        Self::Gen4ImageTurbo(value)
    }
}

impl From<TextToImageGen4ImageRequest> for TextToImageRequest {
    fn from(value: TextToImageGen4ImageRequest) -> Self {
        Self::Gen4Image(value)
    }
}

impl From<TextToImageGemini25FlashRequest> for TextToImageRequest {
    fn from(value: TextToImageGemini25FlashRequest) -> Self {
        Self::Gemini25Flash(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGen4ImageTurboRequest {
    pub model: ImageModel,
    pub prompt_text: String,
    pub ratio: ImageRatio,
    pub reference_images: Vec<TextToImageReferenceImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl TextToImageGen4ImageTurboRequest {
    pub fn new(
        prompt_text: impl Into<String>,
        ratio: ImageRatio,
        reference_images: Vec<TextToImageReferenceImage>,
    ) -> Self {
        Self {
            model: ImageModel::Gen4ImageTurbo,
            prompt_text: prompt_text.into(),
            ratio,
            reference_images,
            content_moderation: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != ImageModel::Gen4ImageTurbo {
            return Err(RunwayError::Validation {
                message: "TextToImageGen4ImageTurboRequest must use model gen4_image_turbo".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_image_ratio(
            self.ratio,
            &[
                ImageRatio::Square1024,
                ImageRatio::Square1080,
                ImageRatio::Wide1168x880,
                ImageRatio::Landscape1360x768,
                ImageRatio::Landscape1440x1080,
                ImageRatio::Portrait1080x1440,
                ImageRatio::Ultrawide1808x768,
                ImageRatio::HdLandscape,
                ImageRatio::HdPortrait,
                ImageRatio::Ultrawide2112x912,
                ImageRatio::Landscape,
                ImageRatio::Portrait,
                ImageRatio::Square720,
                ImageRatio::Landscape960x720,
                ImageRatio::Portrait720x960,
                ImageRatio::Ultrawide1680x720,
            ],
            "ratio",
        )?;
        if !(1..=3).contains(&self.reference_images.len()) {
            return Err(RunwayError::Validation {
                message: "gen4_image_turbo requires 1 to 3 referenceImages".into(),
            });
        }
        for reference_image in &self.reference_images {
            reference_image.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGen4ImageRequest {
    pub model: ImageModel,
    pub prompt_text: String,
    pub ratio: ImageRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<TextToImageReferenceImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl TextToImageGen4ImageRequest {
    pub fn new(prompt_text: impl Into<String>, ratio: ImageRatio) -> Self {
        Self {
            model: ImageModel::Gen4Image,
            prompt_text: prompt_text.into(),
            ratio,
            content_moderation: None,
            reference_images: None,
            seed: None,
        }
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn reference_images(mut self, reference_images: Vec<TextToImageReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != ImageModel::Gen4Image {
            return Err(RunwayError::Validation {
                message: "TextToImageGen4ImageRequest must use model gen4_image".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_image_ratio(
            self.ratio,
            &[
                ImageRatio::Square1024,
                ImageRatio::Square1080,
                ImageRatio::Wide1168x880,
                ImageRatio::Landscape1360x768,
                ImageRatio::Landscape1440x1080,
                ImageRatio::Portrait1080x1440,
                ImageRatio::Ultrawide1808x768,
                ImageRatio::HdLandscape,
                ImageRatio::HdPortrait,
                ImageRatio::Ultrawide2112x912,
                ImageRatio::Landscape,
                ImageRatio::Portrait,
                ImageRatio::Square720,
                ImageRatio::Landscape960x720,
                ImageRatio::Portrait720x960,
                ImageRatio::Ultrawide1680x720,
            ],
            "ratio",
        )?;
        if let Some(reference_images) = &self.reference_images {
            if reference_images.len() > 3 {
                return Err(RunwayError::Validation {
                    message: "gen4_image supports up to 3 referenceImages".into(),
                });
            }
            for reference_image in reference_images {
                reference_image.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGemini25FlashRequest {
    pub model: ImageModel,
    pub prompt_text: String,
    pub ratio: ImageRatio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<TextToImageReferenceImage>>,
}

impl TextToImageGemini25FlashRequest {
    pub fn new(prompt_text: impl Into<String>, ratio: ImageRatio) -> Self {
        Self {
            model: ImageModel::Gemini25Flash,
            prompt_text: prompt_text.into(),
            ratio,
            reference_images: None,
        }
    }

    pub fn reference_images(mut self, reference_images: Vec<TextToImageReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != ImageModel::Gemini25Flash {
            return Err(RunwayError::Validation {
                message: "TextToImageGemini25FlashRequest must use model gemini_2.5_flash".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        validate_image_ratio(
            self.ratio,
            &[
                ImageRatio::GeminiLandscape1344x768,
                ImageRatio::GeminiPortrait768x1344,
                ImageRatio::Square1024,
                ImageRatio::GeminiLandscape1184x864,
                ImageRatio::GeminiPortrait864x1184,
                ImageRatio::GeminiUltrawide1536x672,
                ImageRatio::GeminiPortrait832x1248,
                ImageRatio::GeminiLandscape1248x832,
                ImageRatio::GeminiPortrait896x1152,
                ImageRatio::GeminiLandscape1152x896,
            ],
            "ratio",
        )?;
        if let Some(reference_images) = &self.reference_images {
            if reference_images.len() > 3 {
                return Err(RunwayError::Validation {
                    message: "gemini_2.5_flash supports up to 3 referenceImages".into(),
                });
            }
            for reference_image in reference_images {
                reference_image.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageReferenceImage {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl TextToImageReferenceImage {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            tag: None,
        }
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "referenceImages[].uri")
    }
}

// ── Character Performance ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPerformanceRequest {
    pub character: CharacterPerformanceCharacter,
    pub model: CharacterPerformanceModel,
    pub reference: CharacterPerformanceReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_intensity: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

impl CharacterPerformanceRequest {
    pub fn new(
        character: CharacterPerformanceCharacter,
        reference: CharacterPerformanceReference,
    ) -> Self {
        Self {
            character,
            model: CharacterPerformanceModel::ActTwo,
            reference,
            body_control: None,
            content_moderation: None,
            expression_intensity: None,
            ratio: None,
            seed: None,
        }
    }

    pub fn body_control(mut self, body_control: bool) -> Self {
        self.body_control = Some(body_control);
        self
    }

    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    pub fn expression_intensity(mut self, expression_intensity: u8) -> Self {
        self.expression_intensity = Some(expression_intensity);
        self
    }

    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != CharacterPerformanceModel::ActTwo {
            return Err(RunwayError::Validation {
                message: "CharacterPerformanceRequest must use model act_two".into(),
            });
        }
        self.character.validate()?;
        self.reference.validate()?;
        if let Some(expression_intensity) = self.expression_intensity {
            if !(1..=5).contains(&expression_intensity) {
                return Err(RunwayError::Validation {
                    message: "expressionIntensity must be between 1 and 5".into(),
                });
            }
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[
                    VideoRatio::Landscape,
                    VideoRatio::Portrait,
                    VideoRatio::Square,
                    VideoRatio::Wide,
                    VideoRatio::Tall,
                    VideoRatio::Ultrawide,
                ],
                "ratio",
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CharacterPerformanceModel {
    #[serde(rename = "act_two")]
    ActTwo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPerformanceCharacter {
    #[serde(rename = "type")]
    pub character_type: CharacterPerformanceCharacterType,
    pub uri: String,
}

impl CharacterPerformanceCharacter {
    pub fn image(uri: impl Into<String>) -> Self {
        Self {
            character_type: CharacterPerformanceCharacterType::Image,
            uri: uri.into(),
        }
    }

    pub fn video(uri: impl Into<String>) -> Self {
        Self {
            character_type: CharacterPerformanceCharacterType::Video,
            uri: uri.into(),
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "character.uri")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CharacterPerformanceCharacterType {
    Image,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPerformanceReference {
    #[serde(rename = "type")]
    pub reference_type: CharacterPerformanceReferenceType,
    pub uri: String,
}

impl CharacterPerformanceReference {
    pub fn video(uri: impl Into<String>) -> Self {
        Self {
            reference_type: CharacterPerformanceReferenceType::Video,
            uri: uri.into(),
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "reference.uri")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CharacterPerformanceReferenceType {
    Video,
}

// ── Sound Effect ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SoundEffectRequest {
    pub model: SoundEffectModel,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_output: Option<bool>,
}

impl SoundEffectRequest {
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: SoundEffectModel::ElevenTextToSoundV2,
            prompt_text: prompt_text.into(),
            duration: None,
            loop_output: None,
        }
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn loop_output(mut self, loop_output: bool) -> Self {
        self.loop_output = Some(loop_output);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != SoundEffectModel::ElevenTextToSoundV2 {
            return Err(RunwayError::Validation {
                message: "SoundEffectRequest must use model eleven_text_to_sound_v2".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        if let Some(duration) = self.duration {
            if !(0.5..=30.0).contains(&duration) {
                return Err(RunwayError::Validation {
                    message: "duration must be between 0.5 and 30 seconds".into(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SoundEffectModel {
    #[serde(rename = "eleven_text_to_sound_v2")]
    ElevenTextToSoundV2,
}

// ── Speech to Speech ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpeechToSpeechRequest {
    pub media: SpeechToSpeechMedia,
    pub model: SpeechToSpeechModel,
    pub voice: RunwayPresetVoice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}

impl SpeechToSpeechRequest {
    pub fn new(media: SpeechToSpeechMedia, voice: RunwayPresetVoice) -> Self {
        Self {
            media,
            model: SpeechToSpeechModel::ElevenMultilingualStsV2,
            voice,
            remove_background_noise: None,
        }
    }

    pub fn remove_background_noise(mut self, remove_background_noise: bool) -> Self {
        self.remove_background_noise = Some(remove_background_noise);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != SpeechToSpeechModel::ElevenMultilingualStsV2 {
            return Err(RunwayError::Validation {
                message: "SpeechToSpeechRequest must use model eleven_multilingual_sts_v2".into(),
            });
        }
        self.media.validate()?;
        self.voice.validate()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeechToSpeechModel {
    #[serde(rename = "eleven_multilingual_sts_v2")]
    ElevenMultilingualStsV2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpeechToSpeechMedia {
    #[serde(rename = "type")]
    pub media_type: SpeechToSpeechMediaType,
    pub uri: String,
}

impl SpeechToSpeechMedia {
    pub fn audio(uri: impl Into<String>) -> Self {
        Self {
            media_type: SpeechToSpeechMediaType::Audio,
            uri: uri.into(),
        }
    }

    pub fn video(uri: impl Into<String>) -> Self {
        Self {
            media_type: SpeechToSpeechMediaType::Video,
            uri: uri.into(),
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "media.uri")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SpeechToSpeechMediaType {
    Audio,
    Video,
}

// ── Text to Speech ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TextToSpeechRequest {
    pub model: TextToSpeechModel,
    pub prompt_text: String,
    pub voice: RunwayPresetVoice,
}

impl TextToSpeechRequest {
    pub fn new(prompt_text: impl Into<String>, voice: RunwayPresetVoice) -> Self {
        Self {
            model: TextToSpeechModel::ElevenMultilingualV2,
            prompt_text: prompt_text.into(),
            voice,
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != TextToSpeechModel::ElevenMultilingualV2 {
            return Err(RunwayError::Validation {
                message: "TextToSpeechRequest must use model eleven_multilingual_v2".into(),
            });
        }
        validate_prompt_text(&self.prompt_text, "promptText")?;
        self.voice.validate()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TextToSpeechModel {
    #[serde(rename = "eleven_multilingual_v2")]
    ElevenMultilingualV2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RunwayPresetVoice {
    pub preset_id: RunwayPresetVoiceId,
    #[serde(rename = "type")]
    pub voice_type: RunwayPresetVoiceType,
}

impl RunwayPresetVoice {
    pub fn new(preset_id: RunwayPresetVoiceId) -> Self {
        Self {
            preset_id,
            voice_type: RunwayPresetVoiceType::RunwayPreset,
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.voice_type != RunwayPresetVoiceType::RunwayPreset {
            return Err(RunwayError::Validation {
                message: "voice.type must be runway-preset".into(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RunwayPresetVoiceType {
    #[serde(rename = "runway-preset")]
    RunwayPreset,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RunwayPresetVoiceId {
    Maya,
    Arjun,
    Serene,
    Bernard,
    Billy,
    Mark,
    Clint,
    Mabel,
    Chad,
    Leslie,
    Eleanor,
    Elias,
    Elliot,
    Grungle,
    Brodie,
    Sandra,
    Kirk,
    Kylie,
    Lara,
    Lisa,
    Malachi,
    Marlene,
    Martin,
    Miriam,
    Monster,
    Paula,
    Pip,
    Rusty,
    Ragnar,
    Xylar,
    Maggie,
    Jack,
    Katie,
    Noah,
    James,
    Rina,
    Ella,
    Mariah,
    Frank,
    Claudia,
    Niki,
    Vincent,
    Kendrick,
    Myrna,
    Tom,
    Wanda,
    Benjamin,
    Kiana,
    Rachel,
}

// ── Voice Dubbing ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoiceDubbingRequest {
    pub audio_uri: String,
    pub model: VoiceDubbingModel,
    pub target_lang: VoiceDubbingLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_voice_cloning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_background_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<u32>,
}

impl VoiceDubbingRequest {
    pub fn new(audio_uri: impl Into<String>, target_lang: VoiceDubbingLanguage) -> Self {
        Self {
            audio_uri: audio_uri.into(),
            model: VoiceDubbingModel::ElevenVoiceDubbing,
            target_lang,
            disable_voice_cloning: None,
            drop_background_audio: None,
            num_speakers: None,
        }
    }

    pub fn disable_voice_cloning(mut self, disable_voice_cloning: bool) -> Self {
        self.disable_voice_cloning = Some(disable_voice_cloning);
        self
    }

    pub fn drop_background_audio(mut self, drop_background_audio: bool) -> Self {
        self.drop_background_audio = Some(drop_background_audio);
        self
    }

    pub fn num_speakers(mut self, num_speakers: u32) -> Self {
        self.num_speakers = Some(num_speakers);
        self
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VoiceDubbingModel::ElevenVoiceDubbing {
            return Err(RunwayError::Validation {
                message: "VoiceDubbingRequest must use model eleven_voice_dubbing".into(),
            });
        }
        validate_media_uri(&self.audio_uri, "audioUri")?;
        if let Some(num_speakers) = self.num_speakers {
            if num_speakers == 0 {
                return Err(RunwayError::Validation {
                    message: "numSpeakers must be greater than 0".into(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceDubbingModel {
    #[serde(rename = "eleven_voice_dubbing")]
    ElevenVoiceDubbing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceDubbingLanguage {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "fil")]
    Fil,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "ta")]
    Ta,
}

// ── Voice Isolation ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoiceIsolationRequest {
    pub audio_uri: String,
    pub model: VoiceIsolationModel,
}

impl VoiceIsolationRequest {
    pub fn new(audio_uri: impl Into<String>) -> Self {
        Self {
            audio_uri: audio_uri.into(),
            model: VoiceIsolationModel::ElevenVoiceIsolation,
        }
    }

    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VoiceIsolationModel::ElevenVoiceIsolation {
            return Err(RunwayError::Validation {
                message: "VoiceIsolationRequest must use model eleven_voice_isolation".into(),
            });
        }
        validate_media_uri(&self.audio_uri, "audioUri")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceIsolationModel {
    #[serde(rename = "eleven_voice_isolation")]
    ElevenVoiceIsolation,
}

// ── Unofficial Extensions ───────────────────────────────────────────────────

#[cfg(feature = "unstable-endpoints")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LipSyncRequest {
    pub model: VideoModel,
    pub prompt_video: MediaInput,
    pub prompt_audio: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

#[cfg(feature = "unstable-endpoints")]
impl LipSyncRequest {
    pub fn new(model: VideoModel, video: MediaInput, audio: MediaInput) -> Self {
        Self {
            model,
            prompt_video: video,
            prompt_audio: audio,
            max_duration: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn max_duration(mut self, secs: u8) -> Self {
        self.max_duration = Some(secs);
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

#[cfg(feature = "unstable-endpoints")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageUpscaleRequest {
    pub model: ImageModel,
    pub prompt_image: MediaInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
}

#[cfg(feature = "unstable-endpoints")]
impl ImageUpscaleRequest {
    pub fn new(model: ImageModel, image: MediaInput) -> Self {
        Self {
            model,
            prompt_image: image,
            resolution: None,
            seed: None,
            content_moderation: None,
        }
    }

    pub fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = Some(resolution);
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

// ── Upload ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UploadType {
    Ephemeral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadRequest {
    pub filename: String,
    #[serde(rename = "type")]
    pub upload_type: UploadType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadResponse {
    pub runway_uri: String,
    pub upload_url: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateEphemeralUploadRequest {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
    pub file_metadata: Option<String>,
}

impl CreateEphemeralUploadRequest {
    pub fn new(filename: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            filename: filename.into(),
            bytes: bytes.into(),
            content_type: None,
            file_metadata: None,
        }
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn file_metadata(mut self, file_metadata: impl Into<String>) -> Self {
        self.file_metadata = Some(file_metadata.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UploadCreateEphemeralResponse {
    pub uri: String,
}
