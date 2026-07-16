//! Typed requests for Runway's production recipe workflows.

use serde::{Deserialize, Serialize};

/// Version selector shared by recipes currently pinned to the June 2026 contract.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeVersion {
    /// Pin workflow behavior to the June 2026 version.
    #[serde(rename = "2026-06")]
    V2026_06,
    /// Follow the newest stable version without compatibility guarantees.
    #[serde(rename = "unsafe-latest")]
    UnsafeLatest,
}

/// Version selector for the product-ad recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductAdRecipeVersion {
    /// Pin workflow behavior to the June 2026 version.
    #[serde(rename = "2026-06")]
    V2026_06,
    /// Pin workflow behavior to the July 2026 version.
    #[serde(rename = "2026-07")]
    V2026_07,
    /// Follow the newest stable version without compatibility guarantees.
    #[serde(rename = "unsafe-latest")]
    UnsafeLatest,
}

/// Image input used by recipe workflows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeImage {
    /// HTTPS URL of the image.
    pub uri: String,
}

impl RecipeImage {
    /// Create a recipe image input from an HTTPS URL.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}

/// Video input used by recipe workflows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeVideo {
    /// HTTPS URL of the video.
    pub uri: String,
}

impl RecipeVideo {
    /// Create a recipe video input from an HTTPS URL.
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}

/// Target language supported by the ad-localization recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeAdLocalizationLanguage {
    /// Arabic.
    #[serde(rename = "ar")]
    Ar,
    /// Simplified Chinese.
    #[serde(rename = "zh")]
    Zh,
    /// Traditional Chinese.
    #[serde(rename = "zh-Hant")]
    ZhHant,
    /// Dutch.
    #[serde(rename = "nl")]
    Nl,
    /// English.
    #[serde(rename = "en")]
    En,
    /// French.
    #[serde(rename = "fr")]
    Fr,
    /// German.
    #[serde(rename = "de")]
    De,
    /// Hindi.
    #[serde(rename = "hi")]
    Hi,
    /// Indonesian.
    #[serde(rename = "id")]
    Id,
    /// Italian.
    #[serde(rename = "it")]
    It,
    /// Japanese.
    #[serde(rename = "ja")]
    Ja,
    /// Korean.
    #[serde(rename = "ko")]
    Ko,
    /// Polish.
    #[serde(rename = "pl")]
    Pl,
    /// Portuguese.
    #[serde(rename = "pt")]
    Pt,
    /// Russian.
    #[serde(rename = "ru")]
    Ru,
    /// Spanish.
    #[serde(rename = "es")]
    Es,
    /// Swedish.
    #[serde(rename = "sv")]
    Sv,
    /// Thai.
    #[serde(rename = "th")]
    Th,
    /// Turkish.
    #[serde(rename = "tr")]
    Tr,
    /// Ukrainian.
    #[serde(rename = "uk")]
    Uk,
    /// Vietnamese.
    #[serde(rename = "vi")]
    Vi,
    /// Greek.
    #[serde(rename = "el")]
    El,
}

/// Request to localize an existing advertisement image.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeAdLocalizationRequest {
    /// Advertisement image to localize.
    pub reference_image: RecipeImage,
    /// Language for localized on-screen messaging.
    pub target_language: RecipeAdLocalizationLanguage,
    /// Recipe workflow version.
    pub version: RecipeVersion,
}

impl RecipeAdLocalizationRequest {
    /// Create an ad-localization request.
    pub fn new(
        reference_image: RecipeImage,
        target_language: RecipeAdLocalizationLanguage,
        version: RecipeVersion,
    ) -> Self {
        Self {
            reference_image,
            target_language,
            version,
        }
    }
}

/// Rendering quality for the marketing-stock-image recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RecipeImageQuality {
    /// Fastest, lowest-credit rendering.
    Low,
    /// Balanced rendering quality.
    Medium,
    /// Highest-fidelity rendering.
    High,
}

/// Request to generate polished marketing stock imagery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeMarketingStockImageRequest {
    /// Marketing-image creative brief.
    pub prompt: String,
    /// Recipe workflow version.
    pub version: RecipeVersion,
    /// Number of images to generate, from one through four.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_count: Option<u8>,
    /// Rendering quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<RecipeImageQuality>,
    /// Optional brand-logo reference image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_image: Option<RecipeImage>,
}

impl RecipeMarketingStockImageRequest {
    /// Create a marketing-stock-image request.
    pub fn new(prompt: impl Into<String>, version: RecipeVersion) -> Self {
        Self {
            prompt: prompt.into(),
            version,
            output_count: None,
            quality: None,
            reference_image: None,
        }
    }

    /// Set the number of output images.
    pub fn output_count(mut self, output_count: u8) -> Self {
        self.output_count = Some(output_count);
        self
    }

    /// Set the rendering quality.
    pub fn quality(mut self, quality: RecipeImageQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Add a brand-logo reference image.
    pub fn reference_image(mut self, reference_image: RecipeImage) -> Self {
        self.reference_image = Some(reference_image);
        self
    }
}

/// Allowed total duration for the multi-shot-video recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "u8", into = "u8")]
pub enum RecipeMultiShotDuration {
    /// Five seconds.
    Five,
    /// Ten seconds.
    Ten,
    /// Fifteen seconds.
    Fifteen,
}

impl From<RecipeMultiShotDuration> for u8 {
    fn from(value: RecipeMultiShotDuration) -> Self {
        match value {
            RecipeMultiShotDuration::Five => 5,
            RecipeMultiShotDuration::Ten => 10,
            RecipeMultiShotDuration::Fifteen => 15,
        }
    }
}

impl TryFrom<u8> for RecipeMultiShotDuration {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(Self::Five),
            10 => Ok(Self::Ten),
            15 => Ok(Self::Fifteen),
            _ => Err(format!("unsupported multi-shot duration: {value}")),
        }
    }
}

/// Output dimensions supported by the multi-shot-video recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeMultiShotRatio {
    /// 1280 by 720 landscape output.
    #[serde(rename = "1280:720")]
    Landscape720p,
    /// 720 by 1280 portrait output.
    #[serde(rename = "720:1280")]
    Portrait720p,
    /// 960 by 960 square output.
    #[serde(rename = "960:960")]
    Square720p,
    /// 1920 by 1080 landscape output.
    #[serde(rename = "1920:1080")]
    Landscape1080p,
    /// 1080 by 1920 portrait output.
    #[serde(rename = "1080:1920")]
    Portrait1080p,
    /// 1440 by 1440 square output.
    #[serde(rename = "1440:1440")]
    Square1080p,
}

/// One custom shot in a multi-shot-video request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeMultiShot {
    /// Shot duration in seconds.
    pub duration: u8,
    /// Shot-description prompt.
    pub prompt: String,
}

impl RecipeMultiShot {
    /// Create a custom shot description.
    pub fn new(duration: u8, prompt: impl Into<String>) -> Self {
        Self {
            duration,
            prompt: prompt.into(),
        }
    }
}

/// Multi-shot-video request, discriminated by the `mode` field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum RecipeMultiShotVideoRequest {
    /// Automatically decompose a story prompt into five shots.
    Auto {
        /// Story prompt.
        prompt: String,
        /// Recipe workflow version.
        version: RecipeVersion,
        /// Whether to generate audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<bool>,
        /// Total output duration.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<RecipeMultiShotDuration>,
        /// Optional image used as the first output frame.
        #[serde(rename = "firstFrame", skip_serializing_if = "Option::is_none")]
        first_frame: Option<RecipeImage>,
        /// Output dimensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        ratio: Option<RecipeMultiShotRatio>,
    },
    /// Polish a caller-provided list of three to five shots.
    Custom {
        /// Custom shot list.
        shots: Vec<RecipeMultiShot>,
        /// Recipe workflow version.
        version: RecipeVersion,
        /// Whether to generate audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<bool>,
        /// Total output duration.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<RecipeMultiShotDuration>,
        /// Optional image used as the first output frame.
        #[serde(rename = "firstFrame", skip_serializing_if = "Option::is_none")]
        first_frame: Option<RecipeImage>,
        /// Output dimensions.
        #[serde(skip_serializing_if = "Option::is_none")]
        ratio: Option<RecipeMultiShotRatio>,
    },
}

impl RecipeMultiShotVideoRequest {
    /// Create an automatic multi-shot request.
    pub fn auto(prompt: impl Into<String>, version: RecipeVersion) -> Self {
        Self::Auto {
            prompt: prompt.into(),
            version,
            audio: None,
            duration: None,
            first_frame: None,
            ratio: None,
        }
    }

    /// Create a custom-shot multi-shot request.
    pub fn custom(shots: Vec<RecipeMultiShot>, version: RecipeVersion) -> Self {
        Self::Custom {
            shots,
            version,
            audio: None,
            duration: None,
            first_frame: None,
            ratio: None,
        }
    }

    /// Configure whether the recipe generates audio.
    pub fn audio(mut self, enabled: bool) -> Self {
        match &mut self {
            Self::Auto { audio, .. } | Self::Custom { audio, .. } => *audio = Some(enabled),
        }
        self
    }

    /// Set the total output duration.
    pub fn duration(mut self, value: RecipeMultiShotDuration) -> Self {
        match &mut self {
            Self::Auto { duration, .. } | Self::Custom { duration, .. } => *duration = Some(value),
        }
        self
    }

    /// Set an image to use as the first output frame.
    pub fn first_frame(mut self, value: RecipeImage) -> Self {
        match &mut self {
            Self::Auto { first_frame, .. } | Self::Custom { first_frame, .. } => {
                *first_frame = Some(value)
            }
        }
        self
    }

    /// Set the output dimensions.
    pub fn ratio(mut self, value: RecipeMultiShotRatio) -> Self {
        match &mut self {
            Self::Auto { ratio, .. } | Self::Custom { ratio, .. } => *ratio = Some(value),
        }
        self
    }
}

/// Output dimensions supported by the product-ad recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeProductAdRatio {
    /// 1280 by 720 output.
    #[serde(rename = "1280:720")]
    Landscape720p,
    /// 720 by 1280 output.
    #[serde(rename = "720:1280")]
    Portrait720p,
    /// 960 by 960 output.
    #[serde(rename = "960:960")]
    Square720p,
    /// 834 by 1112 output.
    #[serde(rename = "834:1112")]
    Portrait834x1112,
    /// 1920 by 1080 output.
    #[serde(rename = "1920:1080")]
    Landscape1080p,
    /// 1080 by 1920 output.
    #[serde(rename = "1080:1920")]
    Portrait1080p,
    /// 1440 by 1440 output.
    #[serde(rename = "1440:1440")]
    Square1080p,
    /// 1248 by 1664 output.
    #[serde(rename = "1248:1664")]
    Portrait1248x1664,
}

/// Request to generate a cinematic product advertisement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeProductAdRequest {
    /// One to ten images showing the same product.
    pub product_images: Vec<RecipeImage>,
    /// Recipe workflow version.
    pub version: ProductAdRecipeVersion,
    /// Whether to generate audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Output duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    /// Product description and specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<String>,
    /// Output dimensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<RecipeProductAdRatio>,
    /// Optional visual-style reference images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_images: Option<Vec<RecipeImage>>,
    /// Optional creative direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_concept: Option<String>,
}

impl RecipeProductAdRequest {
    /// Create a product-ad request.
    pub fn new(product_images: Vec<RecipeImage>, version: ProductAdRecipeVersion) -> Self {
        Self {
            product_images,
            version,
            audio: None,
            duration: None,
            product_info: None,
            ratio: None,
            style_images: None,
            user_concept: None,
        }
    }

    /// Configure whether the recipe generates audio.
    pub fn audio(mut self, enabled: bool) -> Self {
        self.audio = Some(enabled);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Add product details that inform creative direction.
    pub fn product_info(mut self, product_info: impl Into<String>) -> Self {
        self.product_info = Some(product_info.into());
        self
    }

    /// Set the output dimensions.
    pub fn ratio(mut self, ratio: RecipeProductAdRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Add visual-style reference images.
    pub fn style_images(mut self, style_images: Vec<RecipeImage>) -> Self {
        self.style_images = Some(style_images);
        self
    }

    /// Add caller-provided creative direction.
    pub fn user_concept(mut self, user_concept: impl Into<String>) -> Self {
        self.user_concept = Some(user_concept.into());
        self
    }
}

/// Request to generate a fashion product campaign image set.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeProductCampaignImageRequest {
    /// Product image to preserve across the generated campaign.
    pub image: RecipeImage,
    /// Campaign style and creative brief.
    pub prompt: String,
    /// Recipe workflow version.
    pub version: RecipeVersion,
}

impl RecipeProductCampaignImageRequest {
    /// Create a product-campaign-image request.
    pub fn new(image: RecipeImage, prompt: impl Into<String>, version: RecipeVersion) -> Self {
        Self {
            image,
            prompt: prompt.into(),
            version,
        }
    }
}

/// View label for a product-swap reference image.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RecipeProductView {
    /// Front view.
    Front,
    /// Side view.
    Side,
    /// Back view.
    Back,
}

/// New-product reference image used by the product-swap recipe.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeProductSwapImage {
    /// HTTPS URL of the product image.
    pub uri: String,
    /// Optional view label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<RecipeProductView>,
}

impl RecipeProductSwapImage {
    /// Create a product-swap reference image.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            view: None,
        }
    }

    /// Label the product view shown by this image.
    pub fn view(mut self, view: RecipeProductView) -> Self {
        self.view = Some(view);
        self
    }
}

/// Output resolution supported by the product-swap recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeProductSwapResolution {
    /// 720p output.
    #[serde(rename = "720p")]
    P720,
    /// 1080p output.
    #[serde(rename = "1080p")]
    P1080,
}

/// Request to replace a product in a reference video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeProductSwapRequest {
    /// One to ten images of the replacement product.
    pub new_product_images: Vec<RecipeProductSwapImage>,
    /// Image of the product being replaced.
    pub original_product_image: RecipeImage,
    /// Video containing the product to replace.
    pub reference_video: RecipeVideo,
    /// Recipe workflow version.
    pub version: RecipeVersion,
    /// Whether to generate audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Output duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    /// Output video resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<RecipeProductSwapResolution>,
}

impl RecipeProductSwapRequest {
    /// Create a product-swap request.
    pub fn new(
        new_product_images: Vec<RecipeProductSwapImage>,
        original_product_image: RecipeImage,
        reference_video: RecipeVideo,
        version: RecipeVersion,
    ) -> Self {
        Self {
            new_product_images,
            original_product_image,
            reference_video,
            version,
            audio: None,
            duration: None,
            resolution: None,
        }
    }

    /// Configure whether the recipe generates audio.
    pub fn audio(mut self, enabled: bool) -> Self {
        self.audio = Some(enabled);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the output video resolution.
    pub fn resolution(mut self, resolution: RecipeProductSwapResolution) -> Self {
        self.resolution = Some(resolution);
        self
    }
}

/// Output dimensions supported by the product-UGC recipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeProductUgcRatio {
    /// 720 by 1280 portrait output.
    #[serde(rename = "720:1280")]
    Portrait720p,
    /// 1080 by 1920 portrait output.
    #[serde(rename = "1080:1920")]
    Portrait1080p,
}

/// Request to generate a vertical user-generated-content product advertisement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeProductUgcRequest {
    /// Image of the on-camera character.
    pub character_image: RecipeImage,
    /// Image of the promoted product.
    pub product_image: RecipeImage,
    /// Recipe workflow version.
    pub version: RecipeVersion,
    /// Whether to generate audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Output duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u8>,
    /// Product details and creative brief.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<String>,
    /// Output dimensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<RecipeProductUgcRatio>,
    /// Optional direction for tone, message, or dialogue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_concept: Option<String>,
}

impl RecipeProductUgcRequest {
    /// Create a product-UGC request.
    pub fn new(
        character_image: RecipeImage,
        product_image: RecipeImage,
        version: RecipeVersion,
    ) -> Self {
        Self {
            character_image,
            product_image,
            version,
            audio: None,
            duration: None,
            product_info: None,
            ratio: None,
            user_concept: None,
        }
    }

    /// Configure whether the recipe generates audio.
    pub fn audio(mut self, enabled: bool) -> Self {
        self.audio = Some(enabled);
        self
    }

    /// Set the output duration in seconds.
    pub fn duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Add product details and a creative brief.
    pub fn product_info(mut self, product_info: impl Into<String>) -> Self {
        self.product_info = Some(product_info.into());
        self
    }

    /// Set the output dimensions.
    pub fn ratio(mut self, ratio: RecipeProductUgcRatio) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Add caller-provided creative direction.
    pub fn user_concept(mut self, user_concept: impl Into<String>) -> Self {
        self.user_concept = Some(user_concept.into());
        self
    }
}
