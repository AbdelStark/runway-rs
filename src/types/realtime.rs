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

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

impl fmt::Debug for RealtimeSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotReady {
                id,
                created_at,
                queued,
            } => f
                .debug_struct("RealtimeSession::NotReady")
                .field("id", id)
                .field("created_at", created_at)
                .field("queued", queued)
                .finish(),
            Self::Ready {
                id,
                created_at,
                expires_at,
                ..
            } => f
                .debug_struct("RealtimeSession::Ready")
                .field("id", id)
                .field("created_at", created_at)
                .field("expires_at", expires_at)
                .field("session_key", &"[REDACTED]")
                .finish(),
            Self::Running { id, created_at } => f
                .debug_struct("RealtimeSession::Running")
                .field("id", id)
                .field("created_at", created_at)
                .finish(),
            Self::Completed {
                id,
                created_at,
                duration,
            } => f
                .debug_struct("RealtimeSession::Completed")
                .field("id", id)
                .field("created_at", created_at)
                .field("duration", duration)
                .finish(),
            Self::Failed {
                id,
                created_at,
                failure,
                failure_code,
            } => f
                .debug_struct("RealtimeSession::Failed")
                .field("id", id)
                .field("created_at", created_at)
                .field("failure", failure)
                .field("failure_code", failure_code)
                .finish(),
            Self::Cancelled { id, created_at } => f
                .debug_struct("RealtimeSession::Cancelled")
                .field("id", id)
                .field("created_at", created_at)
                .finish(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateRealtimeSessionRequest {
    pub avatar: RealtimeAvatarInput,
    pub model: RealtimeSessionModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<RealtimeIntegration>,
    /// Deprecated upstream compatibility field. Prefer [`integration`](Self::integration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livekit: Option<RealtimeLiveKitConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_script: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tools: Vec<RealtimeTool>,
}

impl CreateRealtimeSessionRequest {
    pub fn new(avatar: RealtimeAvatarInput) -> Self {
        Self {
            avatar,
            model: RealtimeSessionModel::Gwm1Avatars,
            integration: None,
            livekit: None,
            max_duration: None,
            personality: None,
            start_script: None,
            tools: Vec::new(),
        }
    }

    pub fn integration(mut self, integration: RealtimeIntegration) -> Self {
        self.integration = Some(integration);
        self
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

    pub fn tool(mut self, tool: RealtimeTool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn tools(mut self, tools: impl IntoIterator<Item = RealtimeTool>) -> Self {
        self.tools.extend(tools);
        self
    }

    pub fn validate(&self) -> Result<(), crate::RunwayError> {
        for tool in &self.tools {
            tool.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RealtimeAvatarInput {
    #[serde(rename = "runway-preset")]
    RunwayPreset {
        #[serde(rename = "presetId")]
        preset_id: RealtimePresetAvatarId,
    },
    #[serde(rename = "custom")]
    Custom {
        #[serde(rename = "avatarId")]
        avatar_id: String,
    },
}

impl RealtimeAvatarInput {
    pub fn runway_preset(preset_id: RealtimePresetAvatarId) -> Self {
        Self::RunwayPreset { preset_id }
    }

    pub fn custom(avatar_id: impl Into<String>) -> Self {
        Self::Custom {
            avatar_id: avatar_id.into(),
        }
    }
}

/// Official preset avatars available to realtime sessions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RealtimePresetAvatarId {
    GameCharacter,
    MusicSuperstar,
    GameCharacterMan,
    CatCharacter,
    Influencer,
    TennisCoach,
    HumanResource,
    FashionDesigner,
    CookingTeacher,
}

impl fmt::Display for RealtimePresetAvatarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::GameCharacter => "game-character",
            Self::MusicSuperstar => "music-superstar",
            Self::GameCharacterMan => "game-character-man",
            Self::CatCharacter => "cat-character",
            Self::Influencer => "influencer",
            Self::TennisCoach => "tennis-coach",
            Self::HumanResource => "human-resource",
            Self::FashionDesigner => "fashion-designer",
            Self::CookingTeacher => "cooking-teacher",
        };
        f.write_str(value)
    }
}

/// External conversation or audio transport used by a realtime avatar session.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeIntegration {
    /// ElevenLabs owns the conversation while Runway renders avatar video.
    Elevenlabs {
        #[serde(rename = "signedUrl")]
        signed_url: String,
    },
    /// Runway joins an external LiveKit room and publishes avatar video.
    Livekit {
        token: String,
        #[serde(rename = "roomName")]
        room_name: String,
        url: String,
        #[serde(rename = "agentIdentity", skip_serializing_if = "Option::is_none")]
        agent_identity: Option<String>,
    },
}

impl RealtimeIntegration {
    pub fn elevenlabs(signed_url: impl Into<String>) -> Self {
        Self::Elevenlabs {
            signed_url: signed_url.into(),
        }
    }

    pub fn livekit(
        token: impl Into<String>,
        room_name: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self::Livekit {
            token: token.into(),
            room_name: room_name.into(),
            url: url.into(),
            agent_identity: None,
        }
    }

    pub fn agent_identity(mut self, identity: impl Into<String>) -> Self {
        if let Self::Livekit { agent_identity, .. } = &mut self {
            *agent_identity = Some(identity.into());
        }
        self
    }
}

impl fmt::Debug for RealtimeIntegration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Elevenlabs { .. } => f
                .debug_struct("RealtimeIntegration::Elevenlabs")
                .field("signed_url", &"[REDACTED]")
                .finish(),
            Self::Livekit {
                room_name,
                agent_identity,
                ..
            } => f
                .debug_struct("RealtimeIntegration::Livekit")
                .field("token", &"[REDACTED]")
                .field("room_name", room_name)
                .field("url", &"[REDACTED]")
                .field("agent_identity", agent_identity)
                .finish(),
        }
    }
}

/// Legacy LiveKit configuration retained for wire compatibility.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeLiveKitConfig {
    pub token: String,
    pub room_name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_identity: Option<String>,
}

impl fmt::Debug for RealtimeLiveKitConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RealtimeLiveKitConfig")
            .field("token", &"[REDACTED]")
            .field("room_name", &self.room_name)
            .field("url", &"[REDACTED]")
            .field("agent_identity", &self.agent_identity)
            .finish()
    }
}

/// Tool callable by an avatar during a realtime session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeTool {
    /// Fire-and-forget event delivered to the frontend client.
    ClientEvent {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        parameters: Vec<RealtimeToolParameter>,
    },
    /// Round-trip call to the configured backend.
    BackendRpc {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        parameters: Vec<RealtimeToolParameter>,
        #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
        timeout_seconds: Option<f64>,
    },
}

impl RealtimeTool {
    pub fn client_event(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::ClientEvent {
            description: description.into(),
            name: name.into(),
            parameters: Vec::new(),
        }
    }

    pub fn backend_rpc(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::BackendRpc {
            description: description.into(),
            name: name.into(),
            parameters: Vec::new(),
            timeout_seconds: None,
        }
    }

    pub fn parameter(mut self, parameter: RealtimeToolParameter) -> Self {
        match &mut self {
            Self::ClientEvent { parameters, .. } | Self::BackendRpc { parameters, .. } => {
                parameters.push(parameter);
            }
        }
        self
    }

    pub fn timeout_seconds(mut self, timeout_seconds: f64) -> Self {
        if let Self::BackendRpc {
            timeout_seconds: value,
            ..
        } = &mut self
        {
            *value = Some(timeout_seconds);
        }
        self
    }

    fn validate(&self) -> Result<(), crate::RunwayError> {
        let (name, parameters, timeout) = match self {
            Self::ClientEvent {
                name, parameters, ..
            } => (name, parameters, None),
            Self::BackendRpc {
                name,
                parameters,
                timeout_seconds,
                ..
            } => (name, parameters, *timeout_seconds),
        };
        if !is_valid_tool_name(name) {
            return Err(crate::RunwayError::Validation {
                message: format!(
                    "Realtime tool name {name:?} must start with a letter or underscore and contain only ASCII letters, digits, or underscores"
                ),
            });
        }
        if timeout.is_some_and(|value| !value.is_finite() || value <= 0.0) {
            return Err(crate::RunwayError::Validation {
                message: "Realtime backend RPC timeout must be finite and greater than zero".into(),
            });
        }
        for parameter in parameters {
            parameter.validate()?;
        }
        Ok(())
    }
}

fn is_valid_tool_name(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// Typed schema for a realtime tool parameter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeToolParameter {
    String {
        description: String,
        name: String,
        #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
        allowed_values: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
    Integer {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
    Number {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
    Boolean {
        description: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
    Array {
        description: String,
        items: RealtimeArrayItemSchema,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
    Object {
        description: String,
        name: String,
        properties: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
}

impl RealtimeToolParameter {
    pub fn string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::String {
            description: description.into(),
            name: name.into(),
            allowed_values: None,
            required: None,
        }
    }

    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::Integer {
            description: description.into(),
            name: name.into(),
            required: None,
        }
    }

    pub fn number(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::Number {
            description: description.into(),
            name: name.into(),
            required: None,
        }
    }

    pub fn boolean(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::Boolean {
            description: description.into(),
            name: name.into(),
            required: None,
        }
    }

    pub fn array(
        name: impl Into<String>,
        description: impl Into<String>,
        item_type: RealtimeArrayItemType,
    ) -> Self {
        Self::Array {
            description: description.into(),
            items: RealtimeArrayItemSchema { item_type },
            name: name.into(),
            required: None,
        }
    }

    pub fn object(
        name: impl Into<String>,
        description: impl Into<String>,
        properties: Vec<serde_json::Value>,
    ) -> Self {
        Self::Object {
            description: description.into(),
            name: name.into(),
            properties,
            required: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        match &mut self {
            Self::String {
                required: value, ..
            }
            | Self::Integer {
                required: value, ..
            }
            | Self::Number {
                required: value, ..
            }
            | Self::Boolean {
                required: value, ..
            }
            | Self::Array {
                required: value, ..
            }
            | Self::Object {
                required: value, ..
            } => *value = Some(required),
        }
        self
    }

    pub fn allowed_values(mut self, values: impl IntoIterator<Item = String>) -> Self {
        if let Self::String { allowed_values, .. } = &mut self {
            *allowed_values = Some(values.into_iter().collect());
        }
        self
    }

    fn validate(&self) -> Result<(), crate::RunwayError> {
        let name = match self {
            Self::String { name, .. }
            | Self::Integer { name, .. }
            | Self::Number { name, .. }
            | Self::Boolean { name, .. }
            | Self::Array { name, .. }
            | Self::Object { name, .. } => name,
        };
        if !is_valid_tool_name(name) {
            return Err(crate::RunwayError::Validation {
                message: format!("Invalid realtime tool parameter name: {name:?}"),
            });
        }
        Ok(())
    }
}

/// Element type for an array-valued realtime tool parameter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeArrayItemType {
    String,
    Integer,
    Number,
    Boolean,
}

/// Schema for elements of an array-valued tool parameter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RealtimeArrayItemSchema {
    #[serde(rename = "type")]
    pub item_type: RealtimeArrayItemType,
}
