use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeSessionCreateResponse {
    pub id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RealtimeSessionStatus {
    NotReady,
    Ready,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl fmt::Display for RealtimeSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotReady => write!(f, "NOT_READY"),
            Self::Ready => write!(f, "READY"),
            Self::Running => write!(f, "RUNNING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RealtimeSession {
    NotReady {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(default)]
        queued: Option<bool>,
    },
    Ready {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "expiresAt")]
        expires_at: String,
        #[serde(rename = "sessionKey")]
        session_key: String,
    },
    Running {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Completed {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        duration: f64,
    },
    Failed {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        failure: String,
        #[serde(rename = "failureCode")]
        failure_code: String,
    },
    Cancelled {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
}

impl RealtimeSession {
    pub fn id(&self) -> &str {
        match self {
            Self::NotReady { id, .. }
            | Self::Ready { id, .. }
            | Self::Running { id, .. }
            | Self::Completed { id, .. }
            | Self::Failed { id, .. }
            | Self::Cancelled { id, .. } => id,
        }
    }

    pub fn status(&self) -> RealtimeSessionStatus {
        match self {
            Self::NotReady { .. } => RealtimeSessionStatus::NotReady,
            Self::Ready { .. } => RealtimeSessionStatus::Ready,
            Self::Running { .. } => RealtimeSessionStatus::Running,
            Self::Completed { .. } => RealtimeSessionStatus::Completed,
            Self::Failed { .. } => RealtimeSessionStatus::Failed,
            Self::Cancelled { .. } => RealtimeSessionStatus::Cancelled,
        }
    }

    pub fn session_key(&self) -> Option<&str> {
        match self {
            Self::Ready { session_key, .. } => Some(session_key),
            _ => None,
        }
    }

    pub fn failure(&self) -> Option<&str> {
        match self {
            Self::Failed { failure, .. } => Some(failure),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RealtimeSessionModel {
    #[serde(rename = "gwm1_avatars")]
    Gwm1Avatars,
}

impl fmt::Display for RealtimeSessionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gwm1Avatars => write!(f, "gwm1_avatars"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateRealtimeSessionRequest {
    pub avatar: RealtimeAvatarInput,
    pub model: RealtimeSessionModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_script: Option<String>,
}

impl CreateRealtimeSessionRequest {
    pub fn new(avatar: RealtimeAvatarInput) -> Self {
        Self {
            avatar,
            model: RealtimeSessionModel::Gwm1Avatars,
            max_duration: None,
            personality: None,
            start_script: None,
        }
    }

    pub fn max_duration(mut self, max_duration: u32) -> Self {
        self.max_duration = Some(max_duration);
        self
    }

    pub fn personality(mut self, personality: impl Into<String>) -> Self {
        self.personality = Some(personality.into());
        self
    }

    pub fn start_script(mut self, start_script: impl Into<String>) -> Self {
        self.start_script = Some(start_script.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RealtimeAvatarInput {
    #[serde(rename = "runway-preset")]
    RunwayPreset {
        #[serde(rename = "presetId")]
        preset_id: String,
    },
    #[serde(rename = "custom")]
    Custom {
        #[serde(rename = "avatarId")]
        avatar_id: String,
    },
}

impl RealtimeAvatarInput {
    pub fn runway_preset(preset_id: impl Into<String>) -> Self {
        Self::RunwayPreset {
            preset_id: preset_id.into(),
        }
    }

    pub fn custom(avatar_id: impl Into<String>) -> Self {
        Self::Custom {
            avatar_id: avatar_id.into(),
        }
    }
}
