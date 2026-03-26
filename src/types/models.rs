use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageModel {
    #[serde(rename = "gen4_image_turbo")]
    Gen4ImageTurbo,
    #[serde(rename = "gen4_image")]
    Gen4Image,
    #[serde(rename = "gemini_2.5_flash")]
    Gemini25Flash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VideoRatio {
    #[serde(rename = "1280:720")]
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

impl Default for VideoRatio {
    fn default() -> Self {
        Self::Landscape
    }
}
