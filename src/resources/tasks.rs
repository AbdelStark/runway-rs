#[cfg(feature = "unstable-endpoints")]
use async_stream::try_stream;
#[cfg(feature = "unstable-endpoints")]
use futures_core::Stream;
use uuid::Uuid;

use crate::client::{RequestOptions, RunwayClient, WithResponse};
use crate::error::RunwayError;
use crate::types::task::Task;
#[cfg(feature = "unstable-endpoints")]
use crate::types::task::{TaskList, TaskListQuery};

pub struct TasksResource {
    pub(crate) client: RunwayClient,
}

/// Query parameters serialized for the GET /v1/tasks endpoint.
///
/// Uses a separate struct so that `Option<TaskStatus>` serializes to
/// the correct SCREAMING_SNAKE_CASE string via serde, and `None` fields
/// are omitted entirely from the query string.
#[cfg(feature = "unstable-endpoints")]
#[derive(serde::Serialize)]
struct TaskListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<crate::types::task::TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

#[cfg(feature = "unstable-endpoints")]
impl From<&TaskListQuery> for TaskListParams {
    fn from(q: &TaskListQuery) -> Self {
        Self {
            status: q.status,
            limit: q.limit,
            offset: q.offset,
        }
    }
}

impl TasksResource {
    /// Retrieve a task by id.
    pub async fn retrieve(&self, id: Uuid) -> Result<Task, RunwayError> {
        Ok(self
            .retrieve_with_options(id, RequestOptions::default())
            .await?
            .data)
    }

    pub async fn retrieve_with_options(
        &self,
        id: Uuid,
        options: RequestOptions,
    ) -> Result<WithResponse<Task>, RunwayError> {
        self.client
            .get_with_options(&format!("/v1/tasks/{}", id), &options)
            .await
    }

    #[cfg(feature = "unstable-endpoints")]
    /// List tasks, optionally filtered by status with pagination.
    pub async fn list(&self, query: TaskListQuery) -> Result<TaskList, RunwayError> {
        Ok(self
            .list_with_options(query, RequestOptions::default())
            .await?
            .data)
    }

    #[cfg(feature = "unstable-endpoints")]
    pub async fn list_with_options(
        &self,
        query: TaskListQuery,
        options: RequestOptions,
    ) -> Result<WithResponse<TaskList>, RunwayError> {
        let params = TaskListParams::from(&query);
        self.client
            .get_with_query_with_options("/v1/tasks", &params, &options)
            .await
    }

    #[cfg(feature = "unstable-endpoints")]
    /// Stream pages of tasks, automatically fetching subsequent pages
    /// while `has_more` is `true`.
    ///
    /// Each item yielded is a full [`TaskList`] page. The caller can
    /// flatten pages into individual tasks as needed.
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_core::Stream;
    /// use runway_sdk::{RunwayClient, TaskListQuery};
    ///
    /// let client = RunwayClient::new()?;
    /// let stream = client.tasks().list_stream(TaskListQuery::new().limit(10));
    /// // Use with futures::StreamExt::next() or similar stream combinators.
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_stream(
        self,
        query: TaskListQuery,
    ) -> impl Stream<Item = Result<TaskList, RunwayError>> {
        let client = self.client;
        let page_size = query.limit.unwrap_or(100);
        let status_filter = query.status;
        let initial_offset = query.offset.unwrap_or(0);

        try_stream! {
            let mut offset = initial_offset;
            loop {
                let params = TaskListParams {
                    status: status_filter,
                    limit: Some(page_size),
                    offset: Some(offset),
                };

                let page: TaskList = client.get_with_query("/v1/tasks", &params).await?;
                let has_more = page.has_more.unwrap_or(false);
                let count = page.tasks.len() as u32;
                yield page;

                if !has_more || count == 0 {
                    break;
                }
                offset += count;
            }
        }
    }

    #[cfg(feature = "unstable-endpoints")]
    /// Stream individual tasks across all pages, automatically paginating.
    ///
    /// This is a convenience wrapper around [`list_stream`](Self::list_stream)
    /// that flattens pages into individual [`Task`] items.
    pub fn list_all(self, query: TaskListQuery) -> impl Stream<Item = Result<Task, RunwayError>> {
        let client = self.client;
        let page_size = query.limit.unwrap_or(100);
        let status_filter = query.status;
        let initial_offset = query.offset.unwrap_or(0);

        try_stream! {
            let mut offset = initial_offset;
            loop {
                let params = TaskListParams {
                    status: status_filter,
                    limit: Some(page_size),
                    offset: Some(offset),
                };

                let page: TaskList = client.get_with_query("/v1/tasks", &params).await?;
                let has_more = page.has_more.unwrap_or(false);
                let count = page.tasks.len() as u32;
                for task in page.tasks {
                    yield task;
                }

                if !has_more || count == 0 {
                    break;
                }
                offset += count;
            }
        }
    }

    pub async fn get(&self, id: Uuid) -> Result<Task, RunwayError> {
        self.retrieve(id).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RunwayError> {
        self.delete_with_options(id, RequestOptions::default())
            .await?;
        Ok(())
    }

    pub async fn delete_with_options(
        &self,
        id: Uuid,
        options: RequestOptions,
    ) -> Result<WithResponse<()>, RunwayError> {
        self.client
            .delete_with_options(&format!("/v1/tasks/{}", id), &options)
            .await
    }

    #[cfg(feature = "unstable-endpoints")]
    /// Cancel a running task. Unlike delete, cancel stops an in-progress task
    /// without removing it from the task list.
    pub async fn cancel(&self, id: Uuid) -> Result<(), RunwayError> {
        self.cancel_with_options(id, RequestOptions::default())
            .await?;
        Ok(())
    }

    #[cfg(feature = "unstable-endpoints")]
    pub async fn cancel_with_options(
        &self,
        id: Uuid,
        options: RequestOptions,
    ) -> Result<WithResponse<serde_json::Value>, RunwayError> {
        self.client
            .post_with_options(
                &format!("/v1/tasks/{}/cancel", id),
                &serde_json::json!({}),
                &options,
            )
            .await
    }
}
