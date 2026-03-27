use runway_sdk::*;
use std::time::Duration;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(base_url: &str) -> Config {
    Config::new("test-api-key-12345")
        .base_url(base_url)
        .poll_interval(Duration::from_millis(100))
        .max_poll_duration(Duration::from_secs(10))
}

#[tokio::test]
async fn test_text_to_video_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .and(header("Authorization", "Bearer test-api-key-12345"))
        .and(header("X-Runway-Version", "2024-11-06"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let pending = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "A cat on a skateboard",
        ))
        .await
        .unwrap();

    assert_eq!(
        pending.id().to_string(),
        "550e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_image_to_video_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/image_to_video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "660e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let pending = client
        .image_to_video()
        .create(ImageToVideoRequest::new(
            VideoModel::Gen4Turbo,
            "Zoom in",
            MediaInput::from_url("https://example.com/img.jpg"),
        ))
        .await
        .unwrap();

    assert_eq!(
        pending.id().to_string(),
        "660e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_get_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "status": "RUNNING",
            "createdAt": "2024-01-01T00:00:00Z",
            "progress": 0.5
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));
    let task_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let task = client.tasks().get(task_id).await.unwrap();
    assert_eq!(task.status, TaskStatus::Running);
    assert_eq!(task.progress, Some(0.5));
}

#[tokio::test]
async fn test_delete_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));
    let task_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    client.tasks().delete(task_id).await.unwrap();
}

#[tokio::test]
async fn test_unauthorized_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));
    let task_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let result = client.tasks().get(task_id).await;
    assert!(matches!(result, Err(RunwayError::Unauthorized)));
}

#[tokio::test]
async fn test_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_string(r#"{"error": "Invalid model specified"}"#),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let result = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "test prompt",
        ))
        .await;

    match result {
        Err(RunwayError::Api { status, .. }) => assert_eq!(status, 400),
        other => panic!("Expected Api error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_rate_limit_retry() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(ResponseTemplate::new(429).append_header("retry-after", "1"))
        .expect(1)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let pending = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "test prompt",
        ))
        .await
        .unwrap();

    assert_eq!(
        pending.id().to_string(),
        "550e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_upload_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "upload-123",
            "uploadUrl": "https://presigned.example.com/upload"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let resp = client.uploads().create("test.jpg").await.unwrap();
    assert_eq!(resp.id, "upload-123");
    assert_eq!(resp.upload_url, "https://presigned.example.com/upload");
}

#[tokio::test]
async fn test_avatars_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/avatars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "avatars": [
                {"id": "av-1", "name": "Avatar One"},
                {"id": "av-2", "name": "Avatar Two"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri()));

    let list = client.avatars().list().await.unwrap();
    assert_eq!(list.avatars.len(), 2);
    assert_eq!(list.avatars[0].name, "Avatar One");
}

#[tokio::test]
async fn test_full_polling_sequence() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "status": "SUCCEEDED",
            "createdAt": "2024-01-01T00:00:00Z",
            "output": ["https://cdn.runway.com/video.mp4"],
            "progress": 1.0
        })))
        .expect(1..)
        .mount(&mock_server)
        .await;

    let config = Config::new("test-key")
        .base_url(mock_server.uri())
        .poll_interval(Duration::from_millis(100))
        .max_poll_duration(Duration::from_secs(10));

    let client = RunwayClient::with_config(config);

    let task = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "A beautiful sunset",
        ))
        .await
        .unwrap()
        .wait_with_config(Duration::from_millis(100), Duration::from_secs(10))
        .await
        .unwrap();

    assert_eq!(task.status, TaskStatus::Succeeded);
    assert_eq!(
        task.output.unwrap(),
        vec!["https://cdn.runway.com/video.mp4"]
    );
}

#[tokio::test]
async fn test_polling_task_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "status": "FAILED",
            "createdAt": "2024-01-01T00:00:00Z",
            "failure": "Content policy violation",
            "failureCode": "CONTENT_MODERATION"
        })))
        .mount(&mock_server)
        .await;

    let config = Config::new("test-key")
        .base_url(mock_server.uri())
        .poll_interval(Duration::from_millis(100));

    let client = RunwayClient::with_config(config);

    let result = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "test",
        ))
        .await
        .unwrap()
        .wait_with_config(Duration::from_millis(100), Duration::from_secs(10))
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Content policy violation"));
}
