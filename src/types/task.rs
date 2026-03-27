use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TaskStatus {
    Pending,
    Throttled,
    Cancelled,
    Running,
    Succeeded,
    Failed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Throttled => write!(f, "THROTTLED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Running => write!(f, "RUNNING"),
            Self::Succeeded => write!(f, "SUCCEEDED"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Task {
    Pending {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Throttled {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Cancelled {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Running {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(default)]
        progress: Option<f64>,
    },
    Failed {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
        failure: String,
        #[serde(rename = "failureCode", default)]
        failure_code: Option<String>,
    },
    Succeeded {
        id: Uuid,
        #[serde(rename = "createdAt")]
        created_at: String,
        output: Vec<String>,
    },
}

impl Task {
    /// Returns the task identifier.
    pub fn id(&self) -> Uuid {
        match self {
            Self::Pending { id, .. }
            | Self::Throttled { id, .. }
            | Self::Cancelled { id, .. }
            | Self::Running { id, .. }
            | Self::Failed { id, .. }
            | Self::Succeeded { id, .. } => *id,
        }
    }

    /// Returns the task lifecycle status.
    pub fn status(&self) -> TaskStatus {
        match self {
            Self::Pending { .. } => TaskStatus::Pending,
            Self::Throttled { .. } => TaskStatus::Throttled,
            Self::Cancelled { .. } => TaskStatus::Cancelled,
            Self::Running { .. } => TaskStatus::Running,
            Self::Succeeded { .. } => TaskStatus::Succeeded,
            Self::Failed { .. } => TaskStatus::Failed,
        }
    }

    /// Returns the creation timestamp string.
    pub fn created_at(&self) -> &str {
        match self {
            Self::Pending { created_at, .. }
            | Self::Throttled { created_at, .. }
            | Self::Cancelled { created_at, .. }
            | Self::Running { created_at, .. }
            | Self::Failed { created_at, .. }
            | Self::Succeeded { created_at, .. } => created_at,
        }
    }

    /// Returns the task progress when the server provides it.
    pub fn progress(&self) -> Option<f64> {
        match self {
            Self::Running { progress, .. } => *progress,
            _ => None,
        }
    }

    /// Returns the output URLs if the task succeeded and has output.
    pub fn output_urls(&self) -> Option<&[String]> {
        match self {
            Self::Succeeded { output, .. } => Some(output.as_slice()),
            _ => None,
        }
    }

    /// Returns the failure message when the task failed.
    pub fn failure(&self) -> Option<&str> {
        match self {
            Self::Failed { failure, .. } => Some(failure.as_str()),
            _ => None,
        }
    }

    /// Returns the failure code when the task failed.
    pub fn failure_code(&self) -> Option<&str> {
        match self {
            Self::Failed { failure_code, .. } => failure_code.as_deref(),
            _ => None,
        }
    }

    /// Returns `true` if the task has reached a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded { .. } | Self::Failed { .. } | Self::Cancelled { .. }
        )
    }

    /// Returns `true` if the task succeeded.
    pub fn is_succeeded(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns `true` if the task failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Returns `true` if the task was cancelled or deleted.
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskCreateResponse {
    pub id: Uuid,
}

#[cfg(feature = "unstable-endpoints")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaskList {
    pub tasks: Vec<Task>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[cfg(feature = "unstable-endpoints")]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[cfg(feature = "unstable-endpoints")]
impl TaskListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}
