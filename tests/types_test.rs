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
