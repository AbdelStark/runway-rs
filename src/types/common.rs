use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ContentModeration {
    Automatic,
}

impl Default for ContentModeration {
    fn default() -> Self {
        Self::Automatic
    }
}
