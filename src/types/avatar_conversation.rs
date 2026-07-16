//! Request and response models for realtime avatar conversation history.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A cursor-paginated collection of avatar conversation summaries.
pub type AvatarConversationList = crate::types::common::CursorPage<AvatarConversationSummary>;

/// Filters accepted when listing avatar conversations.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationListQuery {
    /// Cursor returned by the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of conversations to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Restrict results to a custom avatar ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Return conversations created before this exclusive timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Return conversations created at or after this inclusive timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl AvatarConversationListQuery {
    /// Create an empty conversation-list query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Continue listing from a cursor returned by the preceding page.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Limit the number of conversations returned by the API.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Restrict results to a custom avatar ID.
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Restrict results to conversations before an exclusive timestamp.
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Restrict results to conversations at or after an inclusive timestamp.
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Validate query constraints enforced before sending a request.
    pub fn validate(&self) -> Result<(), crate::error::RunwayError> {
        if self.limit == Some(0) {
            return Err(crate::error::RunwayError::Validation {
                message: "Avatar conversation list limit must be greater than zero".into(),
            });
        }
        Ok(())
    }
}

/// Lifecycle status of an avatar conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AvatarConversationStatus {
    /// The realtime session is active.
    InProgress,
    /// The conversation completed successfully.
    Ended,
    /// The conversation ended because of an error.
    Failed,
}

impl fmt::Display for AvatarConversationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Ended => write!(f, "ended"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

/// Avatar identity embedded in a full conversation response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AvatarConversationAvatar {
    /// A Runway-provided preset avatar.
    #[serde(rename = "runway-preset")]
    RunwayPreset {
        /// Preset avatar identifier.
        #[serde(rename = "presetId")]
        preset_id: String,
    },
    /// A user-created avatar, which may since have been deleted.
    #[serde(rename = "custom")]
    Custom {
        /// Custom avatar identifier, or `None` if it was deleted.
        id: Option<String>,
        /// Avatar image URL, or `None` when unavailable.
        #[serde(rename = "imageUrl")]
        image_url: Option<String>,
        /// Avatar name, or `None` when unavailable.
        name: Option<String>,
    },
}

/// Avatar identity embedded in a conversation-list item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AvatarConversationSummaryAvatar {
    /// A Runway-provided preset avatar.
    #[serde(rename = "runway-preset")]
    RunwayPreset {
        /// Display name of the preset avatar.
        name: String,
        /// Preset avatar identifier.
        #[serde(rename = "presetId")]
        preset_id: String,
    },
    /// A user-created avatar, which may since have been deleted.
    #[serde(rename = "custom")]
    Custom {
        /// Custom avatar identifier, or `None` if it was deleted.
        id: Option<String>,
        /// Avatar name, or `None` when unavailable.
        name: Option<String>,
    },
}

/// Type of tool configured for an avatar conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AvatarConversationToolType {
    /// A fire-and-forget event delivered to the frontend client.
    ClientEvent,
    /// A request-response call made to a backend service.
    BackendRpc,
}

/// Summary of a tool configured for a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvatarConversationTool {
    /// Description explaining when and how the tool is used.
    pub description: String,
    /// Tool name.
    pub name: String,
    /// Tool transport type.
    #[serde(rename = "type")]
    pub tool_type: AvatarConversationToolType,
}

/// Speaker that produced a transcript entry.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AvatarConversationTranscriptRole {
    /// The human participant.
    User,
    /// The avatar assistant.
    Assistant,
}

/// A tool invocation emitted by the avatar assistant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationToolCall {
    /// Arguments supplied to the tool.
    pub arguments: HashMap<String, Value>,
    /// Tool name.
    pub name: String,
    /// Optional identifier linking this call to its result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Result returned by a conversation tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AvatarConversationToolResultValue {
    /// Structured JSON object returned by a tool.
    Object(HashMap<String, Value>),
    /// Plain-text result returned by a tool.
    String(String),
}

/// Result returned by a conversation tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationToolResult {
    /// Tool name.
    pub name: String,
    /// Optional identifier linking this result to its call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tool-call duration in milliseconds when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    /// Error text when the tool failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Object or string tool result; `None` also represents an absent or null result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AvatarConversationToolResultValue>,
}

/// One entry in an avatar conversation transcript.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationTranscriptEntry {
    /// Spoken text, or `None` for a tool-only turn.
    pub content: Option<String>,
    /// Participant that produced the entry.
    pub role: AvatarConversationTranscriptRole,
    /// Entry timestamp when available.
    pub timestamp: Option<String>,
    /// Tool calls made during this assistant turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<AvatarConversationToolCall>>,
    /// Tool results received during this assistant turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_results: Option<Vec<AvatarConversationToolResult>>,
}

/// Full response for a conversation that is currently active.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationInProgress {
    /// Unique conversation identifier.
    pub id: String,
    /// Avatar used by the conversation, or `None` when unavailable.
    pub avatar: Option<AvatarConversationAvatar>,
    /// Creation timestamp.
    pub created_at: String,
    /// Elapsed duration in seconds, or `None` if not started.
    pub duration: Option<u64>,
    /// Maximum allowed duration in seconds, or `None` if unset.
    pub max_duration: Option<u64>,
    /// Conversation name.
    pub name: String,
    /// Expiring recording URL, or `None` when no recording is available.
    pub recording_url: Option<String>,
    /// Start timestamp, or `None` if not started.
    pub started_at: Option<String>,
    /// Tools configured for the session.
    pub tools: Vec<AvatarConversationTool>,
    /// Conversation transcript.
    pub transcript: Vec<AvatarConversationTranscriptEntry>,
}

/// Full response for a conversation that completed successfully.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationEnded {
    /// Unique conversation identifier.
    pub id: String,
    /// Avatar used by the conversation, or `None` when unavailable.
    pub avatar: Option<AvatarConversationAvatar>,
    /// Creation timestamp.
    pub created_at: String,
    /// Conversation duration in seconds, or `None` when unavailable.
    pub duration: Option<u64>,
    /// End timestamp, or `None` when unavailable.
    pub ended_at: Option<String>,
    /// Maximum allowed duration in seconds, or `None` if unset.
    pub max_duration: Option<u64>,
    /// Conversation name.
    pub name: String,
    /// Expiring recording URL, or `None` when no recording is available.
    pub recording_url: Option<String>,
    /// Start timestamp, or `None` when unavailable.
    pub started_at: Option<String>,
    /// Tools configured for the session.
    pub tools: Vec<AvatarConversationTool>,
    /// Conversation transcript.
    pub transcript: Vec<AvatarConversationTranscriptEntry>,
}

/// Full response for a conversation that ended because of an error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationFailed {
    /// Unique conversation identifier.
    pub id: String,
    /// Avatar used by the conversation, or `None` when unavailable.
    pub avatar: Option<AvatarConversationAvatar>,
    /// Creation timestamp.
    pub created_at: String,
    /// Conversation duration in seconds, or `None` if it never started.
    pub duration: Option<u64>,
    /// End timestamp, or `None` if it failed before starting.
    pub ended_at: Option<String>,
    /// Human-readable failure reason.
    pub failure: String,
    /// Machine-readable failure code.
    pub failure_code: String,
    /// Maximum allowed duration in seconds, or `None` if unset.
    pub max_duration: Option<u64>,
    /// Conversation name.
    pub name: String,
    /// Expiring recording URL, or `None` when no recording is available.
    pub recording_url: Option<String>,
    /// Start timestamp, or `None` if it failed before starting.
    pub started_at: Option<String>,
    /// Tools configured for the session.
    pub tools: Vec<AvatarConversationTool>,
    /// Conversation transcript.
    pub transcript: Vec<AvatarConversationTranscriptEntry>,
}

/// Detailed avatar conversation response, discriminated by `status`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AvatarConversation {
    /// The realtime conversation is currently active.
    InProgress(AvatarConversationInProgress),
    /// The conversation completed successfully.
    Ended(AvatarConversationEnded),
    /// The conversation ended because of an error.
    Failed(AvatarConversationFailed),
}

impl AvatarConversation {
    /// Return the unique conversation identifier.
    pub fn id(&self) -> &str {
        match self {
            Self::InProgress(value) => &value.id,
            Self::Ended(value) => &value.id,
            Self::Failed(value) => &value.id,
        }
    }

    /// Return the conversation lifecycle status.
    pub fn status(&self) -> AvatarConversationStatus {
        match self {
            Self::InProgress(_) => AvatarConversationStatus::InProgress,
            Self::Ended(_) => AvatarConversationStatus::Ended,
            Self::Failed(_) => AvatarConversationStatus::Failed,
        }
    }
}

/// Summary returned by the avatar-conversation list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvatarConversationSummary {
    /// Unique conversation identifier.
    pub id: String,
    /// Avatar used in the conversation, or `None` when unavailable.
    pub avatar: Option<AvatarConversationSummaryAvatar>,
    /// Creation timestamp.
    pub created_at: String,
    /// Duration in seconds, or `None` if the conversation has not started.
    pub duration: Option<u64>,
    /// Whether tools were configured for this conversation.
    pub has_tools: bool,
    /// Conversation name.
    pub name: String,
    /// Current conversation status.
    pub status: AvatarConversationStatus,
}
