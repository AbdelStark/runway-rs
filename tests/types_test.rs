use runway_sdk::*;

#[test]
fn test_video_model_serialization() {
    let model = VideoModel::Gen45;
    let json = serde_json::to_string(&model).unwrap();
    assert_eq!(json, r#""gen4.5""#);

    let model = VideoModel::Gen4Turbo;
    let json = serde_json::to_string(&model).unwrap();
    assert_eq!(json, r#""gen4_turbo""#);

    let model = VideoModel::Veo31Fast;
    let json = serde_json::to_string(&model).unwrap();
    assert_eq!(json, r#""veo3.1_fast""#);
}

#[test]
fn test_video_model_deserialization() {
    let model: VideoModel = serde_json::from_str(r#""gen4.5""#).unwrap();
    assert_eq!(model, VideoModel::Gen45);

    let model: VideoModel = serde_json::from_str(r#""gen3a_turbo""#).unwrap();
    assert_eq!(model, VideoModel::Gen3aTurbo);
}

#[test]
fn test_image_model_serialization() {
    let model = ImageModel::Gen4ImageTurbo;
    let json = serde_json::to_string(&model).unwrap();
    assert_eq!(json, r#""gen4_image_turbo""#);

    let model = ImageModel::Gemini25Flash;
    let json = serde_json::to_string(&model).unwrap();
    assert_eq!(json, r#""gemini_2.5_flash""#);
}

#[test]
fn test_video_ratio_serialization() {
    let ratio = VideoRatio::Landscape;
    let json = serde_json::to_string(&ratio).unwrap();
    assert_eq!(json, r#""1280:720""#);

    let ratio = VideoRatio::Square;
    let json = serde_json::to_string(&ratio).unwrap();
    assert_eq!(json, r#""960:960""#);
}

#[test]
fn test_task_status_serialization() {
    let status = TaskStatus::Succeeded;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""SUCCEEDED""#);

    let status = TaskStatus::Pending;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""PENDING""#);
}

#[test]
fn test_task_status_deserialization() {
    let status: TaskStatus = serde_json::from_str(r#""RUNNING""#).unwrap();
    assert_eq!(status, TaskStatus::Running);

    let status: TaskStatus = serde_json::from_str(r#""FAILED""#).unwrap();
    assert_eq!(status, TaskStatus::Failed);
}

#[test]
fn test_task_deserialization() {
    let json = r#"{
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "status": "SUCCEEDED",
        "createdAt": "2024-01-01T00:00:00Z",
        "output": ["https://example.com/video.mp4"],
        "progress": 1.0
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.status, TaskStatus::Succeeded);
    assert_eq!(task.output.unwrap(), vec!["https://example.com/video.mp4"]);
    assert_eq!(task.progress, Some(1.0));
    assert!(task.failure.is_none());
}

#[test]
fn test_task_failed_deserialization() {
    let json = r#"{
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "status": "FAILED",
        "createdAt": "2024-01-01T00:00:00Z",
        "failure": "Content moderation triggered",
        "failureCode": "CONTENT_MODERATION"
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.status, TaskStatus::Failed);
    assert_eq!(task.failure, Some("Content moderation triggered".into()));
    assert_eq!(task.failure_code, Some("CONTENT_MODERATION".into()));
}

#[test]
fn test_text_to_video_request_serialization() {
    let req = TextToVideoRequest::new(VideoModel::Gen45, "A cat on a skateboard")
        .ratio(VideoRatio::Landscape)
        .duration(5)
        .seed(42);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4.5");
    assert_eq!(json["promptText"], "A cat on a skateboard");
    assert_eq!(json["ratio"], "1280:720");
    assert_eq!(json["duration"], 5);
    assert_eq!(json["seed"], 42);
}

#[test]
fn test_image_to_video_request_serialization() {
    let req = ImageToVideoRequest::new(
        VideoModel::Gen4Turbo,
        "Zoom in slowly",
        MediaInput::from_url("https://example.com/image.jpg"),
    )
    .duration(10);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_turbo");
    assert_eq!(json["promptText"], "Zoom in slowly");
    assert_eq!(json["promptImage"], "https://example.com/image.jpg");
    assert_eq!(json["duration"], 10);
    assert!(json.get("ratio").is_none());
}

#[test]
fn test_media_input_from_url() {
    let input = MediaInput::from_url("https://example.com/image.jpg");
    let json = serde_json::to_string(&input).unwrap();
    assert_eq!(json, r#""https://example.com/image.jpg""#);
}

#[test]
fn test_media_input_from_base64() {
    let input = MediaInput::from_base64("image/png", &[1, 2, 3]);
    let json = serde_json::to_string(&input).unwrap();
    assert!(json.contains("data:image/png;base64,"));
}

#[test]
fn test_media_input_from_string() {
    let input: MediaInput = "https://example.com/image.jpg".into();
    let json = serde_json::to_string(&input).unwrap();
    assert_eq!(json, r#""https://example.com/image.jpg""#);
}

#[test]
fn test_sound_effect_request_serialization() {
    let req = SoundEffectRequest::new("thunder rumbling in the distance").duration(10);
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["promptText"], "thunder rumbling in the distance");
    assert_eq!(json["duration"], 10);
}

#[test]
fn test_text_to_image_request_serialization() {
    let req = TextToImageRequest::new(ImageModel::Gen4ImageTurbo, "A cyberpunk city")
        .ratio(VideoRatio::Wide)
        .seed(123);
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_image_turbo");
    assert_eq!(json["promptText"], "A cyberpunk city");
    assert_eq!(json["ratio"], "1104:832");
    assert_eq!(json["seed"], 123);
}

#[test]
fn test_create_avatar_request() {
    let req = CreateAvatarRequest::new("Test Avatar")
        .description("A test")
        .init_image("https://example.com/img.jpg");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "Test Avatar");
    assert_eq!(json["description"], "A test");
    assert_eq!(json["initImage"], "https://example.com/img.jpg");
}

#[test]
fn test_run_workflow_request() {
    let req = RunWorkflowRequest::new()
        .param("prompt", serde_json::json!("hello"))
        .param("steps", serde_json::json!(20));
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["params"]["prompt"], "hello");
    assert_eq!(json["params"]["steps"], 20);
}

#[test]
fn test_content_moderation_serialization() {
    let cm = ContentModeration::Automatic;
    let json = serde_json::to_string(&cm).unwrap();
    assert_eq!(json, r#""automatic""#);
}

#[test]
fn test_client_with_api_key() {
    let client = RunwayClient::with_api_key("test-key-123").unwrap();
    assert_eq!(client.inner.config.api_key, "test-key-123");
    assert_eq!(client.inner.config.base_url, "https://api.dev.runwayml.com");
}

#[test]
fn test_client_missing_api_key() {
    let original = std::env::var("RUNWAYML_API_SECRET").ok();
    std::env::remove_var("RUNWAYML_API_SECRET");

    let result = RunwayClient::new();
    assert!(result.is_err());

    if let Some(val) = original {
        std::env::set_var("RUNWAYML_API_SECRET", val);
    }
}

#[test]
fn test_client_with_invalid_api_key() {
    // API key with newline should fail validation, not panic
    let result = RunwayClient::with_api_key("bad\nkey");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("invalid header characters"));
}

#[test]
fn test_video_to_video_request_serialization() {
    let req = VideoToVideoRequest::new(
        VideoModel::Gen45,
        "Transform the scene",
        MediaInput::from_url("https://example.com/video.mp4"),
    )
    .ratio(VideoRatio::Portrait)
    .duration(8);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4.5");
    assert_eq!(json["promptText"], "Transform the scene");
    assert_eq!(json["promptVideo"], "https://example.com/video.mp4");
    assert_eq!(json["ratio"], "720:1280");
    assert_eq!(json["duration"], 8);
}

#[test]
fn test_character_performance_request_serialization() {
    let req = CharacterPerformanceRequest::new(
        VideoModel::Gen4Turbo,
        "Act surprised",
        MediaInput::from_url("https://example.com/face.jpg"),
        MediaInput::from_url("https://example.com/motion.mp4"),
    )
    .duration(5)
    .seed(99);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_turbo");
    assert_eq!(json["promptText"], "Act surprised");
    assert_eq!(json["promptImage"], "https://example.com/face.jpg");
    assert_eq!(json["promptVideo"], "https://example.com/motion.mp4");
    assert_eq!(json["duration"], 5);
    assert_eq!(json["seed"], 99);
}

#[test]
fn test_speech_to_speech_request_serialization() {
    let req = SpeechToSpeechRequest::new(MediaInput::from_url("https://example.com/audio.wav"))
        .voice_id("voice-123")
        .seed(42);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["audio"], "https://example.com/audio.wav");
    assert_eq!(json["voiceId"], "voice-123");
    assert_eq!(json["seed"], 42);
}

#[test]
fn test_text_to_speech_request_serialization() {
    let req = TextToSpeechRequest::new("Hello, world!")
        .voice_id("voice-456")
        .seed(7);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["promptText"], "Hello, world!");
    assert_eq!(json["voiceId"], "voice-456");
    assert_eq!(json["seed"], 7);
}

#[test]
fn test_voice_dubbing_request_serialization() {
    let req = VoiceDubbingRequest::new(MediaInput::from_url("https://example.com/audio.mp3"))
        .target_language("es")
        .seed(10);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["audio"], "https://example.com/audio.mp3");
    assert_eq!(json["targetLanguage"], "es");
    assert_eq!(json["seed"], 10);
}

#[test]
fn test_voice_isolation_request_serialization() {
    let req =
        VoiceIsolationRequest::new(MediaInput::from_url("https://example.com/noisy.wav")).seed(55);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["audio"], "https://example.com/noisy.wav");
    assert_eq!(json["seed"], 55);
}

#[test]
fn test_create_document_request_serialization() {
    let req = CreateDocumentRequest::new("My Document")
        .content("Document body text")
        .description("A test document");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "My Document");
    assert_eq!(json["content"], "Document body text");
    assert_eq!(json["description"], "A test document");
}

#[test]
fn test_update_document_request_serialization() {
    let req = UpdateDocumentRequest::new()
        .name("Updated Name")
        .content("New content");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "Updated Name");
    assert_eq!(json["content"], "New content");
    assert!(json.get("description").is_none());
}

#[test]
fn test_create_voice_request_serialization() {
    let req = CreateVoiceRequest::new("My Voice")
        .audio("https://example.com/sample.wav")
        .description("A custom voice");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "My Voice");
    assert_eq!(json["audio"], "https://example.com/sample.wav");
    assert_eq!(json["description"], "A custom voice");
}

#[test]
fn test_preview_voice_request_serialization() {
    let req = PreviewVoiceRequest::new("Test speech text").voice_id("voice-789");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["text"], "Test speech text");
    assert_eq!(json["voiceId"], "voice-789");
}

#[test]
fn test_usage_query_request_serialization() {
    let req = UsageQueryRequest::new()
        .start_date("2024-01-01")
        .end_date("2024-12-31");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["startDate"], "2024-01-01");
    assert_eq!(json["endDate"], "2024-12-31");
}

#[test]
fn test_create_realtime_session_request_serialization() {
    let req = CreateRealtimeSessionRequest::new()
        .model("gen4_turbo")
        .params(serde_json::json!({"key": "value"}));

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_turbo");
    assert_eq!(json["params"]["key"], "value");
}

#[test]
fn test_update_avatar_request_serialization() {
    let req = UpdateAvatarRequest::new()
        .name("New Name")
        .description("Updated description");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "New Name");
    assert_eq!(json["description"], "Updated description");
}

// ── Lip Sync ────────────────────────────────────────────────────────────

#[test]
fn test_lip_sync_request_serialization() {
    let req = LipSyncRequest::new(
        VideoModel::Gen45,
        MediaInput::from_url("https://example.com/video.mp4"),
        MediaInput::from_url("https://example.com/audio.wav"),
    )
    .max_duration(30)
    .seed(42);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4.5");
    assert_eq!(json["promptVideo"], "https://example.com/video.mp4");
    assert_eq!(json["promptAudio"], "https://example.com/audio.wav");
    assert_eq!(json["maxDuration"], 30);
    assert_eq!(json["seed"], 42);
    assert!(json.get("contentModeration").is_none());
}

#[test]
fn test_lip_sync_request_minimal() {
    let req = LipSyncRequest::new(
        VideoModel::Gen4Turbo,
        MediaInput::from_url("https://example.com/v.mp4"),
        MediaInput::from_url("https://example.com/a.mp3"),
    );

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_turbo");
    assert!(json.get("maxDuration").is_none());
    assert!(json.get("seed").is_none());
}

// ── Image Upscale ───────────────────────────────────────────────────────

#[test]
fn test_image_upscale_request_serialization() {
    let req = ImageUpscaleRequest::new(
        ImageModel::Gen4ImageTurbo,
        MediaInput::from_url("https://example.com/img.jpg"),
    )
    .resolution(4096)
    .seed(7);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_image_turbo");
    assert_eq!(json["promptImage"], "https://example.com/img.jpg");
    assert_eq!(json["resolution"], 4096);
    assert_eq!(json["seed"], 7);
}

#[test]
fn test_image_upscale_request_minimal() {
    let req = ImageUpscaleRequest::new(
        ImageModel::Gen4Image,
        MediaInput::from_url("https://example.com/low_res.png"),
    );

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_image");
    assert!(json.get("resolution").is_none());
    assert!(json.get("seed").is_none());
}

// ── Task List Query ─────────────────────────────────────────────────────

#[test]
fn test_task_list_query_serialization() {
    let query = TaskListQuery::new()
        .status(TaskStatus::Running)
        .limit(10)
        .offset(5);

    let json = serde_json::to_value(&query).unwrap();
    assert_eq!(json["status"], "RUNNING");
    assert_eq!(json["limit"], 10);
    assert_eq!(json["offset"], 5);
}

#[test]
fn test_task_list_query_empty() {
    let query = TaskListQuery::new();
    let json = serde_json::to_value(&query).unwrap();
    assert!(json.get("status").is_none());
    assert!(json.get("limit").is_none());
    assert!(json.get("offset").is_none());
}

#[test]
fn test_task_list_deserialization() {
    let json = r#"{
        "tasks": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "status": "SUCCEEDED",
                "createdAt": "2024-01-01T00:00:00Z",
                "output": ["https://example.com/video.mp4"]
            },
            {
                "id": "660e8400-e29b-41d4-a716-446655440000",
                "status": "RUNNING",
                "createdAt": "2024-01-02T00:00:00Z",
                "progress": 0.5
            }
        ],
        "hasMore": true
    }"#;

    let list: TaskList = serde_json::from_str(json).unwrap();
    assert_eq!(list.tasks.len(), 2);
    assert_eq!(list.tasks[0].status, TaskStatus::Succeeded);
    assert_eq!(list.tasks[1].status, TaskStatus::Running);
    assert_eq!(list.has_more, Some(true));
}

// ── Send + Sync assertions ───────────────────────────────────────────────

fn _assert_send<T: Send>() {}
fn _assert_sync<T: Sync>() {}

#[test]
fn test_client_is_send_sync() {
    _assert_send::<RunwayClient>();
    _assert_sync::<RunwayClient>();
}

#[test]
fn test_pending_task_is_send() {
    _assert_send::<PendingTask>();
}

#[test]
fn test_error_is_send_sync() {
    _assert_send::<RunwayError>();
    _assert_sync::<RunwayError>();
}

// ── Config Debug redacts API key ─────────────────────────────────────────

#[test]
fn test_config_debug_redacts_api_key() {
    let config = Config::new("super-secret-api-key-12345");
    let debug_output = format!("{:?}", config);
    assert!(
        !debug_output.contains("super-secret"),
        "Config Debug output should not contain the API key"
    );
    assert!(
        debug_output.contains("REDACTED"),
        "Config Debug output should show [REDACTED]"
    );
}

#[test]
fn test_config_builder() {
    use runway_sdk::Config;
    use std::time::Duration;

    let config = Config::new("key")
        .base_url("https://custom.api.com")
        .api_version("2025-01-01")
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .poll_interval(Duration::from_secs(10))
        .max_poll_duration(Duration::from_secs(300));

    assert_eq!(config.api_key, "key");
    assert_eq!(config.base_url, "https://custom.api.com");
    assert_eq!(config.api_version, "2025-01-01");
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.poll_interval, Duration::from_secs(10));
    assert_eq!(config.max_poll_duration, Duration::from_secs(300));
}

// ── Webhook URL serialization tests ─────────────────────────────────────

#[test]
fn test_text_to_video_webhook_url() {
    let req = TextToVideoRequest::new(VideoModel::Gen45, "test prompt")
        .webhook_url("https://example.com/webhook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/webhook");
}

#[test]
fn test_text_to_video_no_webhook_url() {
    let req = TextToVideoRequest::new(VideoModel::Gen45, "test prompt");
    let json = serde_json::to_value(&req).unwrap();
    assert!(json.get("webhookUrl").is_none());
}

#[test]
fn test_image_to_video_webhook_url() {
    let req = ImageToVideoRequest::new(
        VideoModel::Gen45,
        "test",
        MediaInput::from_url("https://example.com/img.png"),
    )
    .webhook_url("https://hooks.example.com/callback");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://hooks.example.com/callback");
}

#[test]
fn test_sound_effect_webhook_url() {
    let req = SoundEffectRequest::new("thunder").webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_lip_sync_webhook_url() {
    let req = LipSyncRequest::new(
        VideoModel::Gen45,
        MediaInput::from_url("https://example.com/video.mp4"),
        MediaInput::from_url("https://example.com/audio.mp3"),
    )
    .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_image_upscale_webhook_url() {
    let req = ImageUpscaleRequest::new(
        ImageModel::Gen4ImageTurbo,
        MediaInput::from_url("https://example.com/img.png"),
    )
    .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_voice_dubbing_webhook_url() {
    let req = VoiceDubbingRequest::new(MediaInput::from_url("https://example.com/audio.mp3"))
        .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_voice_isolation_webhook_url() {
    let req = VoiceIsolationRequest::new(MediaInput::from_url("https://example.com/audio.mp3"))
        .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_text_to_speech_webhook_url() {
    let req = TextToSpeechRequest::new("hello world").webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_speech_to_speech_webhook_url() {
    let req = SpeechToSpeechRequest::new(MediaInput::from_url("https://example.com/audio.mp3"))
        .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_character_performance_webhook_url() {
    let req = CharacterPerformanceRequest::new(
        VideoModel::Gen45,
        "test",
        MediaInput::from_url("https://example.com/img.png"),
        MediaInput::from_url("https://example.com/video.mp4"),
    )
    .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_video_to_video_webhook_url() {
    let req = VideoToVideoRequest::new(
        VideoModel::Gen45,
        "test",
        MediaInput::from_url("https://example.com/video.mp4"),
    )
    .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

#[test]
fn test_text_to_image_webhook_url() {
    let req = TextToImageRequest::new(ImageModel::Gen4ImageTurbo, "test")
        .webhook_url("https://example.com/hook");
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["webhookUrl"], "https://example.com/hook");
}

// ── Response type Serialize roundtrip tests ─────────────────────────────

#[test]
fn test_avatar_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "av_123",
        "name": "Test Avatar",
        "description": "A test avatar",
        "createdAt": "2024-01-01T00:00:00Z"
    });
    let avatar: runway_sdk::Avatar = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&avatar).unwrap();
    assert_eq!(serialized["id"], "av_123");
    assert_eq!(serialized["name"], "Test Avatar");
}

#[test]
fn test_voice_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "voice_123",
        "name": "Test Voice",
        "description": "A test voice"
    });
    let voice: runway_sdk::Voice = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&voice).unwrap();
    assert_eq!(serialized["id"], "voice_123");
}

#[test]
fn test_organization_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "org_123",
        "name": "Test Org",
        "createdAt": "2024-01-01T00:00:00Z"
    });
    let org: runway_sdk::Organization = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&org).unwrap();
    assert_eq!(serialized["id"], "org_123");
}

#[test]
fn test_workflow_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "wf_123",
        "name": "Test Workflow",
        "description": "A test workflow"
    });
    let wf: runway_sdk::Workflow = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&wf).unwrap();
    assert_eq!(serialized["id"], "wf_123");
}

// ── Display impl tests ──────────────────────────────────────────────────

#[test]
fn test_video_model_display() {
    assert_eq!(VideoModel::Gen45.to_string(), "gen4.5");
    assert_eq!(VideoModel::Gen4Turbo.to_string(), "gen4_turbo");
    assert_eq!(VideoModel::Gen3aTurbo.to_string(), "gen3a_turbo");
    assert_eq!(VideoModel::Veo31.to_string(), "veo3.1");
    assert_eq!(VideoModel::Veo31Fast.to_string(), "veo3.1_fast");
    assert_eq!(VideoModel::Veo3.to_string(), "veo3");
}

#[test]
fn test_image_model_display() {
    assert_eq!(ImageModel::Gen4ImageTurbo.to_string(), "gen4_image_turbo");
    assert_eq!(ImageModel::Gen4Image.to_string(), "gen4_image");
    assert_eq!(ImageModel::Gemini25Flash.to_string(), "gemini_2.5_flash");
}

#[test]
fn test_video_ratio_display() {
    assert_eq!(VideoRatio::Landscape.to_string(), "1280:720");
    assert_eq!(VideoRatio::Portrait.to_string(), "720:1280");
    assert_eq!(VideoRatio::Wide.to_string(), "1104:832");
    assert_eq!(VideoRatio::Square.to_string(), "960:960");
    assert_eq!(VideoRatio::Tall.to_string(), "832:1104");
    assert_eq!(VideoRatio::Ultrawide.to_string(), "1584:672");
}

// ── Task convenience method tests ───────────────────────────────────────

#[test]
fn test_task_is_terminal() {
    let succeeded = Task {
        id: uuid::Uuid::new_v4(),
        status: TaskStatus::Succeeded,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: Some(vec!["https://example.com/video.mp4".to_string()]),
        failure: None,
        failure_code: None,
        progress: Some(1.0),
    };
    assert!(succeeded.is_terminal());
    assert!(succeeded.is_succeeded());
    assert!(!succeeded.is_failed());

    let failed = Task {
        id: uuid::Uuid::new_v4(),
        status: TaskStatus::Failed,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: None,
        failure: Some("Error".to_string()),
        failure_code: Some("ERR".to_string()),
        progress: None,
    };
    assert!(failed.is_terminal());
    assert!(!failed.is_succeeded());
    assert!(failed.is_failed());

    let running = Task {
        id: uuid::Uuid::new_v4(),
        status: TaskStatus::Running,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: None,
        failure: None,
        failure_code: None,
        progress: Some(0.5),
    };
    assert!(!running.is_terminal());
    assert!(!running.is_succeeded());
    assert!(!running.is_failed());
}

#[test]
fn test_task_output_urls() {
    let task_with_output = Task {
        id: uuid::Uuid::new_v4(),
        status: TaskStatus::Succeeded,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: Some(vec![
            "https://cdn.runway.com/video1.mp4".to_string(),
            "https://cdn.runway.com/video2.mp4".to_string(),
        ]),
        failure: None,
        failure_code: None,
        progress: Some(1.0),
    };
    let urls = task_with_output.output_urls().unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://cdn.runway.com/video1.mp4");

    let task_no_output = Task {
        id: uuid::Uuid::new_v4(),
        status: TaskStatus::Running,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: None,
        failure: None,
        failure_code: None,
        progress: Some(0.3),
    };
    assert!(task_no_output.output_urls().is_none());
}

// ── PartialEq tests ────────────────────────────────────────────────────

#[test]
fn test_model_enums_eq_and_hash() {
    use std::collections::HashSet;

    let mut models = HashSet::new();
    models.insert(VideoModel::Gen45);
    models.insert(VideoModel::Gen4Turbo);
    models.insert(VideoModel::Gen45); // duplicate
    assert_eq!(models.len(), 2);

    let mut ratios = HashSet::new();
    ratios.insert(VideoRatio::Landscape);
    ratios.insert(VideoRatio::Portrait);
    assert_eq!(ratios.len(), 2);
}

#[test]
fn test_task_status_display() {
    assert_eq!(TaskStatus::Pending.to_string(), "PENDING");
    assert_eq!(TaskStatus::Throttled.to_string(), "THROTTLED");
    assert_eq!(TaskStatus::Running.to_string(), "RUNNING");
    assert_eq!(TaskStatus::Succeeded.to_string(), "SUCCEEDED");
    assert_eq!(TaskStatus::Failed.to_string(), "FAILED");
}

#[test]
fn test_generation_request_partial_eq() {
    let req1 = TextToVideoRequest::new(VideoModel::Gen45, "A sunset")
        .duration(10)
        .seed(42);
    let req2 = TextToVideoRequest::new(VideoModel::Gen45, "A sunset")
        .duration(10)
        .seed(42);
    assert_eq!(req1, req2);

    let req3 = TextToVideoRequest::new(VideoModel::Gen45, "A sunrise");
    assert_ne!(req1, req3);
}

#[test]
fn test_task_partial_eq() {
    let id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let task1 = Task {
        id,
        status: TaskStatus::Succeeded,
        created_at: "2024-01-01".to_string(),
        output: Some(vec!["url".to_string()]),
        failure: None,
        failure_code: None,
        progress: Some(1.0),
    };
    let task2 = task1.clone();
    assert_eq!(task1, task2);
}
