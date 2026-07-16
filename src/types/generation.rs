use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use crate::error::RunwayError;

use super::common::ContentModeration;
#[cfg(feature = "unstable-endpoints")]
use super::media::MediaInput;
use super::media::{PromptFramePosition, PromptImageInput};
use super::models::{ImageModel, ImageRatio, VideoModel, VideoRatio};

fn validate_prompt_text(prompt_text: &str, field_name: &str) -> Result<(), RunwayError> {
    validate_prompt_text_length(prompt_text, field_name, 1000)
}

fn validate_prompt_text_length(
    prompt_text: &str,
    field_name: &str,
    max_utf16_code_units: usize,
) -> Result<(), RunwayError> {
    if prompt_text.trim().is_empty() {
        return Err(RunwayError::Validation {
            message: format!("{field_name} cannot be empty"),
        });
    }

    validate_utf16_length(prompt_text, field_name, max_utf16_code_units)
}

fn validate_utf16_length(
    value: &str,
    field_name: &str,
    max_utf16_code_units: usize,
) -> Result<(), RunwayError> {
    if value.encode_utf16().count() > max_utf16_code_units {
        return Err(RunwayError::Validation {
            message: format!(
                "{field_name} must be at most {max_utf16_code_units} UTF-16 code units"
            ),
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

struct JsonNumber(f64);

impl Serialize for JsonNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.fract() == 0.0 && self.0 >= i64::MIN as f64 && self.0 <= i64::MAX as f64 {
            serializer.serialize_i64(self.0 as i64)
        } else {
            serializer.serialize_f64(self.0)
        }
    }
}

fn serialize_optional_json_number<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(value) => serializer.serialize_some(&JsonNumber(*value)),
        None => serializer.serialize_none(),
    }
}

const SEEDANCE_ALL_RATIOS: &[VideoRatio] = &[
    VideoRatio::R992x432,
    VideoRatio::R864x496,
    VideoRatio::R752x560,
    VideoRatio::R640x640,
    VideoRatio::R560x752,
    VideoRatio::R496x864,
    VideoRatio::R1470x630,
    VideoRatio::Landscape,
    VideoRatio::R1112x834,
    VideoRatio::Square,
    VideoRatio::R834x1112,
    VideoRatio::Portrait,
    VideoRatio::R2206x946,
    VideoRatio::HdLandscape,
    VideoRatio::R1664x1248,
    VideoRatio::R1440x1440,
    VideoRatio::R1248x1664,
    VideoRatio::HdPortrait,
    VideoRatio::R3840x1646,
    VideoRatio::R3840x2160,
    VideoRatio::R3840x2880,
    VideoRatio::R3840x3840,
    VideoRatio::R2880x3840,
    VideoRatio::R2160x3840,
];

const SEEDANCE_FAST_RATIOS: &[VideoRatio] = &[
    VideoRatio::R992x432,
    VideoRatio::R864x496,
    VideoRatio::R752x560,
    VideoRatio::R640x640,
    VideoRatio::R560x752,
    VideoRatio::R496x864,
    VideoRatio::R1470x630,
    VideoRatio::Landscape,
    VideoRatio::R1112x834,
    VideoRatio::Square,
    VideoRatio::R834x1112,
    VideoRatio::Portrait,
];

/// Audio context supplied to a Seedance 2 generation request.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Seedance2AudioReference {
    /// Reference discriminator. Constructors set this to `audio`.
    #[serde(rename = "type")]
    pub reference_type: Seedance2AudioReferenceType,
    /// URI of the audio reference.
    pub uri: String,
}

impl Seedance2AudioReference {
    /// Create an audio reference.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            reference_type: Seedance2AudioReferenceType::Audio,
            uri: uri.into(),
        }
    }

    fn validate(&self) -> Result<(), RunwayError> {
        if self.reference_type != Seedance2AudioReferenceType::Audio {
            return Err(RunwayError::Validation {
                message: "referenceAudio[].type must be audio".into(),
            });
        }
        validate_media_uri(&self.uri, "referenceAudio[].uri")
    }
}

/// Wire discriminator for a Seedance audio reference.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Seedance2AudioReferenceType {
    /// An audio reference.
    Audio,
}

/// Image context supplied to a Seedance 2 generation request.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Seedance2ImageReference {
    /// URI of the image reference.
    pub uri: String,
}

impl Seedance2ImageReference {
    /// Create an image reference.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }

    fn validate(&self) -> Result<(), RunwayError> {
        validate_media_uri(&self.uri, "references[].uri")
    }
}

/// An image used in a Seedance 2 image-to-video request.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Seedance2PromptImage {
    /// URI of the prompt image.
    pub uri: String,
    /// Optional keyframe position. Omit this for reference-image mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PromptFramePosition>,
}

impl Seedance2PromptImage {
    /// Create an unpositioned reference image.
    pub fn reference(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            position: None,
        }
    }

    /// Create a first-frame keyframe.
    pub fn first(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            position: Some(PromptFramePosition::First),
        }
    }

    /// Create a last-frame keyframe.
    pub fn last(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            position: Some(PromptFramePosition::Last),
        }
    }
}

/// String or structured image input for Seedance 2 image-to-video requests.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Seedance2PromptImageInput {
    /// A single image URI.
    Uri(String),
    /// Positioned keyframes or unpositioned reference images.
    Images(Vec<Seedance2PromptImage>),
}

impl Seedance2PromptImageInput {
    /// Create an array input from keyframes or references.
    pub fn images(images: Vec<Seedance2PromptImage>) -> Self {
        Self::Images(images)
    }
}

impl From<String> for Seedance2PromptImageInput {
    fn from(value: String) -> Self {
        Self::Uri(value)
    }
}

impl From<&str> for Seedance2PromptImageInput {
    fn from(value: &str) -> Self {
        Self::Uri(value.to_owned())
    }
}

impl From<Vec<Seedance2PromptImage>> for Seedance2PromptImageInput {
    fn from(value: Vec<Seedance2PromptImage>) -> Self {
        Self::Images(value)
    }
}

fn validate_seedance_prompt_image(
    prompt_image: &Seedance2PromptImageInput,
) -> Result<(), RunwayError> {
    let Seedance2PromptImageInput::Images(images) = prompt_image else {
        if let Seedance2PromptImageInput::Uri(uri) = prompt_image {
            return validate_media_uri(uri, "promptImage");
        }
        unreachable!();
    };

    for image in images {
        validate_media_uri(&image.uri, "promptImage[].uri")?;
    }

    let positioned = images
        .iter()
        .filter(|image| image.position.is_some())
        .count();
    if positioned == 0 {
        return Ok(());
    }
    if positioned != images.len() {
        return Err(RunwayError::Validation {
            message: "promptImage cannot mix keyframes and unpositioned reference images".into(),
        });
    }
    Ok(())
}

/// Video context supplied to a Seedance 2 generation request.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Seedance2VideoReference {
    /// Reference discriminator. Constructors set this to `video`.
    #[serde(rename = "type")]
    pub reference_type: Seedance2VideoReferenceType,
    /// URI of the video reference.
    pub uri: String,
}

impl Seedance2VideoReference {
    /// Create a video reference.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            reference_type: Seedance2VideoReferenceType::Video,
            uri: uri.into(),
        }
    }

    fn validate(&self) -> Result<(), RunwayError> {
        if self.reference_type != Seedance2VideoReferenceType::Video {
            return Err(RunwayError::Validation {
                message: "referenceVideos[].type must be video".into(),
            });
        }
        validate_media_uri(&self.uri, "referenceVideos[].uri")
    }
}

/// Wire discriminator for a Seedance video reference.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Seedance2VideoReferenceType {
    /// A video reference.
    Video,
}

fn validate_seedance_references(
    reference_audio: Option<&[Seedance2AudioReference]>,
    references: Option<&[Seedance2ImageReference]>,
    reference_videos: Option<&[Seedance2VideoReference]>,
) -> Result<(), RunwayError> {
    if let Some(reference_audio) = reference_audio {
        for reference in reference_audio {
            reference.validate()?;
        }
    }
    if let Some(references) = references {
        if references.len() > 9 {
            return Err(RunwayError::Validation {
                message: "references may contain at most 9 images".into(),
            });
        }
        for reference in references {
            reference.validate()?;
        }
    }
    if let Some(reference_videos) = reference_videos {
        for reference in reference_videos {
            reference.validate()?;
        }
    }
    Ok(())
}

// ── Text to Video ───────────────────────────────────────────────────────────

/// A current text-to-video request discriminated by its `model` field.
///
/// This request union is serialization-only because its wire variants overlap
/// structurally and cannot be deserialized safely with an untagged strategy.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TextToVideoRequest {
    Gen45(TextToVideoGen45Request),
    Veo31(TextToVideoVeo31Request),
    Veo31Fast(TextToVideoVeo31FastRequest),
    /// A Happyhorse 1.0 generation request.
    Happyhorse10(TextToVideoHappyhorse10Request),
    /// A Seedance 2.0 generation request.
    Seedance2(TextToVideoSeedance2Request),
    /// A Seedance 2.0 Fast generation request.
    Seedance2Fast(TextToVideoSeedance2FastRequest),
    /// A Seedance 2.0 Mini generation request.
    Seedance2Mini(TextToVideoSeedance2MiniRequest),
    /// A Gemini Omni Flash generation request.
    GeminiOmniFlash(TextToVideoGeminiOmniFlashRequest),
    Veo3(TextToVideoVeo3Request),
}

impl TextToVideoRequest {
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Gen45(request) => request.validate(),
            Self::Veo31(request) => request.validate(),
            Self::Veo31Fast(request) => request.validate(),
            Self::Happyhorse10(request) => request.validate(),
            Self::Seedance2(request) => request.validate(),
            Self::Seedance2Fast(request) => request.validate(),
            Self::Seedance2Mini(request) => request.validate(),
            Self::GeminiOmniFlash(request) => request.validate(),
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

impl From<TextToVideoHappyhorse10Request> for TextToVideoRequest {
    fn from(value: TextToVideoHappyhorse10Request) -> Self {
        Self::Happyhorse10(value)
    }
}

impl From<TextToVideoSeedance2Request> for TextToVideoRequest {
    fn from(value: TextToVideoSeedance2Request) -> Self {
        Self::Seedance2(value)
    }
}

impl From<TextToVideoSeedance2FastRequest> for TextToVideoRequest {
    fn from(value: TextToVideoSeedance2FastRequest) -> Self {
        Self::Seedance2Fast(value)
    }
}

impl From<TextToVideoSeedance2MiniRequest> for TextToVideoRequest {
    fn from(value: TextToVideoSeedance2MiniRequest) -> Self {
        Self::Seedance2Mini(value)
    }
}

impl From<TextToVideoGeminiOmniFlashRequest> for TextToVideoRequest {
    fn from(value: TextToVideoGeminiOmniFlashRequest) -> Self {
        Self::GeminiOmniFlash(value)
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

impl TextToVideoVeo31Request {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31,
            prompt_text: prompt_text.into(),
            ratio,
            audio: None,
            duration: None,
            negative_prompt: None,
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

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

impl TextToVideoVeo31FastRequest {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            model: VideoModel::Veo31Fast,
            prompt_text: prompt_text.into(),
            ratio,
            audio: None,
            duration: None,
            negative_prompt: None,
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

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
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

/// Request to generate video from text with Happyhorse 1.0.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoHappyhorse10Request {
    /// Model discriminator. Constructors set this to `happyhorse_1_0`.
    pub model: VideoModel,
    /// Description of the requested video, up to 2500 UTF-16 code units.
    pub prompt_text: String,
    /// Optional output duration in seconds.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_json_number"
    )]
    pub duration: Option<f64>,
    /// Optional output resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
}

impl TextToVideoHappyhorse10Request {
    /// Create a Happyhorse 1.0 text-to-video request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: VideoModel::Happyhorse10,
            prompt_text: prompt_text.into(),
            duration: None,
            ratio: None,
        }
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the output resolution.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Validate this request against the official Happyhorse 1.0 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Happyhorse10 {
            return Err(RunwayError::Validation {
                message: "TextToVideoHappyhorse10Request must use model happyhorse_1_0".into(),
            });
        }
        validate_prompt_text_length(&self.prompt_text, "promptText", 2500)?;
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[
                    VideoRatio::Landscape,
                    VideoRatio::Portrait,
                    VideoRatio::Square,
                    VideoRatio::R1108x832,
                    VideoRatio::R832x1108,
                    VideoRatio::HdLandscape,
                    VideoRatio::HdPortrait,
                    VideoRatio::R1440x1440,
                    VideoRatio::R1662x1248,
                    VideoRatio::R1248x1662,
                ],
                "ratio",
            )?;
        }
        Ok(())
    }
}

/// Request to generate video from text with Seedance 2.0.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoSeedance2Request {
    /// Model discriminator. Constructors set this to `seedance2`.
    pub model: VideoModel,
    /// Description of the requested video, up to 3500 UTF-16 code units.
    pub prompt_text: String,
    /// Whether the generated video should include audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Optional output duration in seconds.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_json_number"
    )]
    pub duration: Option<f64>,
    /// Optional output resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    /// Optional audio references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_audio: Option<Vec<Seedance2AudioReference>>,
    /// Optional image references, up to nine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Seedance2ImageReference>>,
    /// Optional video references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_videos: Option<Vec<Seedance2VideoReference>>,
}

impl TextToVideoSeedance2Request {
    /// Create a Seedance 2.0 text-to-video request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: VideoModel::Seedance2,
            prompt_text: prompt_text.into(),
            audio: None,
            duration: None,
            ratio: None,
            reference_audio: None,
            references: None,
            reference_videos: None,
        }
    }

    /// Configure whether to generate audio.
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the output resolution.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Set audio references.
    pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
        self.reference_audio = Some(references);
        self
    }

    /// Set image references.
    pub fn references(mut self, references: Vec<Seedance2ImageReference>) -> Self {
        self.references = Some(references);
        self
    }

    /// Set video references.
    pub fn reference_videos(mut self, references: Vec<Seedance2VideoReference>) -> Self {
        self.reference_videos = Some(references);
        self
    }

    /// Validate this request against the official Seedance 2.0 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Seedance2 {
            return Err(RunwayError::Validation {
                message: "TextToVideoSeedance2Request must use model seedance2".into(),
            });
        }
        validate_prompt_text_length(&self.prompt_text, "promptText", 3500)?;
        if let Some(ratio) = self.ratio {
            validate_video_ratio(ratio, SEEDANCE_ALL_RATIOS, "ratio")?;
        }
        validate_seedance_references(
            self.reference_audio.as_deref(),
            self.references.as_deref(),
            self.reference_videos.as_deref(),
        )
    }
}

macro_rules! define_text_seedance_lite_request {
    ($name:ident, $model:expr, $model_literal:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            /// Model discriminator set by the constructor.
            pub model: VideoModel,
            /// Description of the requested video, up to 3500 UTF-16 code units.
            pub prompt_text: String,
            /// Whether the generated video should include audio.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub audio: Option<bool>,
            /// Optional output duration in seconds.
            #[serde(
                skip_serializing_if = "Option::is_none",
                serialize_with = "serialize_optional_json_number"
            )]
            pub duration: Option<f64>,
            /// Optional 480p or 720p output resolution.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ratio: Option<VideoRatio>,
            /// Optional audio references.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reference_audio: Option<Vec<Seedance2AudioReference>>,
            /// Optional image references, up to nine.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub references: Option<Vec<Seedance2ImageReference>>,
            /// Optional video references.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reference_videos: Option<Vec<Seedance2VideoReference>>,
        }

        impl $name {
            /// Create a request for this model.
            pub fn new(prompt_text: impl Into<String>) -> Self {
                Self {
                    model: $model,
                    prompt_text: prompt_text.into(),
                    audio: None,
                    duration: None,
                    ratio: None,
                    reference_audio: None,
                    references: None,
                    reference_videos: None,
                }
            }

            /// Configure whether to generate audio.
            pub fn audio(mut self, audio: bool) -> Self {
                self.audio = Some(audio);
                self
            }

            /// Set the output duration in seconds.
            pub fn duration(mut self, duration: impl Into<f64>) -> Self {
                self.duration = Some(duration.into());
                self
            }

            /// Set the output resolution.
            pub fn ratio(mut self, ratio: VideoRatio) -> Self {
                self.ratio = Some(ratio);
                self
            }

            /// Set audio references.
            pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
                self.reference_audio = Some(references);
                self
            }

            /// Set image references.
            pub fn references(mut self, references: Vec<Seedance2ImageReference>) -> Self {
                self.references = Some(references);
                self
            }

            /// Set video references.
            pub fn reference_videos(mut self, references: Vec<Seedance2VideoReference>) -> Self {
                self.reference_videos = Some(references);
                self
            }

            /// Validate this request against its official model contract.
            pub fn validate(&self) -> Result<(), RunwayError> {
                if self.model != $model {
                    return Err(RunwayError::Validation {
                        message: concat!(stringify!($name), " must use model ", $model_literal)
                            .into(),
                    });
                }
                validate_prompt_text_length(&self.prompt_text, "promptText", 3500)?;
                if let Some(ratio) = self.ratio {
                    validate_video_ratio(ratio, SEEDANCE_FAST_RATIOS, "ratio")?;
                }
                validate_seedance_references(
                    self.reference_audio.as_deref(),
                    self.references.as_deref(),
                    self.reference_videos.as_deref(),
                )
            }
        }
    };
}

define_text_seedance_lite_request!(
    TextToVideoSeedance2FastRequest,
    VideoModel::Seedance2Fast,
    "seedance2_fast",
    "Request to generate video from text with Seedance 2.0 Fast."
);

define_text_seedance_lite_request!(
    TextToVideoSeedance2MiniRequest,
    VideoModel::Seedance2Mini,
    "seedance2_mini",
    "Request to generate video from text with Seedance 2.0 Mini."
);

/// Request to generate video from text with Gemini Omni Flash.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToVideoGeminiOmniFlashRequest {
    /// Model discriminator. Constructors set this to `gemini_omni_flash`.
    pub model: VideoModel,
    /// Description of the requested video.
    pub prompt_text: String,
    /// Optional whole-number duration from 3 through 10 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    /// Optional landscape or portrait output ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
}

impl TextToVideoGeminiOmniFlashRequest {
    /// Create a Gemini Omni Flash text-to-video request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: VideoModel::GeminiOmniFlash,
            prompt_text: prompt_text.into(),
            duration: None,
            ratio: None,
        }
    }

    /// Set a whole-number duration from 3 through 10 seconds.
    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the output ratio.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Validate this request against the official Gemini Omni Flash contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::GeminiOmniFlash {
            return Err(RunwayError::Validation {
                message: "TextToVideoGeminiOmniFlashRequest must use model gemini_omni_flash"
                    .into(),
            });
        }
        validate_prompt_text_length(&self.prompt_text, "promptText", usize::MAX)?;
        if let Some(duration) = self.duration {
            validate_duration_range(duration, 3, 10, "duration")?;
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[VideoRatio::Landscape, VideoRatio::Portrait],
                "ratio",
            )?;
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
}

impl TextToVideoVeo3Request {
    pub fn new(prompt_text: impl Into<String>, ratio: VideoRatio) -> Self {
        Self {
            duration: 8,
            model: VideoModel::Veo3,
            prompt_text: prompt_text.into(),
            ratio,
            negative_prompt: None,
        }
    }

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
        self
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
        )?;
        Ok(())
    }
}

// ── Image to Video ──────────────────────────────────────────────────────────

/// An image-to-video request discriminated by its `model` field.
///
/// This request union is serialization-only because its wire variants overlap
/// structurally and cannot be deserialized safely with an untagged strategy. It
/// includes the Node 4.10 variants plus a legacy `gen3a_turbo` compatibility
/// shim.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ImageToVideoRequest {
    Gen45(ImageToVideoGen45Request),
    Gen4Turbo(ImageToVideoGen4TurboRequest),
    /// Legacy compatibility variant absent from the Node 4.10 request union.
    Gen3aTurbo(ImageToVideoGen3aTurboRequest),
    Veo31(ImageToVideoVeo31Request),
    Veo31Fast(ImageToVideoVeo31FastRequest),
    /// A Happyhorse 1.0 generation request.
    Happyhorse10(ImageToVideoHappyhorse10Request),
    /// A Seedance 2.0 generation request.
    Seedance2(ImageToVideoSeedance2Request),
    /// A Seedance 2.0 Fast generation request.
    Seedance2Fast(ImageToVideoSeedance2FastRequest),
    /// A Seedance 2.0 Mini generation request.
    Seedance2Mini(ImageToVideoSeedance2MiniRequest),
    /// A Gemini Omni Flash generation request.
    GeminiOmniFlash(ImageToVideoGeminiOmniFlashRequest),
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
            Self::Happyhorse10(request) => request.validate(),
            Self::Seedance2(request) => request.validate(),
            Self::Seedance2Fast(request) => request.validate(),
            Self::Seedance2Mini(request) => request.validate(),
            Self::GeminiOmniFlash(request) => request.validate(),
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

impl From<ImageToVideoHappyhorse10Request> for ImageToVideoRequest {
    fn from(value: ImageToVideoHappyhorse10Request) -> Self {
        Self::Happyhorse10(value)
    }
}

impl From<ImageToVideoSeedance2Request> for ImageToVideoRequest {
    fn from(value: ImageToVideoSeedance2Request) -> Self {
        Self::Seedance2(value)
    }
}

impl From<ImageToVideoSeedance2FastRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoSeedance2FastRequest) -> Self {
        Self::Seedance2Fast(value)
    }
}

impl From<ImageToVideoSeedance2MiniRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoSeedance2MiniRequest) -> Self {
        Self::Seedance2Mini(value)
    }
}

impl From<ImageToVideoGeminiOmniFlashRequest> for ImageToVideoRequest {
    fn from(value: ImageToVideoGeminiOmniFlashRequest) -> Self {
        Self::GeminiOmniFlash(value)
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

/// Legacy `gen3a_turbo` compatibility request absent from the Node 4.10 union.
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
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
            negative_prompt: None,
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

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
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
            negative_prompt: None,
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

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
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

/// Output quality for Happyhorse 1.0 image-to-video generations.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum HappyhorseResolution {
    /// 720p output.
    #[serde(rename = "720P")]
    P720,
    /// 1080p output.
    #[serde(rename = "1080P")]
    P1080,
}

/// Request to generate video from an image with Happyhorse 1.0.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoHappyhorse10Request {
    /// Model discriminator. Constructors set this to `happyhorse_1_0`.
    pub model: VideoModel,
    /// First-frame image URI or single first-frame array.
    pub prompt_image: PromptImageInput,
    /// Optional output duration in seconds.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_json_number"
    )]
    pub duration: Option<f64>,
    /// Optional motion description, up to 2500 UTF-16 code units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    /// Optional output quality tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<HappyhorseResolution>,
}

impl ImageToVideoHappyhorse10Request {
    /// Create a Happyhorse 1.0 image-to-video request.
    pub fn new(prompt_image: impl Into<PromptImageInput>) -> Self {
        Self {
            model: VideoModel::Happyhorse10,
            prompt_image: prompt_image.into(),
            duration: None,
            prompt_text: None,
            resolution: None,
        }
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the motion description.
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    /// Set the output quality tier.
    pub fn resolution(mut self, resolution: HappyhorseResolution) -> Self {
        self.resolution = Some(resolution);
        self
    }

    /// Validate this request against the official Happyhorse 1.0 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Happyhorse10 {
            return Err(RunwayError::Validation {
                message: "ImageToVideoHappyhorse10Request must use model happyhorse_1_0".into(),
            });
        }
        validate_prompt_image_frames(&self.prompt_image, false, true)?;
        if let Some(prompt_text) = &self.prompt_text {
            validate_utf16_length(prompt_text, "promptText", 2500)?;
        }
        Ok(())
    }
}

/// Request to generate video from images with Seedance 2.0.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoSeedance2Request {
    /// Model discriminator. Constructors set this to `seedance2`.
    pub model: VideoModel,
    /// A single image URI, positioned keyframes, or unpositioned references.
    pub prompt_image: Seedance2PromptImageInput,
    /// Whether the generated video should include audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Optional output duration in seconds.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_json_number"
    )]
    pub duration: Option<f64>,
    /// Optional prompt, up to 3500 UTF-16 code units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    /// Optional output resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    /// Optional audio references. These require `promptText`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_audio: Option<Vec<Seedance2AudioReference>>,
}

impl ImageToVideoSeedance2Request {
    /// Create a Seedance 2.0 image-to-video request.
    pub fn new(prompt_image: impl Into<Seedance2PromptImageInput>) -> Self {
        Self {
            model: VideoModel::Seedance2,
            prompt_image: prompt_image.into(),
            audio: None,
            duration: None,
            prompt_text: None,
            ratio: None,
            reference_audio: None,
        }
    }

    /// Configure whether to generate audio.
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the text prompt.
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    /// Set the output resolution.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Set audio references.
    pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
        self.reference_audio = Some(references);
        self
    }

    /// Validate this request against the official Seedance 2.0 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Seedance2 {
            return Err(RunwayError::Validation {
                message: "ImageToVideoSeedance2Request must use model seedance2".into(),
            });
        }
        validate_seedance_prompt_image(&self.prompt_image)?;
        if let Some(prompt_text) = &self.prompt_text {
            validate_utf16_length(prompt_text, "promptText", 3500)?;
        }
        if self
            .reference_audio
            .as_deref()
            .is_some_and(|references| !references.is_empty())
            && self.prompt_text.is_none()
        {
            return Err(RunwayError::Validation {
                message: "referenceAudio requires promptText".into(),
            });
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(ratio, SEEDANCE_ALL_RATIOS, "ratio")?;
        }
        validate_seedance_references(self.reference_audio.as_deref(), None, None)
    }
}

macro_rules! define_image_seedance_lite_request {
    ($name:ident, $model:expr, $model_literal:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            /// Model discriminator set by the constructor.
            pub model: VideoModel,
            /// A single image URI, positioned keyframes, or unpositioned references.
            pub prompt_image: Seedance2PromptImageInput,
            /// Whether the generated video should include audio.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub audio: Option<bool>,
            /// Optional output duration in seconds.
            #[serde(
                skip_serializing_if = "Option::is_none",
                serialize_with = "serialize_optional_json_number"
            )]
            pub duration: Option<f64>,
            /// Optional prompt, up to 3500 UTF-16 code units.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub prompt_text: Option<String>,
            /// Optional 480p or 720p output resolution.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ratio: Option<VideoRatio>,
            /// Optional audio references. These require `promptText`.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reference_audio: Option<Vec<Seedance2AudioReference>>,
        }

        impl $name {
            /// Create a request for this model.
            pub fn new(prompt_image: impl Into<Seedance2PromptImageInput>) -> Self {
                Self {
                    model: $model,
                    prompt_image: prompt_image.into(),
                    audio: None,
                    duration: None,
                    prompt_text: None,
                    ratio: None,
                    reference_audio: None,
                }
            }

            /// Configure whether to generate audio.
            pub fn audio(mut self, audio: bool) -> Self {
                self.audio = Some(audio);
                self
            }

            /// Set the output duration in seconds.
            pub fn duration(mut self, duration: impl Into<f64>) -> Self {
                self.duration = Some(duration.into());
                self
            }

            /// Set the text prompt.
            pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
                self.prompt_text = Some(prompt_text.into());
                self
            }

            /// Set the output resolution.
            pub fn ratio(mut self, ratio: VideoRatio) -> Self {
                self.ratio = Some(ratio);
                self
            }

            /// Set audio references.
            pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
                self.reference_audio = Some(references);
                self
            }

            /// Validate this request against its official model contract.
            pub fn validate(&self) -> Result<(), RunwayError> {
                if self.model != $model {
                    return Err(RunwayError::Validation {
                        message: concat!(stringify!($name), " must use model ", $model_literal)
                            .into(),
                    });
                }
                validate_seedance_prompt_image(&self.prompt_image)?;
                if let Some(prompt_text) = &self.prompt_text {
                    validate_utf16_length(prompt_text, "promptText", 3500)?;
                }
                if self
                    .reference_audio
                    .as_deref()
                    .is_some_and(|references| !references.is_empty())
                    && self.prompt_text.is_none()
                {
                    return Err(RunwayError::Validation {
                        message: "referenceAudio requires promptText".into(),
                    });
                }
                if let Some(ratio) = self.ratio {
                    validate_video_ratio(ratio, SEEDANCE_FAST_RATIOS, "ratio")?;
                }
                validate_seedance_references(self.reference_audio.as_deref(), None, None)
            }
        }
    };
}

define_image_seedance_lite_request!(
    ImageToVideoSeedance2FastRequest,
    VideoModel::Seedance2Fast,
    "seedance2_fast",
    "Request to generate video from images with Seedance 2.0 Fast."
);

define_image_seedance_lite_request!(
    ImageToVideoSeedance2MiniRequest,
    VideoModel::Seedance2Mini,
    "seedance2_mini",
    "Request to generate video from images with Seedance 2.0 Mini."
);

/// Request to generate video from an image with Gemini Omni Flash.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageToVideoGeminiOmniFlashRequest {
    /// Model discriminator. Constructors set this to `gemini_omni_flash`.
    pub model: VideoModel,
    /// First-frame image URI or single first-frame array.
    pub prompt_image: PromptImageInput,
    /// Optional whole-number duration from 3 through 10 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    /// Optional description of how the video should evolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    /// Optional landscape or portrait ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
}

impl ImageToVideoGeminiOmniFlashRequest {
    /// Create a Gemini Omni Flash image-to-video request.
    pub fn new(prompt_image: impl Into<PromptImageInput>) -> Self {
        Self {
            model: VideoModel::GeminiOmniFlash,
            prompt_image: prompt_image.into(),
            duration: None,
            prompt_text: None,
            ratio: None,
        }
    }

    /// Set a whole-number duration from 3 through 10 seconds.
    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the motion description.
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    /// Set the output ratio.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Validate this request against the official Gemini Omni Flash contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::GeminiOmniFlash {
            return Err(RunwayError::Validation {
                message: "ImageToVideoGeminiOmniFlashRequest must use model gemini_omni_flash"
                    .into(),
            });
        }
        validate_prompt_image_frames(&self.prompt_image, false, true)?;
        if let Some(duration) = self.duration {
            validate_duration_range(duration, 3, 10, "duration")?;
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(
                ratio,
                &[VideoRatio::Landscape, VideoRatio::Portrait],
                "ratio",
            )?;
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
    /// Text describing content that should not appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
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
            negative_prompt: None,
            prompt_text: None,
        }
    }

    /// Set text describing content that should not appear.
    pub fn negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(negative_prompt.into());
        self
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

/// A video-to-video request discriminated by its `model` field.
///
/// This request union is serialization-only because its wire variants overlap
/// structurally and cannot be deserialized safely with an untagged strategy. It
/// includes the Node 4.10 variants plus a legacy `gen4_aleph` compatibility
/// shim.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum VideoToVideoCreateRequest {
    /// An Aleph 2 video edit request.
    Aleph2(VideoToVideoAleph2Request),
    /// A Seedance 2.0 video-to-video request.
    Seedance2(VideoToVideoSeedance2Request),
    /// A Seedance 2.0 Fast video-to-video request.
    Seedance2Fast(VideoToVideoSeedance2FastRequest),
    /// A Seedance 2.0 Mini video-to-video request.
    Seedance2Mini(VideoToVideoSeedance2MiniRequest),
    /// A Gemini Omni Flash video edit request.
    GeminiOmniFlash(VideoToVideoGeminiOmniFlashRequest),
    /// A compatibility request for the former `gen4_aleph` model.
    LegacyGen4Aleph(VideoToVideoRequest),
}

impl VideoToVideoCreateRequest {
    /// Validate the selected request variant.
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Aleph2(request) => request.validate(),
            Self::Seedance2(request) => request.validate(),
            Self::Seedance2Fast(request) => request.validate(),
            Self::Seedance2Mini(request) => request.validate(),
            Self::GeminiOmniFlash(request) => request.validate(),
            Self::LegacyGen4Aleph(request) => request.validate(),
        }
    }
}

impl From<VideoToVideoAleph2Request> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoAleph2Request) -> Self {
        Self::Aleph2(value)
    }
}

impl From<VideoToVideoSeedance2Request> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoSeedance2Request) -> Self {
        Self::Seedance2(value)
    }
}

impl From<VideoToVideoSeedance2FastRequest> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoSeedance2FastRequest) -> Self {
        Self::Seedance2Fast(value)
    }
}

impl From<VideoToVideoSeedance2MiniRequest> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoSeedance2MiniRequest) -> Self {
        Self::Seedance2Mini(value)
    }
}

impl From<VideoToVideoGeminiOmniFlashRequest> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoGeminiOmniFlashRequest) -> Self {
        Self::GeminiOmniFlash(value)
    }
}

impl From<VideoToVideoRequest> for VideoToVideoCreateRequest {
    fn from(value: VideoToVideoRequest) -> Self {
        Self::LegacyGen4Aleph(value)
    }
}

/// Whole-second edit window for an Aleph 2 keyframe.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Aleph2EditRange {
    /// End of the edit window, exclusive.
    pub end_seconds: u32,
    /// Start of the edit window.
    pub start_seconds: u32,
}

impl Aleph2EditRange {
    /// Create an edit window from `start_seconds` through `end_seconds`.
    pub fn new(start_seconds: u32, end_seconds: u32) -> Self {
        Self {
            end_seconds,
            start_seconds,
        }
    }

    fn validate(&self) -> Result<(), RunwayError> {
        if self.start_seconds >= self.end_seconds {
            return Err(RunwayError::Validation {
                message: "keyframe range start_seconds must be before end_seconds".into(),
            });
        }
        Ok(())
    }
}

/// Absolute-time Aleph 2 guidance keyframe.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Aleph2SecondsKeyframe {
    /// Timestamp in seconds from the start of the input video.
    pub seconds: f64,
    /// Guidance image URI.
    pub uri: String,
    /// Optional whole-second edit window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Aleph2EditRange>,
}

/// Fractional-position Aleph 2 guidance keyframe.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Aleph2FractionKeyframe {
    /// Position from `0.0` through `1.0` of the input duration.
    pub at: f64,
    /// Guidance image URI.
    pub uri: String,
    /// Optional whole-second edit window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Aleph2EditRange>,
}

/// Timed guidance image for Aleph 2.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Aleph2Keyframe {
    /// Guidance at an absolute timestamp.
    Seconds(Aleph2SecondsKeyframe),
    /// Guidance at a fraction of the input duration.
    Fraction(Aleph2FractionKeyframe),
}

impl Aleph2Keyframe {
    /// Create a keyframe at an absolute timestamp in seconds.
    pub fn seconds(uri: impl Into<String>, seconds: f64) -> Self {
        Self::Seconds(Aleph2SecondsKeyframe {
            seconds,
            uri: uri.into(),
            range: None,
        })
    }

    /// Create a keyframe at a fractional position from `0.0` through `1.0`.
    pub fn at(uri: impl Into<String>, at: f64) -> Self {
        Self::Fraction(Aleph2FractionKeyframe {
            at,
            uri: uri.into(),
            range: None,
        })
    }

    /// Limit this keyframe's edit to a whole-second window.
    pub fn range(mut self, range: Aleph2EditRange) -> Self {
        match &mut self {
            Self::Seconds(keyframe) => keyframe.range = Some(range),
            Self::Fraction(keyframe) => keyframe.range = Some(range),
        }
        self
    }

    fn has_range(&self) -> bool {
        match self {
            Self::Seconds(keyframe) => keyframe.range.is_some(),
            Self::Fraction(keyframe) => keyframe.range.is_some(),
        }
    }

    fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Seconds(keyframe) => {
                validate_media_uri(&keyframe.uri, "keyframes[].uri")?;
                if !keyframe.seconds.is_finite() || keyframe.seconds < 0.0 {
                    return Err(RunwayError::Validation {
                        message: "keyframes[].seconds must be a finite non-negative number".into(),
                    });
                }
                if let Some(range) = keyframe.range {
                    range.validate()?;
                    if keyframe.seconds < f64::from(range.start_seconds)
                        || keyframe.seconds >= f64::from(range.end_seconds)
                    {
                        return Err(RunwayError::Validation {
                            message: "keyframes[].seconds must fall within its range".into(),
                        });
                    }
                }
            }
            Self::Fraction(keyframe) => {
                validate_media_uri(&keyframe.uri, "keyframes[].uri")?;
                if !keyframe.at.is_finite() || !(0.0..=1.0).contains(&keyframe.at) {
                    return Err(RunwayError::Validation {
                        message: "keyframes[].at must be between 0.0 and 1.0".into(),
                    });
                }
                if let Some(range) = keyframe.range {
                    range.validate()?;
                }
            }
        }
        Ok(())
    }
}

/// Target aspect ratio used by Aleph 2 expand/outpaint edits.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum Aleph2TargetAspectRatio {
    /// 16:9 widescreen.
    #[serde(rename = "16:9")]
    Wide16x9,
    /// 4:3 landscape.
    #[serde(rename = "4:3")]
    Landscape4x3,
    /// 3:2 landscape.
    #[serde(rename = "3:2")]
    Landscape3x2,
    /// 1:1 square.
    #[serde(rename = "1:1")]
    Square1x1,
    /// 2:3 portrait.
    #[serde(rename = "2:3")]
    Portrait2x3,
    /// 3:4 portrait.
    #[serde(rename = "3:4")]
    Portrait3x4,
    /// 9:16 vertical.
    #[serde(rename = "9:16")]
    Vertical9x16,
    /// 21:9 ultrawide.
    #[serde(rename = "21:9")]
    Ultrawide21x9,
}

/// Request to edit a video with Aleph 2.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoAleph2Request {
    /// Model discriminator. Constructors set this to `aleph2`.
    pub model: VideoModel,
    /// Input video URI.
    pub video_uri: String,
    /// Optional moderation settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<ContentModeration>,
    /// Up to five timed guidance images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframes: Option<Vec<Aleph2Keyframe>>,
    /// Optional non-empty edit description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    /// Deprecated free-form output ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// Optional generation seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    /// Optional expand/outpaint target aspect ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_aspect_ratio: Option<Aleph2TargetAspectRatio>,
}

impl VideoToVideoAleph2Request {
    /// Create an Aleph 2 request for an input video.
    pub fn new(video_uri: impl Into<String>) -> Self {
        Self {
            model: VideoModel::Aleph2,
            video_uri: video_uri.into(),
            content_moderation: None,
            keyframes: None,
            prompt_text: None,
            ratio: None,
            seed: None,
            target_aspect_ratio: None,
        }
    }

    /// Set moderation options.
    pub fn content_moderation(mut self, content_moderation: ContentModeration) -> Self {
        self.content_moderation = Some(content_moderation);
        self
    }

    /// Set timed guidance images.
    pub fn keyframes(mut self, keyframes: Vec<Aleph2Keyframe>) -> Self {
        self.keyframes = Some(keyframes);
        self
    }

    /// Set the non-empty edit description.
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    /// Set the deprecated free-form ratio.
    #[deprecated(note = "use target_aspect_ratio instead")]
    pub fn ratio(mut self, ratio: impl Into<String>) -> Self {
        self.ratio = Some(ratio.into());
        self
    }

    /// Set the generation seed.
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the expand/outpaint target aspect ratio.
    pub fn target_aspect_ratio(mut self, target: Aleph2TargetAspectRatio) -> Self {
        self.target_aspect_ratio = Some(target);
        self
    }

    /// Validate this request against the official Aleph 2 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Aleph2 {
            return Err(RunwayError::Validation {
                message: "VideoToVideoAleph2Request must use model aleph2".into(),
            });
        }
        validate_media_uri(&self.video_uri, "videoUri")?;
        if let Some(prompt_text) = &self.prompt_text {
            if prompt_text.trim().is_empty() {
                return Err(RunwayError::Validation {
                    message: "promptText cannot be empty".into(),
                });
            }
        }
        if let Some(keyframes) = &self.keyframes {
            if keyframes.len() > 5 {
                return Err(RunwayError::Validation {
                    message: "keyframes may contain at most 5 guidance images".into(),
                });
            }
            if let Some(first) = keyframes.first() {
                let has_range = first.has_range();
                if keyframes
                    .iter()
                    .any(|keyframe| keyframe.has_range() != has_range)
                {
                    return Err(RunwayError::Validation {
                        message: "all keyframes must either set a range or omit it".into(),
                    });
                }
            }
            for keyframe in keyframes {
                keyframe.validate()?;
            }
        }
        Ok(())
    }
}

/// Image reference for Gemini Omni Flash video editing.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GeminiOmniFlashImageReference {
    /// Guidance image URI.
    pub uri: String,
}

impl GeminiOmniFlashImageReference {
    /// Create a guidance image reference.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}

/// Request to edit a video with Gemini Omni Flash.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoGeminiOmniFlashRequest {
    /// Model discriminator. Constructors set this to `gemini_omni_flash`.
    pub model: VideoModel,
    /// Non-empty edit instruction.
    pub prompt_text: String,
    /// Input video URI.
    pub video_uri: String,
    /// Optional guidance image references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<GeminiOmniFlashImageReference>>,
}

impl VideoToVideoGeminiOmniFlashRequest {
    /// Create a Gemini Omni Flash video edit request.
    pub fn new(prompt_text: impl Into<String>, video_uri: impl Into<String>) -> Self {
        Self {
            model: VideoModel::GeminiOmniFlash,
            prompt_text: prompt_text.into(),
            video_uri: video_uri.into(),
            references: None,
        }
    }

    /// Set guidance image references.
    pub fn references(mut self, references: Vec<GeminiOmniFlashImageReference>) -> Self {
        self.references = Some(references);
        self
    }

    /// Validate this request against the official Gemini Omni Flash contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::GeminiOmniFlash {
            return Err(RunwayError::Validation {
                message: "VideoToVideoGeminiOmniFlashRequest must use model gemini_omni_flash"
                    .into(),
            });
        }
        if self.prompt_text.trim().is_empty() {
            return Err(RunwayError::Validation {
                message: "promptText cannot be empty".into(),
            });
        }
        validate_media_uri(&self.video_uri, "videoUri")?;
        if let Some(references) = &self.references {
            for reference in references {
                validate_media_uri(&reference.uri, "references[].uri")?;
            }
        }
        Ok(())
    }
}

/// Request to transform a video with Seedance 2.0.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoToVideoSeedance2Request {
    /// Model discriminator. Constructors set this to `seedance2`.
    pub model: VideoModel,
    /// Input video URI.
    pub prompt_video: String,
    /// Whether to generate audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Optional output duration in seconds.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_json_number"
    )]
    pub duration: Option<f64>,
    /// Optional prompt, up to 3500 UTF-16 code units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
    /// Optional output resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<VideoRatio>,
    /// Optional audio references. These require `promptText`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_audio: Option<Vec<Seedance2AudioReference>>,
    /// Optional image references, up to nine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Seedance2ImageReference>>,
    /// Optional video references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_videos: Option<Vec<Seedance2VideoReference>>,
}

impl VideoToVideoSeedance2Request {
    /// Create a Seedance 2.0 video-to-video request.
    pub fn new(prompt_video: impl Into<String>) -> Self {
        Self {
            model: VideoModel::Seedance2,
            prompt_video: prompt_video.into(),
            audio: None,
            duration: None,
            prompt_text: None,
            ratio: None,
            reference_audio: None,
            references: None,
            reference_videos: None,
        }
    }

    /// Configure whether to generate audio.
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = Some(audio);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the optional text prompt.
    pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
        self.prompt_text = Some(prompt_text.into());
        self
    }

    /// Set the output resolution.
    pub fn ratio(mut self, ratio: VideoRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Set audio references.
    pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
        self.reference_audio = Some(references);
        self
    }

    /// Set image references.
    pub fn references(mut self, references: Vec<Seedance2ImageReference>) -> Self {
        self.references = Some(references);
        self
    }

    /// Set video references.
    pub fn reference_videos(mut self, references: Vec<Seedance2VideoReference>) -> Self {
        self.reference_videos = Some(references);
        self
    }

    /// Validate this request against the official Seedance 2.0 contract.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != VideoModel::Seedance2 {
            return Err(RunwayError::Validation {
                message: "VideoToVideoSeedance2Request must use model seedance2".into(),
            });
        }
        validate_media_uri(&self.prompt_video, "promptVideo")?;
        if let Some(prompt_text) = &self.prompt_text {
            validate_utf16_length(prompt_text, "promptText", 3500)?;
        }
        if self
            .reference_audio
            .as_deref()
            .is_some_and(|references| !references.is_empty())
            && self.prompt_text.is_none()
        {
            return Err(RunwayError::Validation {
                message: "referenceAudio requires promptText".into(),
            });
        }
        if let Some(ratio) = self.ratio {
            validate_video_ratio(ratio, SEEDANCE_ALL_RATIOS, "ratio")?;
        }
        validate_seedance_references(
            self.reference_audio.as_deref(),
            self.references.as_deref(),
            self.reference_videos.as_deref(),
        )
    }
}

macro_rules! define_video_seedance_lite_request {
    ($name:ident, $model:expr, $model_literal:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            /// Model discriminator set by the constructor.
            pub model: VideoModel,
            /// Input video URI.
            pub prompt_video: String,
            /// Whether to generate audio.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub audio: Option<bool>,
            /// Optional output duration in seconds.
            #[serde(
                skip_serializing_if = "Option::is_none",
                serialize_with = "serialize_optional_json_number"
            )]
            pub duration: Option<f64>,
            /// Optional prompt, up to 3500 UTF-16 code units.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub prompt_text: Option<String>,
            /// Optional 480p or 720p output resolution.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ratio: Option<VideoRatio>,
            /// Optional audio references. These require `promptText`.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reference_audio: Option<Vec<Seedance2AudioReference>>,
            /// Optional image references, up to nine.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub references: Option<Vec<Seedance2ImageReference>>,
            /// Optional video references.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reference_videos: Option<Vec<Seedance2VideoReference>>,
        }

        impl $name {
            /// Create a request for this model.
            pub fn new(prompt_video: impl Into<String>) -> Self {
                Self {
                    model: $model,
                    prompt_video: prompt_video.into(),
                    audio: None,
                    duration: None,
                    prompt_text: None,
                    ratio: None,
                    reference_audio: None,
                    references: None,
                    reference_videos: None,
                }
            }

            /// Configure whether to generate audio.
            pub fn audio(mut self, audio: bool) -> Self {
                self.audio = Some(audio);
                self
            }

            /// Set the output duration in seconds.
            pub fn duration(mut self, duration: impl Into<f64>) -> Self {
                self.duration = Some(duration.into());
                self
            }

            /// Set the optional text prompt.
            pub fn prompt_text(mut self, prompt_text: impl Into<String>) -> Self {
                self.prompt_text = Some(prompt_text.into());
                self
            }

            /// Set the output resolution.
            pub fn ratio(mut self, ratio: VideoRatio) -> Self {
                self.ratio = Some(ratio);
                self
            }

            /// Set audio references.
            pub fn reference_audio(mut self, references: Vec<Seedance2AudioReference>) -> Self {
                self.reference_audio = Some(references);
                self
            }

            /// Set image references.
            pub fn references(mut self, references: Vec<Seedance2ImageReference>) -> Self {
                self.references = Some(references);
                self
            }

            /// Set video references.
            pub fn reference_videos(mut self, references: Vec<Seedance2VideoReference>) -> Self {
                self.reference_videos = Some(references);
                self
            }

            /// Validate this request against its official model contract.
            pub fn validate(&self) -> Result<(), RunwayError> {
                if self.model != $model {
                    return Err(RunwayError::Validation {
                        message: concat!(stringify!($name), " must use model ", $model_literal)
                            .into(),
                    });
                }
                validate_media_uri(&self.prompt_video, "promptVideo")?;
                if let Some(prompt_text) = &self.prompt_text {
                    validate_utf16_length(prompt_text, "promptText", 3500)?;
                }
                if self
                    .reference_audio
                    .as_deref()
                    .is_some_and(|references| !references.is_empty())
                    && self.prompt_text.is_none()
                {
                    return Err(RunwayError::Validation {
                        message: "referenceAudio requires promptText".into(),
                    });
                }
                if let Some(ratio) = self.ratio {
                    validate_video_ratio(ratio, SEEDANCE_FAST_RATIOS, "ratio")?;
                }
                validate_seedance_references(
                    self.reference_audio.as_deref(),
                    self.references.as_deref(),
                    self.reference_videos.as_deref(),
                )
            }
        }
    };
}

define_video_seedance_lite_request!(
    VideoToVideoSeedance2FastRequest,
    VideoModel::Seedance2Fast,
    "seedance2_fast",
    "Request to transform a video with Seedance 2.0 Fast."
);

define_video_seedance_lite_request!(
    VideoToVideoSeedance2MiniRequest,
    VideoModel::Seedance2Mini,
    "seedance2_mini",
    "Request to transform a video with Seedance 2.0 Mini."
);

/// Compatibility request for the former `gen4_aleph` video-to-video model.
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
