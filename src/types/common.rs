use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ContentModeration {
    #[default]
    Automatic,
}
