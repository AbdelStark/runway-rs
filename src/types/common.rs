use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPage<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

impl<T> CursorPage<T> {
    pub fn items(&self) -> &[T] {
        &self.data
    }

    pub fn into_items(self) -> Vec<T> {
        self.data
    }

    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn has_next_page(&self) -> bool {
        self.next_cursor
            .as_deref()
            .is_some_and(|cursor| !cursor.is_empty())
    }
}

impl<T> IntoIterator for CursorPage<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a CursorPage<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl CursorPageQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContentModeration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_figure_threshold: Option<PublicFigureThreshold>,
}

impl Default for ContentModeration {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentModeration {
    pub fn new() -> Self {
        Self {
            public_figure_threshold: None,
        }
    }

    pub fn public_figure_threshold(mut self, threshold: PublicFigureThreshold) -> Self {
        self.public_figure_threshold = Some(threshold);
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PublicFigureThreshold {
    Auto,
    Low,
}
