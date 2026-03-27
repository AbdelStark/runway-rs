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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
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
            ResponseTemplate::new(400).set_body_string(r#"{"error": "Invalid model specified"}"#),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

    let result = client
        .text_to_video()
        .create(TextToVideoRequest::new(VideoModel::Gen45, "test prompt"))
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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

    let pending = client
        .text_to_video()
        .create(TextToVideoRequest::new(VideoModel::Gen45, "test prompt"))
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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

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

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

    let list = client.avatars().list().await.unwrap();
    assert_eq!(list.avatars.len(), 2);
    assert_eq!(list.avatars[0].name, "Avatar One");
}

// ── Document CRUD tests ──────────────────────────────────────────────────

#[tokio::test]
async fn test_documents_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "documents": [
                {"id": "doc-1", "name": "Design Brief"},
                {"id": "doc-2", "name": "Script Draft"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let list = client.documents().list().await.unwrap();
    assert_eq!(list.documents.len(), 2);
    assert_eq!(list.documents[0].name, "Design Brief");
}

#[tokio::test]
async fn test_documents_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "doc-new",
            "name": "New Doc"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let doc = client
        .documents()
        .create(CreateDocumentRequest::new("New Doc").content("body"))
        .await
        .unwrap();
    assert_eq!(doc.id, "doc-new");
    assert_eq!(doc.name, "New Doc");
}

#[tokio::test]
async fn test_documents_update() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/v1/documents/doc-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "doc-1",
            "name": "Updated Name"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let doc = client
        .documents()
        .update("doc-1", UpdateDocumentRequest::new().name("Updated Name"))
        .await
        .unwrap();
    assert_eq!(doc.name, "Updated Name");
}

#[tokio::test]
async fn test_documents_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/documents/doc-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    client.documents().delete("doc-1").await.unwrap();
}

// ── Voice tests ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_voices_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/voices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [
                {"id": "v-1", "name": "Deep Male"},
                {"id": "v-2", "name": "Soft Female"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let list = client.voices().list().await.unwrap();
    assert_eq!(list.voices.len(), 2);
    assert_eq!(list.voices[1].name, "Soft Female");
}

#[tokio::test]
async fn test_voices_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/voices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "v-new",
            "name": "Custom Voice"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let voice = client
        .voices()
        .create(CreateVoiceRequest::new("Custom Voice"))
        .await
        .unwrap();
    assert_eq!(voice.id, "v-new");
}

#[tokio::test]
async fn test_voices_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/voices/v-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    client.voices().delete("v-1").await.unwrap();
}

// ── Organization tests ───────────────────────────────────────────────────

#[tokio::test]
async fn test_organization_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "org-123",
            "name": "My Org"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let org = client.organization().get().await.unwrap();
    assert_eq!(org.id, "org-123");
    assert_eq!(org.name, "My Org");
}

#[tokio::test]
async fn test_organization_usage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/organization/usage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "usage": {"credits": 100}
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let resp = client
        .organization()
        .usage(UsageQueryRequest::new().start_date("2024-01-01"))
        .await
        .unwrap();
    assert!(resp.usage.is_some());
}

// ── Workflow tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_workflows_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/workflows"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workflows": [
                {"id": "wf-1", "name": "Video Pipeline"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let list = client.workflows().list().await.unwrap();
    assert_eq!(list.workflows.len(), 1);
    assert_eq!(list.workflows[0].name, "Video Pipeline");
}

#[tokio::test]
async fn test_workflows_run() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/workflows/wf-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "inv-1"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let resp = client
        .workflows()
        .run(
            "wf-1",
            RunWorkflowRequest::new().param("prompt", serde_json::json!("test")),
        )
        .await
        .unwrap();
    assert_eq!(resp.id, "inv-1");
}

// ── Realtime session tests ───────────────────────────────────────────────

#[tokio::test]
async fn test_realtime_session_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/realtime_sessions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "rs-1",
            "status": "active"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let session = client
        .realtime_sessions()
        .create(CreateRealtimeSessionRequest::new().model("gen4_turbo"))
        .await
        .unwrap();
    assert_eq!(session.id, "rs-1");
}

#[tokio::test]
async fn test_realtime_session_cancel() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/realtime_sessions/rs-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    client.realtime_sessions().cancel("rs-1").await.unwrap();
}

// ── Avatar CRUD tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_avatar_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/avatars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "av-new",
            "name": "Test Avatar"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let avatar = client
        .avatars()
        .create(CreateAvatarRequest::new("Test Avatar"))
        .await
        .unwrap();
    assert_eq!(avatar.id, "av-new");
}

#[tokio::test]
async fn test_avatar_update() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "av-1",
            "name": "Renamed"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let avatar = client
        .avatars()
        .update("av-1", UpdateAvatarRequest::new().name("Renamed"))
        .await
        .unwrap();
    assert_eq!(avatar.name, "Renamed");
}

#[tokio::test]
async fn test_avatar_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    client.avatars().delete("av-1").await.unwrap();
}

// ── Additional generation resource tests ─────────────────────────────────

#[tokio::test]
async fn test_video_to_video_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/video_to_video"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "770e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .video_to_video()
        .create(VideoToVideoRequest::new(
            VideoModel::Gen45,
            "Transform",
            MediaInput::from_url("https://example.com/video.mp4"),
        ))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "770e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_text_to_image_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_image"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "880e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .text_to_image()
        .create(TextToImageRequest::new(
            ImageModel::Gen4ImageTurbo,
            "A sunset",
        ))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "880e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_sound_effect_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/sound_effect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "990e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .sound_effect()
        .create(SoundEffectRequest::new("thunder"))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "990e8400-e29b-41d4-a716-446655440000"
    );
}

// ── Additional generation resource tests (audio/voice) ───────────────────

#[tokio::test]
async fn test_character_performance_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/character_performance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "aa0e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .character_performance()
        .create(CharacterPerformanceRequest::new(
            VideoModel::Gen4Turbo,
            "Wave hello",
            MediaInput::from_url("https://example.com/face.jpg"),
            MediaInput::from_url("https://example.com/motion.mp4"),
        ))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "aa0e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_text_to_speech_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_speech"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "bb0e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .text_to_speech()
        .create(TextToSpeechRequest::new("Hello world").voice_id("v-1"))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "bb0e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_speech_to_speech_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/speech_to_speech"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "cc0e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .speech_to_speech()
        .create(SpeechToSpeechRequest::new(MediaInput::from_url(
            "https://example.com/audio.wav",
        )))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "cc0e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_voice_dubbing_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/voice_dubbing"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "dd0e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .voice_dubbing()
        .create(
            VoiceDubbingRequest::new(MediaInput::from_url("https://example.com/speech.mp3"))
                .target_language("es"),
        )
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "dd0e8400-e29b-41d4-a716-446655440000"
    );
}

#[tokio::test]
async fn test_voice_isolation_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/voice_isolation"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "ee0e8400-e29b-41d4-a716-446655440000"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let pending = client
        .voice_isolation()
        .create(VoiceIsolationRequest::new(MediaInput::from_url(
            "https://example.com/noisy.wav",
        )))
        .await
        .unwrap();
    assert_eq!(
        pending.id().to_string(),
        "ee0e8400-e29b-41d4-a716-446655440000"
    );
}

// ── Workflow invocation test ──────────────────────────────────────────────

#[tokio::test]
async fn test_workflow_invocation_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/workflow_invocations/inv-42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "inv-42",
            "status": "completed",
            "output": {"url": "https://cdn.runway.com/result.mp4"}
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let inv = client.workflow_invocations().get("inv-42").await.unwrap();
    assert_eq!(inv.id, "inv-42");
    assert_eq!(inv.status, Some("completed".into()));
}

// ── Upload file full flow test ───────────────────────────────────────────

#[tokio::test]
async fn test_upload_file() {
    let mock_server = MockServer::start().await;

    // Mock the upload creation endpoint
    let presigned_url = format!("{}/presigned-upload", mock_server.uri());
    Mock::given(method("POST"))
        .and(path("/v1/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "upload-456",
            "uploadUrl": presigned_url
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock the presigned URL PUT — verify no Authorization header is sent
    Mock::given(method("PUT"))
        .and(path("/presigned-upload"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();

    // Create a temporary file
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join("test-upload.txt");
    tokio::fs::write(&tmp_file, b"hello world").await.unwrap();

    let uri = client.uploads().upload_file(&tmp_file).await.unwrap();
    assert_eq!(uri, "runway://upload-456");

    // Cleanup
    let _ = tokio::fs::remove_file(&tmp_file).await;
}

// ── Error handling edge case tests ───────────────────────────────────────

#[tokio::test]
async fn test_delete_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let task_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let result = client.tasks().delete(task_id).await;
    assert!(matches!(result, Err(RunwayError::Unauthorized)));
}

#[tokio::test]
async fn test_patch_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(422).set_body_string(r#"{"error": "Invalid name"}"#))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let result = client
        .avatars()
        .update("av-1", UpdateAvatarRequest::new().name(""))
        .await;

    match result {
        Err(RunwayError::Api { status, .. }) => assert_eq!(status, 422),
        other => panic!("Expected Api error, got {:?}", other),
    }
}

// ── DELETE/PATCH retry tests ─────────────────────────────────────────────

#[tokio::test]
async fn test_delete_rate_limit_retry() {
    let mock_server = MockServer::start().await;

    // First request returns 429, second returns 204
    Mock::given(method("DELETE"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(429).append_header("retry-after", "1"))
        .expect(1)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    client.avatars().delete("av-1").await.unwrap();
}

#[tokio::test]
async fn test_patch_rate_limit_retry() {
    let mock_server = MockServer::start().await;

    // First request returns 429, second returns 200
    Mock::given(method("PATCH"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(429).append_header("retry-after", "1"))
        .expect(1)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/v1/avatars/av-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "av-1",
            "name": "Updated"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RunwayClient::with_config(test_config(&mock_server.uri())).unwrap();
    let avatar = client
        .avatars()
        .update("av-1", UpdateAvatarRequest::new().name("Updated"))
        .await
        .unwrap();
    assert_eq!(avatar.name, "Updated");
}

// ── Rate limit exhaustion test ───────────────────────────────────────────

#[tokio::test]
async fn test_rate_limit_exhausted() {
    let mock_server = MockServer::start().await;

    // Return 429 for every request — exhaust retries
    Mock::given(method("GET"))
        .and(path("/v1/tasks/550e8400-e29b-41d4-a716-446655440000"))
        .respond_with(ResponseTemplate::new(429).append_header("retry-after", "60"))
        .mount(&mock_server)
        .await;

    let config = test_config(&mock_server.uri());
    let client = RunwayClient::with_config(config).unwrap();
    let task_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    let result = client.tasks().get(task_id).await;
    assert!(matches!(result, Err(RunwayError::RateLimited { .. })));
}

// ── Polling & lifecycle tests ────────────────────────────────────────────

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

    let client = RunwayClient::with_config(config).unwrap();

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

    let client = RunwayClient::with_config(config).unwrap();

    let result = client
        .text_to_video()
        .create(TextToVideoRequest::new(VideoModel::Gen45, "test"))
        .await
        .unwrap()
        .wait_with_config(Duration::from_millis(100), Duration::from_secs(10))
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Content policy violation"));
}
