use serde::{Deserialize, Serialize};

use crate::types::common::{CursorPage, CursorPageQuery};

pub type DocumentList = CursorPage<DocumentListItem>;
pub type DocumentListQuery = CursorPageQuery;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    Text,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUsedBy {
    pub id: String,
    #[serde(default)]
    pub image_url: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub content: String,
    pub created_at: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub updated_at: String,
    pub used_by: Vec<DocumentUsedBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentListItem {
    pub id: String,
    pub created_at: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub updated_at: String,
    pub used_by: Vec<DocumentUsedBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentRequest {
    pub content: String,
    pub name: String,
}

impl CreateDocumentRequest {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateDocumentRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
}
