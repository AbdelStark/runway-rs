use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoModel {
    #[serde(rename = "gen4.5")]
    Gen45,
    #[serde(rename = "gen4_turbo")]
    Gen4Turbo,
    /// Legacy compatibility model retained for pre-4.10 image-to-video callers.
    #[serde(rename = "gen3a_turbo")]
    Gen3aTurbo,
    /// Legacy compatibility model retained for pre-4.10 video-to-video callers.
    #[serde(rename = "gen4_aleph")]
    Gen4Aleph,
    #[serde(rename = "veo3.1")]
    Veo31,
    #[serde(rename = "veo3.1_fast")]
    Veo31Fast,
    #[serde(rename = "veo3")]
    Veo3,
    /// Happyhorse 1.0 video generation model.
    #[serde(rename = "happyhorse_1_0")]
    Happyhorse10,
    /// Seedance 2.0 video generation model.
    #[serde(rename = "seedance2")]
    Seedance2,
    /// Faster Seedance 2.0 video generation model.
    #[serde(rename = "seedance2_fast")]
    Seedance2Fast,
    /// Smaller Seedance 2.0 video generation model.
    #[serde(rename = "seedance2_mini")]
    Seedance2Mini,
    /// Gemini Omni Flash video generation and editing model.
    #[serde(rename = "gemini_omni_flash")]
    GeminiOmniFlash,
    /// Aleph 2 video editing model.
    #[serde(rename = "aleph2")]
    Aleph2,
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
            Self::Happyhorse10 => write!(f, "happyhorse_1_0"),
            Self::Seedance2 => write!(f, "seedance2"),
            Self::Seedance2Fast => write!(f, "seedance2_fast"),
            Self::Seedance2Mini => write!(f, "seedance2_mini"),
            Self::GeminiOmniFlash => write!(f, "gemini_omni_flash"),
            Self::Aleph2 => write!(f, "aleph2"),
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
    /// 1108-by-832 output resolution.
    #[serde(rename = "1108:832")]
    R1108x832,
    /// 832-by-1108 output resolution.
    #[serde(rename = "832:1108")]
    R832x1108,
    /// 1662-by-1248 output resolution.
    #[serde(rename = "1662:1248")]
    R1662x1248,
    /// 1248-by-1662 output resolution.
    #[serde(rename = "1248:1662")]
    R1248x1662,
    /// 992-by-432 output resolution.
    #[serde(rename = "992:432")]
    R992x432,
    /// 864-by-496 output resolution.
    #[serde(rename = "864:496")]
    R864x496,
    /// 752-by-560 output resolution.
    #[serde(rename = "752:560")]
    R752x560,
    /// 640-by-640 output resolution.
    #[serde(rename = "640:640")]
    R640x640,
    /// 560-by-752 output resolution.
    #[serde(rename = "560:752")]
    R560x752,
    /// 496-by-864 output resolution.
    #[serde(rename = "496:864")]
    R496x864,
    /// 1470-by-630 output resolution.
    #[serde(rename = "1470:630")]
    R1470x630,
    /// 1112-by-834 output resolution.
    #[serde(rename = "1112:834")]
    R1112x834,
    /// 834-by-1112 output resolution.
    #[serde(rename = "834:1112")]
    R834x1112,
    /// 2206-by-946 output resolution.
    #[serde(rename = "2206:946")]
    R2206x946,
    /// 1664-by-1248 output resolution.
    #[serde(rename = "1664:1248")]
    R1664x1248,
    /// 1440-by-1440 output resolution.
    #[serde(rename = "1440:1440")]
    R1440x1440,
    /// 1248-by-1664 output resolution.
    #[serde(rename = "1248:1664")]
    R1248x1664,
    /// 3840-by-1646 output resolution.
    #[serde(rename = "3840:1646")]
    R3840x1646,
    /// 3840-by-2160 output resolution.
    #[serde(rename = "3840:2160")]
    R3840x2160,
    /// 3840-by-2880 output resolution.
    #[serde(rename = "3840:2880")]
    R3840x2880,
    /// 3840-by-3840 output resolution.
    #[serde(rename = "3840:3840")]
    R3840x3840,
    /// 2880-by-3840 output resolution.
    #[serde(rename = "2880:3840")]
    R2880x3840,
    /// 2160-by-3840 output resolution.
    #[serde(rename = "2160:3840")]
    R2160x3840,
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
            Self::R1108x832 => write!(f, "1108:832"),
            Self::R832x1108 => write!(f, "832:1108"),
            Self::R1662x1248 => write!(f, "1662:1248"),
            Self::R1248x1662 => write!(f, "1248:1662"),
            Self::R992x432 => write!(f, "992:432"),
            Self::R864x496 => write!(f, "864:496"),
            Self::R752x560 => write!(f, "752:560"),
            Self::R640x640 => write!(f, "640:640"),
            Self::R560x752 => write!(f, "560:752"),
            Self::R496x864 => write!(f, "496:864"),
            Self::R1470x630 => write!(f, "1470:630"),
            Self::R1112x834 => write!(f, "1112:834"),
            Self::R834x1112 => write!(f, "834:1112"),
            Self::R2206x946 => write!(f, "2206:946"),
            Self::R1664x1248 => write!(f, "1664:1248"),
            Self::R1440x1440 => write!(f, "1440:1440"),
            Self::R1248x1664 => write!(f, "1248:1664"),
            Self::R3840x1646 => write!(f, "3840:1646"),
            Self::R3840x2160 => write!(f, "3840:2160"),
            Self::R3840x2880 => write!(f, "3840:2880"),
            Self::R3840x3840 => write!(f, "3840:3840"),
            Self::R2880x3840 => write!(f, "2880:3840"),
            Self::R2160x3840 => write!(f, "2160:3840"),
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
    /// 1108-by-832 resolution.
    #[serde(rename = "1108:832")]
    R1108x832,
    /// 832-by-1108 resolution.
    #[serde(rename = "832:1108")]
    R832x1108,
    /// 1662-by-1248 resolution.
    #[serde(rename = "1662:1248")]
    R1662x1248,
    /// 1248-by-1662 resolution.
    #[serde(rename = "1248:1662")]
    R1248x1662,
    /// 992-by-432 resolution.
    #[serde(rename = "992:432")]
    R992x432,
    /// 864-by-496 resolution.
    #[serde(rename = "864:496")]
    R864x496,
    /// 752-by-560 resolution.
    #[serde(rename = "752:560")]
    R752x560,
    /// 640-by-640 resolution.
    #[serde(rename = "640:640")]
    R640x640,
    /// 560-by-752 resolution.
    #[serde(rename = "560:752")]
    R560x752,
    /// 496-by-864 resolution.
    #[serde(rename = "496:864")]
    R496x864,
    /// 1470-by-630 resolution.
    #[serde(rename = "1470:630")]
    R1470x630,
    /// 1112-by-834 resolution.
    #[serde(rename = "1112:834")]
    R1112x834,
    /// 834-by-1112 resolution.
    #[serde(rename = "834:1112")]
    R834x1112,
    /// 2206-by-946 resolution.
    #[serde(rename = "2206:946")]
    R2206x946,
    /// 1664-by-1248 resolution.
    #[serde(rename = "1664:1248")]
    R1664x1248,
    /// 1440-by-1440 resolution.
    #[serde(rename = "1440:1440")]
    R1440x1440,
    /// 1248-by-1664 resolution.
    #[serde(rename = "1248:1664")]
    R1248x1664,
    /// 3840-by-1646 resolution.
    #[serde(rename = "3840:1646")]
    R3840x1646,
    /// 3840-by-2160 resolution.
    #[serde(rename = "3840:2160")]
    R3840x2160,
    /// 3840-by-2880 resolution.
    #[serde(rename = "3840:2880")]
    R3840x2880,
    /// 3840-by-3840 resolution.
    #[serde(rename = "3840:3840")]
    R3840x3840,
    /// 2880-by-3840 resolution.
    #[serde(rename = "2880:3840")]
    R2880x3840,
    /// 2160-by-3840 resolution.
    #[serde(rename = "2160:3840")]
    R2160x3840,
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
            Self::R1108x832 => write!(f, "1108:832"),
            Self::R832x1108 => write!(f, "832:1108"),
            Self::R1662x1248 => write!(f, "1662:1248"),
            Self::R1248x1662 => write!(f, "1248:1662"),
            Self::R992x432 => write!(f, "992:432"),
            Self::R864x496 => write!(f, "864:496"),
            Self::R752x560 => write!(f, "752:560"),
            Self::R640x640 => write!(f, "640:640"),
            Self::R560x752 => write!(f, "560:752"),
            Self::R496x864 => write!(f, "496:864"),
            Self::R1470x630 => write!(f, "1470:630"),
            Self::R1112x834 => write!(f, "1112:834"),
            Self::R834x1112 => write!(f, "834:1112"),
            Self::R2206x946 => write!(f, "2206:946"),
            Self::R1664x1248 => write!(f, "1664:1248"),
            Self::R1440x1440 => write!(f, "1440:1440"),
            Self::R1248x1664 => write!(f, "1248:1664"),
            Self::R3840x1646 => write!(f, "3840:1646"),
            Self::R3840x2160 => write!(f, "3840:2160"),
            Self::R3840x2880 => write!(f, "3840:2880"),
            Self::R3840x3840 => write!(f, "3840:3840"),
            Self::R2880x3840 => write!(f, "2880:3840"),
            Self::R2160x3840 => write!(f, "2160:3840"),
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
            VideoRatio::R1108x832 => Self::R1108x832,
            VideoRatio::R832x1108 => Self::R832x1108,
            VideoRatio::R1662x1248 => Self::R1662x1248,
            VideoRatio::R1248x1662 => Self::R1248x1662,
            VideoRatio::R992x432 => Self::R992x432,
            VideoRatio::R864x496 => Self::R864x496,
            VideoRatio::R752x560 => Self::R752x560,
            VideoRatio::R640x640 => Self::R640x640,
            VideoRatio::R560x752 => Self::R560x752,
            VideoRatio::R496x864 => Self::R496x864,
            VideoRatio::R1470x630 => Self::R1470x630,
            VideoRatio::R1112x834 => Self::R1112x834,
            VideoRatio::R834x1112 => Self::R834x1112,
            VideoRatio::R2206x946 => Self::R2206x946,
            VideoRatio::R1664x1248 => Self::R1664x1248,
            VideoRatio::R1440x1440 => Self::R1440x1440,
            VideoRatio::R1248x1664 => Self::R1248x1664,
            VideoRatio::R3840x1646 => Self::R3840x1646,
            VideoRatio::R3840x2160 => Self::R3840x2160,
            VideoRatio::R3840x2880 => Self::R3840x2880,
            VideoRatio::R3840x3840 => Self::R3840x3840,
            VideoRatio::R2880x3840 => Self::R2880x3840,
            VideoRatio::R2160x3840 => Self::R2160x3840,
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
