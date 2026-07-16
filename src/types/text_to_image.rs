//! Request types for the latest text-to-image model families.
//!
//! The original text-to-image request types remain available from
//! [`crate::types::generation`]. [`TextToImageCreateRequest`] combines those
//! requests with the newer model-specific request shapes without widening a
//! model's ratio or reference-image domain.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::RunwayError;

use super::generation::{
    TextToImageGemini25FlashRequest, TextToImageGen4ImageRequest, TextToImageGen4ImageTurboRequest,
    TextToImageRequest,
};
use super::models::{ImageModel, ImageRatio};

macro_rules! string_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $name:ident {
            $($variant:ident => $wire:literal),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(
                #[doc = concat!("The `", $wire, "` API value.")]
                #[serde(rename = $wire)]
                $variant,
            )+
        }

        impl $name {
            /// Every value supported by the pinned official API schema.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Return the exact string sent to the API.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $wire),+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! number_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $name:ident {
            $($variant:ident => $wire:literal),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(
                #[doc = concat!("The numeric API value `", stringify!($wire), "`.")]
                $variant,
            )+
        }

        impl $name {
            /// Every value supported by the pinned official API schema.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Return the exact number sent to the API.
            pub const fn as_u8(self) -> u8 {
                match self {
                    $(Self::$variant => $wire),+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u8(self.as_u8())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u8::deserialize(deserializer)?;
                match value {
                    $($wire => Ok(Self::$variant),)+
                    _ => Err(serde::de::Error::custom(format!(
                        "unsupported {} value: {value}",
                        stringify!($name)
                    ))),
                }
            }
        }
    };
}

string_enum! {
    /// Model discriminator for GPT Image 2 requests.
    pub enum GptImage2Model {
        GptImage2 => "gpt_image_2",
    }
}

string_enum! {
    /// Output resolutions supported by GPT Image 2.
    pub enum GptImage2Ratio {
        R2048x880 => "2048:880",
        R1920x1088 => "1920:1088",
        R1920x1280 => "1920:1280",
        R1920x1440 => "1920:1440",
        R1920x1536 => "1920:1536",
        R1920x1920 => "1920:1920",
        R1536x1920 => "1536:1920",
        R1440x1920 => "1440:1920",
        R1280x1920 => "1280:1920",
        R1088x1920 => "1088:1920",
        R2912x1248 => "2912:1248",
        R2560x1440 => "2560:1440",
        R2560x1712 => "2560:1712",
        R2560x1920 => "2560:1920",
        R2560x2048 => "2560:2048",
        R2560x2560 => "2560:2560",
        R2048x2560 => "2048:2560",
        R1920x2560 => "1920:2560",
        R1712x2560 => "1712:2560",
        R1440x2560 => "1440:2560",
        R3840x1648 => "3840:1648",
        R3840x2160 => "3840:2160",
        R3504x2336 => "3504:2336",
        R3264x2448 => "3264:2448",
        R3200x2560 => "3200:2560",
        R2880x2880 => "2880:2880",
        R2560x3200 => "2560:3200",
        R2448x3264 => "2448:3264",
        R2336x3504 => "2336:3504",
        R2160x3840 => "2160:3840",
        Auto => "auto",
    }
}

string_enum! {
    /// Background treatment for GPT Image 2 output.
    pub enum GptImage2Background {
        Opaque => "opaque",
        Auto => "auto",
    }
}

string_enum! {
    /// Rendering quality for GPT Image 2 output.
    pub enum GptImage2Quality {
        Low => "low",
        Medium => "medium",
        High => "high",
        Auto => "auto",
    }
}

/// A reference image accepted by GPT Image 2.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GptImage2ReferenceImage {
    /// HTTPS URL or supported Runway media URI.
    pub uri: String,
    /// Optional tag used to refer to this image in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GptImage2ReferenceImage {
    /// Create a reference image from a URI.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            tag: None,
        }
    }

    /// Attach a prompt-visible tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Validate this reference before transport.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_uri(&self.uri, "referenceImages[].uri")
    }
}

/// A GPT Image 2 text-to-image request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGptImage2Request {
    /// Exact model discriminator.
    pub model: GptImage2Model,
    /// Non-empty prompt of at most 32,000 characters.
    pub prompt_text: String,
    /// Output resolution or automatic sizing.
    pub ratio: GptImage2Ratio,
    /// Optional background treatment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<GptImage2Background>,
    /// Number of images to generate, from 1 through 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<u8>,
    /// Optional rendering quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<GptImage2Quality>,
    /// Up to 16 reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<GptImage2ReferenceImage>>,
}

impl TextToImageGptImage2Request {
    /// Create a GPT Image 2 request.
    pub fn new(prompt_text: impl Into<String>, ratio: GptImage2Ratio) -> Self {
        Self {
            model: GptImage2Model::GptImage2,
            prompt_text: prompt_text.into(),
            ratio,
            background: None,
            output_count: None,
            quality: None,
            reference_images: None,
        }
    }

    /// Set the output background treatment.
    pub fn background(mut self, background: GptImage2Background) -> Self {
        self.background = Some(background);
        self
    }

    /// Set the number of generated images.
    pub fn output_count(mut self, output_count: u8) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the rendering quality.
    pub fn quality(mut self, quality: GptImage2Quality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set the reference images.
    pub fn reference_images(mut self, reference_images: Vec<GptImage2ReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_prompt(&self.prompt_text, 32_000)?;
        if let Some(output_count) = self.output_count {
            if !(1..=10).contains(&output_count) {
                return validation("outputCount must be between 1 and 10");
            }
        }
        if let Some(reference_images) = &self.reference_images {
            if reference_images.len() > 16 {
                return validation("gpt_image_2 supports up to 16 referenceImages");
            }
            for reference in reference_images {
                reference.validate()?;
            }
        }
        Ok(())
    }
}

string_enum! {
    /// Model discriminator for Gemini Image 3 Pro requests.
    pub enum GeminiImage3ProModel {
        GeminiImage3Pro => "gemini_image3_pro",
    }
}

string_enum! {
    /// Output resolutions supported by Gemini Image 3 Pro.
    pub enum GeminiImage3ProRatio {
        R1344x768 => "1344:768",
        R768x1344 => "768:1344",
        R1024x1024 => "1024:1024",
        R1184x864 => "1184:864",
        R864x1184 => "864:1184",
        R1536x672 => "1536:672",
        R832x1248 => "832:1248",
        R1248x832 => "1248:832",
        R896x1152 => "896:1152",
        R1152x896 => "1152:896",
        R2048x2048 => "2048:2048",
        R1696x2528 => "1696:2528",
        R2528x1696 => "2528:1696",
        R1792x2400 => "1792:2400",
        R2400x1792 => "2400:1792",
        R1856x2304 => "1856:2304",
        R2304x1856 => "2304:1856",
        R1536x2752 => "1536:2752",
        R2752x1536 => "2752:1536",
        R3168x1344 => "3168:1344",
        R4096x4096 => "4096:4096",
        R3392x5056 => "3392:5056",
        R5056x3392 => "5056:3392",
        R3584x4800 => "3584:4800",
        R4800x3584 => "4800:3584",
        R3712x4608 => "3712:4608",
        R4608x3712 => "4608:3712",
        R3072x5504 => "3072:5504",
        R5504x3072 => "5504:3072",
        R6336x2688 => "6336:2688",
    }
}

string_enum! {
    /// Model discriminator for Gemini Image 3.1 Flash requests.
    pub enum GeminiImage31FlashModel {
        GeminiImage31Flash => "gemini_image3.1_flash",
    }
}

string_enum! {
    /// Output resolutions supported by Gemini Image 3.1 Flash.
    pub enum GeminiImage31FlashRatio {
        R512x512 => "512:512",
        R416x624 => "416:624",
        R624x416 => "624:416",
        R432x592 => "432:592",
        R592x432 => "592:432",
        R448x576 => "448:576",
        R576x448 => "576:448",
        R384x672 => "384:672",
        R672x384 => "672:384",
        R768x336 => "768:336",
        R256x1024 => "256:1024",
        R1024x256 => "1024:256",
        R176x1408 => "176:1408",
        R1408x176 => "1408:176",
        R1024x1024 => "1024:1024",
        R832x1248 => "832:1248",
        R1248x832 => "1248:832",
        R864x1184 => "864:1184",
        R1184x864 => "1184:864",
        R896x1152 => "896:1152",
        R1152x896 => "1152:896",
        R768x1344 => "768:1344",
        R1344x768 => "1344:768",
        R1536x672 => "1536:672",
        R512x2048 => "512:2048",
        R2048x512 => "2048:512",
        R352x2816 => "352:2816",
        R2816x352 => "2816:352",
        R2048x2048 => "2048:2048",
        R1696x2528 => "1696:2528",
        R2528x1696 => "2528:1696",
        R1792x2400 => "1792:2400",
        R2400x1792 => "2400:1792",
        R1856x2304 => "1856:2304",
        R2304x1856 => "2304:1856",
        R1536x2752 => "1536:2752",
        R2752x1536 => "2752:1536",
        R3168x1344 => "3168:1344",
        R1024x4096 => "1024:4096",
        R4096x1024 => "4096:1024",
        R704x5632 => "704:5632",
        R5632x704 => "5632:704",
        R4096x4096 => "4096:4096",
        R3392x5056 => "3392:5056",
        R5056x3392 => "5056:3392",
        R3584x4800 => "3584:4800",
        R4800x3584 => "4800:3584",
        R3712x4608 => "3712:4608",
        R4608x3712 => "4608:3712",
        R3072x5504 => "3072:5504",
        R5504x3072 => "5504:3072",
        R6336x2688 => "6336:2688",
        R2048x8192 => "2048:8192",
        R8192x2048 => "8192:2048",
        R1408x11264 => "1408:11264",
        R11264x1408 => "11264:1408",
    }
}

number_enum! {
    /// Number of images produced by a Gemini Image 3 request.
    pub enum GeminiImageOutputCount {
        One => 1,
        Four => 4,
    }
}

string_enum! {
    /// Subject role assigned to a Gemini reference image.
    pub enum GeminiImageReferenceSubject {
        Object => "object",
        Human => "human",
    }
}

/// A reference image accepted by Gemini Image 3 models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageReferenceImage {
    /// HTTPS URL or supported Runway media URI.
    pub uri: String,
    /// Optional subject role used for model-specific consistency controls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<GeminiImageReferenceSubject>,
    /// Optional tag used to refer to this image in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GeminiImageReferenceImage {
    /// Create a reference image from a URI.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            subject: None,
            tag: None,
        }
    }

    /// Classify this image as a human or object subject.
    pub fn subject(mut self, subject: GeminiImageReferenceSubject) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Attach a prompt-visible tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Validate this reference before transport.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_uri(&self.uri, "referenceImages[].uri")
    }
}

/// A Gemini Image 3 Pro text-to-image request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGeminiImage3ProRequest {
    /// Exact model discriminator.
    pub model: GeminiImage3ProModel,
    /// Generation prompt.
    pub prompt_text: String,
    /// Output resolution.
    pub ratio: GeminiImage3ProRatio,
    /// Optional output count, restricted to one or four.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<GeminiImageOutputCount>,
    /// Up to 14 reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<GeminiImageReferenceImage>>,
}

impl TextToImageGeminiImage3ProRequest {
    /// Create a Gemini Image 3 Pro request.
    pub fn new(prompt_text: impl Into<String>, ratio: GeminiImage3ProRatio) -> Self {
        Self {
            model: GeminiImage3ProModel::GeminiImage3Pro,
            prompt_text: prompt_text.into(),
            ratio,
            output_count: None,
            reference_images: None,
        }
    }

    /// Set the output count to one or four.
    pub fn output_count(mut self, output_count: GeminiImageOutputCount) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the reference images.
    pub fn reference_images(mut self, reference_images: Vec<GeminiImageReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_gemini_references(self.reference_images.as_deref())
    }
}

/// A Gemini Image 3.1 Flash text-to-image request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageGeminiImage31FlashRequest {
    /// Exact model discriminator.
    pub model: GeminiImage31FlashModel,
    /// Generation prompt.
    pub prompt_text: String,
    /// Output resolution.
    pub ratio: GeminiImage31FlashRatio,
    /// Optional output count, restricted to one or four.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<GeminiImageOutputCount>,
    /// Up to 14 reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<GeminiImageReferenceImage>>,
}

impl TextToImageGeminiImage31FlashRequest {
    /// Create a Gemini Image 3.1 Flash request.
    pub fn new(prompt_text: impl Into<String>, ratio: GeminiImage31FlashRatio) -> Self {
        Self {
            model: GeminiImage31FlashModel::GeminiImage31Flash,
            prompt_text: prompt_text.into(),
            ratio,
            output_count: None,
            reference_images: None,
        }
    }

    /// Set the output count to one or four.
    pub fn output_count(mut self, output_count: GeminiImageOutputCount) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the reference images.
    pub fn reference_images(mut self, reference_images: Vec<GeminiImageReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_gemini_references(self.reference_images.as_deref())
    }
}

string_enum! {
    /// Model discriminator for Seedream 5 Pro requests.
    pub enum Seedream5ProModel {
        Seedream5Pro => "seedream5_pro",
    }
}

string_enum! {
    /// Output resolutions supported by Seedream 5 Pro.
    pub enum Seedream5ProRatio {
        R1024x1024 => "1024:1024",
        R1184x896 => "1184:896",
        R896x1184 => "896:1184",
        R1376x768 => "1376:768",
        R768x1376 => "768:1376",
        R1296x864 => "1296:864",
        R864x1296 => "864:1296",
        R2048x2048 => "2048:2048",
        R2304x1728 => "2304:1728",
        R1728x2304 => "1728:2304",
        R2720x1530 => "2720:1530",
        R1530x2720 => "1530:2720",
        R2496x1664 => "2496:1664",
        R1664x2496 => "1664:2496",
        Auto1k => "auto_1k",
        Auto2k => "auto_2k",
    }
}

string_enum! {
    /// Model discriminator for Seedream 5 Lite requests.
    pub enum Seedream5LiteModel {
        Seedream5Lite => "seedream5_lite",
    }
}

string_enum! {
    /// Output resolutions supported by Seedream 5 Lite.
    pub enum Seedream5LiteRatio {
        R2048x2048 => "2048:2048",
        R2304x1728 => "2304:1728",
        R1728x2304 => "1728:2304",
        R2848x1600 => "2848:1600",
        R1600x2848 => "1600:2848",
        R2496x1664 => "2496:1664",
        R1664x2496 => "1664:2496",
        R3136x1344 => "3136:1344",
        R3072x3072 => "3072:3072",
        R3456x2592 => "3456:2592",
        R2592x3456 => "2592:3456",
        R4096x2304 => "4096:2304",
        R2304x4096 => "2304:4096",
        R3744x2496 => "3744:2496",
        R2496x3744 => "2496:3744",
        R4704x2016 => "4704:2016",
    }
}

string_enum! {
    /// File format for Seedream image output.
    pub enum SeedreamOutputFormat {
        Png => "png",
        Jpeg => "jpeg",
    }
}

/// A URI-only reference image accepted by Seedream 5 models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeedreamReferenceImage {
    /// HTTPS URL or supported Runway media URI.
    pub uri: String,
}

impl SeedreamReferenceImage {
    /// Create a Seedream reference image.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }

    /// Validate this reference before transport.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_uri(&self.uri, "referenceImages[].uri")
    }
}

/// A Seedream 5 Pro text-to-image request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageSeedream5ProRequest {
    /// Exact model discriminator.
    pub model: Seedream5ProModel,
    /// Non-empty prompt of at most 4,000 characters.
    pub prompt_text: String,
    /// Output resolution or automatic resolution tier.
    pub ratio: Seedream5ProRatio,
    /// Optional number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<f64>,
    /// Optional file format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<SeedreamOutputFormat>,
    /// Optional reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<SeedreamReferenceImage>>,
}

impl TextToImageSeedream5ProRequest {
    /// Create a Seedream 5 Pro request.
    pub fn new(prompt_text: impl Into<String>, ratio: Seedream5ProRatio) -> Self {
        Self {
            model: Seedream5ProModel::Seedream5Pro,
            prompt_text: prompt_text.into(),
            ratio,
            output_count: None,
            output_format: None,
            reference_images: None,
        }
    }

    /// Set the number of generated images.
    pub fn output_count(mut self, output_count: f64) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the output file format.
    pub fn output_format(mut self, output_format: SeedreamOutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }

    /// Set the reference images.
    pub fn reference_images(mut self, reference_images: Vec<SeedreamReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_prompt(&self.prompt_text, 4_000)?;
        validate_finite(self.output_count, "outputCount")?;
        validate_seedream_references(self.reference_images.as_deref())
    }
}

/// A Seedream 5 Lite text-to-image request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextToImageSeedream5LiteRequest {
    /// Exact model discriminator.
    pub model: Seedream5LiteModel,
    /// Non-empty prompt of at most 4,000 characters.
    pub prompt_text: String,
    /// Output resolution.
    pub ratio: Seedream5LiteRatio,
    /// Optional number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<f64>,
    /// Optional file format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<SeedreamOutputFormat>,
    /// Optional reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<SeedreamReferenceImage>>,
}

impl TextToImageSeedream5LiteRequest {
    /// Create a Seedream 5 Lite request.
    pub fn new(prompt_text: impl Into<String>, ratio: Seedream5LiteRatio) -> Self {
        Self {
            model: Seedream5LiteModel::Seedream5Lite,
            prompt_text: prompt_text.into(),
            ratio,
            output_count: None,
            output_format: None,
            reference_images: None,
        }
    }

    /// Set the number of generated images.
    pub fn output_count(mut self, output_count: f64) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the output file format.
    pub fn output_format(mut self, output_format: SeedreamOutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }

    /// Set the reference images.
    pub fn reference_images(mut self, reference_images: Vec<SeedreamReferenceImage>) -> Self {
        self.reference_images = Some(reference_images);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_prompt(&self.prompt_text, 4_000)?;
        validate_finite(self.output_count, "outputCount")?;
        validate_seedream_references(self.reference_images.as_deref())
    }
}

/// Any request accepted by the current text-to-image endpoint.
///
/// Legacy request structs convert into this enum automatically, so existing
/// calls such as `client.text_to_image().create(legacy_request)` keep working.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum TextToImageCreateRequest {
    /// One of the original Gen-4 or Gemini 2.5 request shapes.
    Legacy(TextToImageRequest),
    /// GPT Image 2.
    GptImage2(TextToImageGptImage2Request),
    /// Gemini Image 3 Pro.
    GeminiImage3Pro(TextToImageGeminiImage3ProRequest),
    /// Gemini Image 3.1 Flash.
    GeminiImage31Flash(TextToImageGeminiImage31FlashRequest),
    /// Seedream 5 Pro.
    Seedream5Pro(TextToImageSeedream5ProRequest),
    /// Seedream 5 Lite.
    Seedream5Lite(TextToImageSeedream5LiteRequest),
}

impl TextToImageCreateRequest {
    /// Validate the selected model-specific request.
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::Legacy(TextToImageRequest::Gemini25Flash(request)) => {
                validate_legacy_gemini25_flash(request)
            }
            Self::Legacy(request) => request.validate(),
            Self::GptImage2(request) => request.validate(),
            Self::GeminiImage3Pro(request) => request.validate(),
            Self::GeminiImage31Flash(request) => request.validate(),
            Self::Seedream5Pro(request) => request.validate(),
            Self::Seedream5Lite(request) => request.validate(),
        }
    }
}

impl From<TextToImageRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageRequest) -> Self {
        Self::Legacy(value)
    }
}

impl From<TextToImageGen4ImageTurboRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageGen4ImageTurboRequest) -> Self {
        Self::Legacy(value.into())
    }
}

impl From<TextToImageGen4ImageRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageGen4ImageRequest) -> Self {
        Self::Legacy(value.into())
    }
}

impl From<TextToImageGemini25FlashRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageGemini25FlashRequest) -> Self {
        Self::Legacy(value.into())
    }
}

impl From<TextToImageGptImage2Request> for TextToImageCreateRequest {
    fn from(value: TextToImageGptImage2Request) -> Self {
        Self::GptImage2(value)
    }
}

impl From<TextToImageGeminiImage3ProRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageGeminiImage3ProRequest) -> Self {
        Self::GeminiImage3Pro(value)
    }
}

impl From<TextToImageGeminiImage31FlashRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageGeminiImage31FlashRequest) -> Self {
        Self::GeminiImage31Flash(value)
    }
}

impl From<TextToImageSeedream5ProRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageSeedream5ProRequest) -> Self {
        Self::Seedream5Pro(value)
    }
}

impl From<TextToImageSeedream5LiteRequest> for TextToImageCreateRequest {
    fn from(value: TextToImageSeedream5LiteRequest) -> Self {
        Self::Seedream5Lite(value)
    }
}

fn validate_nonempty(value: &str, field_name: &str) -> Result<(), RunwayError> {
    if value.trim().is_empty() {
        validation(format!("{field_name} cannot be empty"))
    } else {
        Ok(())
    }
}

fn validate_legacy_gemini25_flash(
    request: &TextToImageGemini25FlashRequest,
) -> Result<(), RunwayError> {
    if request.model != ImageModel::Gemini25Flash {
        return validation("TextToImageGemini25FlashRequest must use model gemini_2.5_flash");
    }
    if !matches!(
        request.ratio,
        ImageRatio::GeminiLandscape1344x768
            | ImageRatio::GeminiPortrait768x1344
            | ImageRatio::Square1024
            | ImageRatio::GeminiLandscape1184x864
            | ImageRatio::GeminiPortrait864x1184
            | ImageRatio::GeminiUltrawide1536x672
            | ImageRatio::GeminiPortrait832x1248
            | ImageRatio::GeminiLandscape1248x832
            | ImageRatio::GeminiPortrait896x1152
            | ImageRatio::GeminiLandscape1152x896
    ) {
        return validation("ratio is not supported by gemini_2.5_flash");
    }
    if let Some(reference_images) = &request.reference_images {
        if reference_images.len() > 3 {
            return validation("gemini_2.5_flash supports up to 3 referenceImages");
        }
        for reference in reference_images {
            reference.validate()?;
        }
    }
    Ok(())
}

fn validate_prompt(prompt_text: &str, maximum_characters: usize) -> Result<(), RunwayError> {
    validate_nonempty(prompt_text, "promptText")?;
    if prompt_text.chars().count() > maximum_characters {
        validation(format!(
            "promptText must be at most {maximum_characters} characters"
        ))
    } else {
        Ok(())
    }
}

fn validate_uri(uri: &str, field_name: &str) -> Result<(), RunwayError> {
    validate_nonempty(uri, field_name)
}

fn validate_finite(value: Option<f64>, field_name: &str) -> Result<(), RunwayError> {
    if value.is_some_and(|value| !value.is_finite()) {
        validation(format!("{field_name} must be a finite number"))
    } else {
        Ok(())
    }
}

fn validate_gemini_references(
    reference_images: Option<&[GeminiImageReferenceImage]>,
) -> Result<(), RunwayError> {
    let Some(reference_images) = reference_images else {
        return Ok(());
    };
    if reference_images.len() > 14 {
        return validation("Gemini Image 3 supports up to 14 referenceImages");
    }

    let mut human_count = 0;
    let mut object_count = 0;
    for reference in reference_images {
        reference.validate()?;
        match reference.subject {
            Some(GeminiImageReferenceSubject::Human) => human_count += 1,
            Some(GeminiImageReferenceSubject::Object) => object_count += 1,
            None => {}
        }
    }
    if human_count > 5 {
        return validation("Gemini Image 3 supports at most 5 human referenceImages");
    }
    if object_count > 9 {
        return validation("Gemini Image 3 supports at most 9 object referenceImages");
    }
    Ok(())
}

fn validate_seedream_references(
    reference_images: Option<&[SeedreamReferenceImage]>,
) -> Result<(), RunwayError> {
    if let Some(reference_images) = reference_images {
        for reference in reference_images {
            reference.validate()?;
        }
    }
    Ok(())
}

fn validation<T>(message: impl Into<String>) -> Result<T, RunwayError> {
    Err(RunwayError::Validation {
        message: message.into(),
    })
}
