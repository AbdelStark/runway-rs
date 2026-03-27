use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoModel {
    #[serde(rename = "gen4.5")]
    Gen45,
    #[serde(rename = "gen4_turbo")]
    Gen4Turbo,
    #[serde(rename = "gen3a_turbo")]
    Gen3aTurbo,
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
            Self::Veo31 => write!(f, "veo3.1"),
            Self::Veo31Fast => write!(f, "veo3.1_fast"),
            Self::Veo3 => write!(f, "veo3"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        }
    }
}
