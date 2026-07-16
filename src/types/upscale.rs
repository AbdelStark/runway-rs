//! Stable image- and video-upscale request types.

use serde::{Deserialize, Serialize};

use crate::error::RunwayError;

/// Magnific precision image-upscaler model discriminator.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ImageUpscaleModel {
    /// The current Magnific precision upscaler.
    #[serde(rename = "magnific_precision_upscaler_v2")]
    MagnificPrecisionUpscalerV2,
}

/// Optimization preset for image upscaling.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ImageUpscaleFlavor {
    /// Illustration-focused enhancement.
    Sublime,
    /// Photographic enhancement.
    Photo,
    /// Denoising enhancement for noisy photographs.
    PhotoDenoiser,
}

/// Multiplicative scale factor for each input image dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageUpscaleScaleFactor {
    /// Double each dimension.
    X2,
    /// Quadruple each dimension.
    X4,
    /// Multiply each dimension by eight.
    X8,
    /// Multiply each dimension by sixteen.
    X16,
}

impl ImageUpscaleScaleFactor {
    /// Every scale factor supported by the pinned official API schema.
    pub const ALL: &'static [Self] = &[Self::X2, Self::X4, Self::X8, Self::X16];

    /// Return the exact numeric scale factor sent to the API.
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::X2 => 2,
            Self::X4 => 4,
            Self::X8 => 8,
            Self::X16 => 16,
        }
    }
}

impl Serialize for ImageUpscaleScaleFactor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.as_u8())
    }
}

impl<'de> Deserialize<'de> for ImageUpscaleScaleFactor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            2 => Ok(Self::X2),
            4 => Ok(Self::X4),
            8 => Ok(Self::X8),
            16 => Ok(Self::X16),
            value => Err(serde::de::Error::custom(format!(
                "unsupported image upscale scale factor: {value}"
            ))),
        }
    }
}

/// Request body for stable Magnific precision image upscaling.
///
/// The API additionally requires each source dimension to be 300–8,000 pixels
/// and the scaled output to contain no more than 25,300,000 pixels. Those
/// constraints are server-validated because this request carries a URI rather
/// than decoded image dimensions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageUpscaleCreateRequest {
    /// HTTPS URL or supported Runway URI of the source image.
    pub image_uri: String,
    /// Exact model discriminator.
    pub model: ImageUpscaleModel,
    /// Optional optimization preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<ImageUpscaleFlavor>,
    /// Optional scale factor; the API defaults to 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<ImageUpscaleScaleFactor>,
    /// Sharpness intensity from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpen: Option<f64>,
    /// Grain and texture enhancement from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_grain: Option<f64>,
    /// Fine-detail enhancement from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultra_detail: Option<f64>,
}

impl ImageUpscaleCreateRequest {
    /// Create an image-upscale request for a source image URI.
    pub fn new(image_uri: impl Into<String>) -> Self {
        Self {
            image_uri: image_uri.into(),
            model: ImageUpscaleModel::MagnificPrecisionUpscalerV2,
            flavor: None,
            scale_factor: None,
            sharpen: None,
            smart_grain: None,
            ultra_detail: None,
        }
    }

    /// Set the optimization preset.
    pub fn flavor(mut self, flavor: ImageUpscaleFlavor) -> Self {
        self.flavor = Some(flavor);
        self
    }

    /// Set the multiplicative scale factor.
    pub fn scale_factor(mut self, scale_factor: ImageUpscaleScaleFactor) -> Self {
        self.scale_factor = Some(scale_factor);
        self
    }

    /// Set the sharpness intensity.
    pub fn sharpen(mut self, sharpen: f64) -> Self {
        self.sharpen = Some(sharpen);
        self
    }

    /// Set the grain and texture enhancement.
    pub fn smart_grain(mut self, smart_grain: f64) -> Self {
        self.smart_grain = Some(smart_grain);
        self
    }

    /// Set the fine-detail enhancement.
    pub fn ultra_detail(mut self, ultra_detail: f64) -> Self {
        self.ultra_detail = Some(ultra_detail);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_uri(&self.image_uri, "imageUri")?;
        validate_percentage(self.sharpen, "sharpen")?;
        validate_percentage(self.smart_grain, "smartGrain")?;
        validate_percentage(self.ultra_detail, "ultraDetail")
    }
}

/// Magnific creative video-upscaler model discriminator.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VideoUpscaleModel {
    /// The current Magnific creative video upscaler.
    #[serde(rename = "magnific_video_upscaler_creative")]
    MagnificVideoUpscalerCreative,
}

/// Processing style for video upscaling.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum VideoUpscaleFlavor {
    /// Enhance color and detail.
    Vivid,
    /// Favor faithful reproduction.
    Natural,
}

/// Target output resolution for video upscaling.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoUpscaleResolution {
    /// 720p output.
    #[serde(rename = "720p")]
    P720,
    /// 1k output.
    #[serde(rename = "1k")]
    K1,
    /// 2k output, which is the API default.
    #[serde(rename = "2k")]
    K2,
    /// 4k output.
    #[serde(rename = "4k")]
    K4,
}

/// Request body for stable Magnific creative video upscaling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoUpscaleCreateRequest {
    /// Exact model discriminator.
    pub model: VideoUpscaleModel,
    /// HTTPS URL or supported Runway URI of the source video.
    pub video_uri: String,
    /// AI-generated detail intensity from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity: Option<f64>,
    /// Optional processing style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor: Option<VideoUpscaleFlavor>,
    /// Whether to increase the output frame rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_boost: Option<bool>,
    /// Optional output resolution; the API defaults to 2k.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<VideoUpscaleResolution>,
    /// Sharpness intensity from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpen: Option<f64>,
    /// Grain and texture enhancement from 0 through 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_grain: Option<f64>,
}

impl VideoUpscaleCreateRequest {
    /// Create a video-upscale request for a source video URI.
    pub fn new(video_uri: impl Into<String>) -> Self {
        Self {
            model: VideoUpscaleModel::MagnificVideoUpscalerCreative,
            video_uri: video_uri.into(),
            creativity: None,
            flavor: None,
            fps_boost: None,
            resolution: None,
            sharpen: None,
            smart_grain: None,
        }
    }

    /// Set the AI-generated detail intensity.
    pub fn creativity(mut self, creativity: f64) -> Self {
        self.creativity = Some(creativity);
        self
    }

    /// Set the processing style.
    pub fn flavor(mut self, flavor: VideoUpscaleFlavor) -> Self {
        self.flavor = Some(flavor);
        self
    }

    /// Set whether to increase the output frame rate.
    pub fn fps_boost(mut self, fps_boost: bool) -> Self {
        self.fps_boost = Some(fps_boost);
        self
    }

    /// Set the target output resolution.
    pub fn resolution(mut self, resolution: VideoUpscaleResolution) -> Self {
        self.resolution = Some(resolution);
        self
    }

    /// Set the sharpness intensity.
    pub fn sharpen(mut self, sharpen: f64) -> Self {
        self.sharpen = Some(sharpen);
        self
    }

    /// Set the grain and texture enhancement.
    pub fn smart_grain(mut self, smart_grain: f64) -> Self {
        self.smart_grain = Some(smart_grain);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_uri(&self.video_uri, "videoUri")?;
        validate_percentage(self.creativity, "creativity")?;
        validate_percentage(self.sharpen, "sharpen")?;
        validate_percentage(self.smart_grain, "smartGrain")
    }
}

fn validate_uri(uri: &str, field_name: &str) -> Result<(), RunwayError> {
    if uri.trim().is_empty() {
        validation(format!("{field_name} cannot be empty"))
    } else {
        Ok(())
    }
}

fn validate_percentage(value: Option<f64>, field_name: &str) -> Result<(), RunwayError> {
    if let Some(value) = value {
        if !value.is_finite() || !(0.0..=100.0).contains(&value) {
            return validation(format!("{field_name} must be between 0 and 100"));
        }
    }
    Ok(())
}

fn validation<T>(message: impl Into<String>) -> Result<T, RunwayError> {
    Err(RunwayError::Validation {
        message: message.into(),
    })
}
