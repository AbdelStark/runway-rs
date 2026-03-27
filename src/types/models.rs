use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoModel {
    #[serde(rename = "gen4.5")]
    Gen45,
    #[serde(rename = "gen4_turbo")]
    Gen4Turbo,
    #[serde(rename = "gen3a_turbo")]
    Gen3aTurbo,
    #[serde(rename = "gen4_aleph")]
    Gen4Aleph,
    #[serde(rename = "veo3.1")]
    Veo31,
    #[serde(rename = "veo3.1_fast")]
    Veo31Fast,
    #[serde(rename = "veo3")]
    Veo3,
}

impl fmt::Display for VideoModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gen45 => write!(f, "gen4.5"),
            Self::Gen4Turbo => write!(f, "gen4_turbo"),
            Self::Gen3aTurbo => write!(f, "gen3a_turbo"),
            Self::Gen4Aleph => write!(f, "gen4_aleph"),
            Self::Veo31 => write!(f, "veo3.1"),
            Self::Veo31Fast => write!(f, "veo3.1_fast"),
            Self::Veo3 => write!(f, "veo3"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageModel {
    #[serde(rename = "gen4_image_turbo")]
    Gen4ImageTurbo,
    #[serde(rename = "gen4_image")]
    Gen4Image,
    #[serde(rename = "gemini_2.5_flash")]
    Gemini25Flash,
}

impl fmt::Display for ImageModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gen4ImageTurbo => write!(f, "gen4_image_turbo"),
            Self::Gen4Image => write!(f, "gen4_image"),
            Self::Gemini25Flash => write!(f, "gemini_2.5_flash"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoRatio {
    #[serde(rename = "1280:720")]
    #[default]
    Landscape,
    #[serde(rename = "720:1280")]
    Portrait,
    #[serde(rename = "1104:832")]
    Wide,
    #[serde(rename = "960:960")]
    Square,
    #[serde(rename = "832:1104")]
    Tall,
    #[serde(rename = "1584:672")]
    Ultrawide,
    #[serde(rename = "1920:1080")]
    HdLandscape,
    #[serde(rename = "1080:1920")]
    HdPortrait,
    #[serde(rename = "1280:768")]
    Gen3Landscape,
    #[serde(rename = "768:1280")]
    Gen3Portrait,
    #[serde(rename = "848:480")]
    SdLandscape,
    #[serde(rename = "640:480")]
    SdClassic,
}

impl fmt::Display for VideoRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Landscape => write!(f, "1280:720"),
            Self::Portrait => write!(f, "720:1280"),
            Self::Wide => write!(f, "1104:832"),
            Self::Square => write!(f, "960:960"),
            Self::Tall => write!(f, "832:1104"),
            Self::Ultrawide => write!(f, "1584:672"),
            Self::HdLandscape => write!(f, "1920:1080"),
            Self::HdPortrait => write!(f, "1080:1920"),
            Self::Gen3Landscape => write!(f, "1280:768"),
            Self::Gen3Portrait => write!(f, "768:1280"),
            Self::SdLandscape => write!(f, "848:480"),
            Self::SdClassic => write!(f, "640:480"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageRatio {
    #[serde(rename = "1024:1024")]
    Square1024,
    #[serde(rename = "1080:1080")]
    Square1080,
    #[serde(rename = "1168:880")]
    Wide1168x880,
    #[serde(rename = "1360:768")]
    Landscape1360x768,
    #[serde(rename = "1440:1080")]
    Landscape1440x1080,
    #[serde(rename = "1080:1440")]
    Portrait1080x1440,
    #[serde(rename = "1808:768")]
    Ultrawide1808x768,
    #[serde(rename = "1920:1080")]
    HdLandscape,
    #[serde(rename = "1080:1920")]
    HdPortrait,
    #[serde(rename = "2112:912")]
    Ultrawide2112x912,
    #[serde(rename = "1280:720")]
    Landscape,
    #[serde(rename = "720:1280")]
    Portrait,
    #[serde(rename = "720:720")]
    Square720,
    #[serde(rename = "960:720")]
    Landscape960x720,
    #[serde(rename = "720:960")]
    Portrait720x960,
    #[serde(rename = "1680:720")]
    Ultrawide1680x720,
    #[serde(rename = "1344:768")]
    GeminiLandscape1344x768,
    #[serde(rename = "768:1344")]
    GeminiPortrait768x1344,
    #[serde(rename = "1184:864")]
    GeminiLandscape1184x864,
    #[serde(rename = "864:1184")]
    GeminiPortrait864x1184,
    #[serde(rename = "1536:672")]
    GeminiUltrawide1536x672,
    #[serde(rename = "832:1248")]
    GeminiPortrait832x1248,
    #[serde(rename = "1248:832")]
    GeminiLandscape1248x832,
    #[serde(rename = "896:1152")]
    GeminiPortrait896x1152,
    #[serde(rename = "1152:896")]
    GeminiLandscape1152x896,
}

impl fmt::Display for ImageRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Square1024 => write!(f, "1024:1024"),
            Self::Square1080 => write!(f, "1080:1080"),
            Self::Wide1168x880 => write!(f, "1168:880"),
            Self::Landscape1360x768 => write!(f, "1360:768"),
            Self::Landscape1440x1080 => write!(f, "1440:1080"),
            Self::Portrait1080x1440 => write!(f, "1080:1440"),
            Self::Ultrawide1808x768 => write!(f, "1808:768"),
            Self::HdLandscape => write!(f, "1920:1080"),
            Self::HdPortrait => write!(f, "1080:1920"),
            Self::Ultrawide2112x912 => write!(f, "2112:912"),
            Self::Landscape => write!(f, "1280:720"),
            Self::Portrait => write!(f, "720:1280"),
            Self::Square720 => write!(f, "720:720"),
            Self::Landscape960x720 => write!(f, "960:720"),
            Self::Portrait720x960 => write!(f, "720:960"),
            Self::Ultrawide1680x720 => write!(f, "1680:720"),
            Self::GeminiLandscape1344x768 => write!(f, "1344:768"),
            Self::GeminiPortrait768x1344 => write!(f, "768:1344"),
            Self::GeminiLandscape1184x864 => write!(f, "1184:864"),
            Self::GeminiPortrait864x1184 => write!(f, "864:1184"),
            Self::GeminiUltrawide1536x672 => write!(f, "1536:672"),
            Self::GeminiPortrait832x1248 => write!(f, "832:1248"),
            Self::GeminiLandscape1248x832 => write!(f, "1248:832"),
            Self::GeminiPortrait896x1152 => write!(f, "896:1152"),
            Self::GeminiLandscape1152x896 => write!(f, "1152:896"),
        }
    }
}

impl From<VideoRatio> for ImageRatio {
    fn from(value: VideoRatio) -> Self {
        match value {
            VideoRatio::Landscape => Self::Landscape,
            VideoRatio::Portrait => Self::Portrait,
            VideoRatio::HdLandscape => Self::HdLandscape,
            VideoRatio::HdPortrait => Self::HdPortrait,
            VideoRatio::Wide => Self::Wide1168x880,
            VideoRatio::Square => Self::Square1024,
            VideoRatio::Tall => Self::Portrait1080x1440,
            VideoRatio::Ultrawide => Self::Ultrawide2112x912,
            VideoRatio::Gen3Landscape => Self::Landscape1360x768,
            VideoRatio::Gen3Portrait => Self::GeminiPortrait768x1344,
            VideoRatio::SdLandscape => Self::Landscape960x720,
            VideoRatio::SdClassic => Self::Square720,
        }
    }
}
