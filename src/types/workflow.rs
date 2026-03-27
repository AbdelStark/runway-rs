use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub id: String,
    pub created_at: String,
    #[serde(default)]
    pub description: Option<String>,
    pub graph: WorkflowGraph,
    pub name: String,
    pub updated_at: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowGraph {
    pub edges: Vec<serde_json::Value>,
    pub nodes: Vec<serde_json::Value>,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowList {
    pub data: Vec<WorkflowListGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowListGroup {
    pub name: String,
    pub versions: Vec<WorkflowListVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowListVersion {
    pub id: String,
    pub created_at: String,
    pub version: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunWorkflowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_outputs: Option<HashMap<String, HashMap<String, WorkflowNodeOutputValue>>>,
}

impl RunWorkflowRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_output(
        mut self,
        node_id: impl Into<String>,
        output_key: impl Into<String>,
        value: WorkflowNodeOutputValue,
    ) -> Self {
        self.node_outputs
            .get_or_insert_with(HashMap::new)
            .entry(node_id.into())
            .or_default()
            .insert(output_key.into(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowNodeOutputValue {
    Primitive { value: PrimitiveNodeValue },
    Image { uri: String },
    Video { uri: String },
    Audio { uri: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PrimitiveNodeValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl From<&str> for PrimitiveNodeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for PrimitiveNodeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for PrimitiveNodeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for PrimitiveNodeValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i64> for PrimitiveNodeValue {
    fn from(value: i64) -> Self {
        Self::Number(value as f64)
    }
}

impl From<u64> for PrimitiveNodeValue {
    fn from(value: u64) -> Self {
        Self::Number(value as f64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRunResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowInvocationStatus {
    Pending,
    Throttled,
    Cancelled,
    Running,
    Failed,
    Succeeded,
}

impl std::fmt::Display for WorkflowInvocationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Throttled => write!(f, "THROTTLED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Running => write!(f, "RUNNING"),
            Self::Failed => write!(f, "FAILED"),
            Self::Succeeded => write!(f, "SUCCEEDED"),
        }
    }
}

pub type WorkflowInvocationOutput = HashMap<String, Vec<String>>;
pub type WorkflowInvocationNodeErrors = HashMap<String, WorkflowNodeError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNodeError {
    pub message: String,
    #[serde(default)]
    pub node_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowInvocation {
    Pending {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Throttled {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Cancelled {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
    Running {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        output: WorkflowInvocationOutput,
        progress: f64,
        #[serde(rename = "nodeErrors", default)]
        node_errors: Option<WorkflowInvocationNodeErrors>,
    },
    Failed {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        failure: String,
        #[serde(rename = "failureCode", default)]
        failure_code: Option<String>,
        #[serde(rename = "nodeErrors", default)]
        node_errors: Option<WorkflowInvocationNodeErrors>,
    },
    Succeeded {
        id: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        output: WorkflowInvocationOutput,
        #[serde(rename = "nodeErrors", default)]
        node_errors: Option<WorkflowInvocationNodeErrors>,
    },
}

impl WorkflowInvocation {
    pub fn id(&self) -> &str {
        match self {
            Self::Pending { id, .. }
            | Self::Throttled { id, .. }
            | Self::Cancelled { id, .. }
            | Self::Running { id, .. }
            | Self::Failed { id, .. }
            | Self::Succeeded { id, .. } => id.as_str(),
        }
    }

    pub fn status(&self) -> WorkflowInvocationStatus {
        match self {
            Self::Pending { .. } => WorkflowInvocationStatus::Pending,
            Self::Throttled { .. } => WorkflowInvocationStatus::Throttled,
            Self::Cancelled { .. } => WorkflowInvocationStatus::Cancelled,
            Self::Running { .. } => WorkflowInvocationStatus::Running,
            Self::Failed { .. } => WorkflowInvocationStatus::Failed,
            Self::Succeeded { .. } => WorkflowInvocationStatus::Succeeded,
        }
    }

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

    pub fn output(&self) -> Option<&WorkflowInvocationOutput> {
        match self {
            Self::Running { output, .. } | Self::Succeeded { output, .. } => Some(output),
            _ => None,
        }
    }

    pub fn progress(&self) -> Option<f64> {
        match self {
            Self::Running { progress, .. } => Some(*progress),
            _ => None,
        }
    }

    pub fn failure(&self) -> Option<&str> {
        match self {
            Self::Failed { failure, .. } => Some(failure.as_str()),
            _ => None,
        }
    }

    pub fn failure_code(&self) -> Option<&str> {
        match self {
            Self::Failed { failure_code, .. } => failure_code.as_deref(),
            _ => None,
        }
    }

    pub fn node_errors(&self) -> Option<&WorkflowInvocationNodeErrors> {
        match self {
            Self::Running { node_errors, .. }
            | Self::Failed { node_errors, .. }
            | Self::Succeeded { node_errors, .. } => node_errors.as_ref(),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded { .. } | Self::Failed { .. } | Self::Cancelled { .. }
        )
    }

    pub fn is_succeeded(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }
}
