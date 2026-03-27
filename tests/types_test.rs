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
        "output": ["https://example.com/video.mp4"]
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.status(), TaskStatus::Succeeded);
    assert_eq!(
        task.output_urls().unwrap(),
        ["https://example.com/video.mp4"]
    );
    assert_eq!(task.progress(), None);
    assert!(task.failure().is_none());
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
    assert_eq!(task.status(), TaskStatus::Failed);
    assert_eq!(task.failure(), Some("Content moderation triggered"));
    assert_eq!(task.failure_code(), Some("CONTENT_MODERATION"));
}

#[test]
fn test_text_to_video_request_serialization() {
    let req =
        TextToVideoGen45Request::new("A cat on a skateboard", VideoRatio::Landscape, 5).seed(42);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4.5");
    assert_eq!(json["promptText"], "A cat on a skateboard");
    assert_eq!(json["ratio"], "1280:720");
    assert_eq!(json["duration"], 5);
    assert_eq!(json["seed"], 42);
}

#[test]
fn test_image_to_video_request_serialization() {
    let req =
        ImageToVideoGen4TurboRequest::new("https://example.com/image.jpg", VideoRatio::Landscape)
            .prompt_text("Zoom in slowly")
            .duration(10);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_turbo");
    assert_eq!(json["promptText"], "Zoom in slowly");
    assert_eq!(json["promptImage"], "https://example.com/image.jpg");
    assert_eq!(json["duration"], 10);
    assert_eq!(json["ratio"], "1280:720");
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
    let req = SoundEffectRequest::new("thunder rumbling in the distance").duration(10.0);
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "eleven_text_to_sound_v2");
    assert_eq!(json["promptText"], "thunder rumbling in the distance");
    assert_eq!(json["duration"], 10.0);
}

#[test]
fn test_text_to_image_request_serialization() {
    let req = TextToImageGen4ImageTurboRequest::new(
        "A cyberpunk city",
        ImageRatio::Square1024,
        vec![TextToImageReferenceImage::new(
            "https://example.com/reference.png",
        )],
    )
    .seed(123);
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_image_turbo");
    assert_eq!(json["promptText"], "A cyberpunk city");
    assert_eq!(json["ratio"], "1024:1024");
    assert_eq!(
        json["referenceImages"][0]["uri"],
        "https://example.com/reference.png"
    );
    assert_eq!(json["seed"], 123);
}

#[test]
fn test_create_avatar_request() {
    let req = CreateAvatarRequest::new(
        "Test Avatar",
        "Friendly and concise",
        "https://example.com/img.jpg",
        AvatarVoiceInput::runway_live_preset("adrian"),
    )
    .document_ids(vec!["doc-1".to_string()]);
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "Test Avatar");
    assert_eq!(json["personality"], "Friendly and concise");
    assert_eq!(json["referenceImage"], "https://example.com/img.jpg");
    assert_eq!(json["voice"]["type"], "runway-live-preset");
    assert_eq!(json["voice"]["presetId"], "adrian");
    assert_eq!(json["documentIds"][0], "doc-1");
}

#[test]
fn test_run_workflow_request() {
    let req = RunWorkflowRequest::new()
        .node_output(
            "node-a",
            "prompt",
            WorkflowNodeOutputValue::Primitive {
                value: PrimitiveNodeValue::from("hello"),
            },
        )
        .node_output(
            "node-a",
            "steps",
            WorkflowNodeOutputValue::Primitive {
                value: PrimitiveNodeValue::from(20_u64),
            },
        );
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["nodeOutputs"]["node-a"]["prompt"]["type"], "primitive");
    assert_eq!(json["nodeOutputs"]["node-a"]["prompt"]["value"], "hello");
    assert_eq!(json["nodeOutputs"]["node-a"]["steps"]["value"], 20.0);
}

#[test]
fn test_content_moderation_serialization() {
    let cm = ContentModeration::new().public_figure_threshold(PublicFigureThreshold::Low);
    let json = serde_json::to_value(&cm).unwrap();
    assert_eq!(json["publicFigureThreshold"], "low");
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
    let req = VideoToVideoRequest::new("Transform the scene", "https://example.com/video.mp4")
        .ratio(VideoRatio::Portrait)
        .references(vec![VideoToVideoReference::image(
            "https://example.com/reference.png",
        )]);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gen4_aleph");
    assert_eq!(json["promptText"], "Transform the scene");
    assert_eq!(json["videoUri"], "https://example.com/video.mp4");
    assert_eq!(json["ratio"], "720:1280");
    assert_eq!(json["references"][0]["type"], "image");
}

#[test]
fn test_character_performance_request_serialization() {
    let req = CharacterPerformanceRequest::new(
        CharacterPerformanceCharacter::image("https://example.com/face.jpg"),
        CharacterPerformanceReference::video("https://example.com/motion.mp4"),
    )
    .expression_intensity(5)
    .seed(99);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "act_two");
    assert_eq!(json["character"]["type"], "image");
    assert_eq!(json["character"]["uri"], "https://example.com/face.jpg");
    assert_eq!(json["reference"]["type"], "video");
    assert_eq!(json["reference"]["uri"], "https://example.com/motion.mp4");
    assert_eq!(json["expressionIntensity"], 5);
    assert_eq!(json["seed"], 99);
}

#[test]
fn test_speech_to_speech_request_serialization() {
    let req = SpeechToSpeechRequest::new(
        SpeechToSpeechMedia::audio("https://example.com/audio.wav"),
        RunwayPresetVoice::new(RunwayPresetVoiceId::Maya),
    )
    .remove_background_noise(true);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["media"]["type"], "audio");
    assert_eq!(json["media"]["uri"], "https://example.com/audio.wav");
    assert_eq!(json["voice"]["type"], "runway-preset");
    assert_eq!(json["voice"]["presetId"], "Maya");
    assert_eq!(json["removeBackgroundNoise"], true);
}

#[test]
fn test_text_to_speech_request_serialization() {
    let req = TextToSpeechRequest::new(
        "Hello, world!",
        RunwayPresetVoice::new(RunwayPresetVoiceId::Maya),
    );

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "eleven_multilingual_v2");
    assert_eq!(json["promptText"], "Hello, world!");
    assert_eq!(json["voice"]["type"], "runway-preset");
    assert_eq!(json["voice"]["presetId"], "Maya");
}

#[test]
fn test_voice_dubbing_request_serialization() {
    let req = VoiceDubbingRequest::new("https://example.com/audio.mp3", VoiceDubbingLanguage::Es)
        .num_speakers(2)
        .drop_background_audio(true);

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["audioUri"], "https://example.com/audio.mp3");
    assert_eq!(json["targetLang"], "es");
    assert_eq!(json["numSpeakers"], 2);
    assert_eq!(json["dropBackgroundAudio"], true);
}

#[test]
fn test_voice_isolation_request_serialization() {
    let req = VoiceIsolationRequest::new("https://example.com/noisy.wav");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["audioUri"], "https://example.com/noisy.wav");
    assert_eq!(json["model"], "eleven_voice_isolation");
}

#[test]
fn test_create_document_request_serialization() {
    let req = CreateDocumentRequest::new("My Document", "Document body text");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "My Document");
    assert_eq!(json["content"], "Document body text");
}

#[test]
fn test_update_document_request_serialization() {
    let req = UpdateDocumentRequest::new()
        .name("Updated Name")
        .content("New content");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "Updated Name");
    assert_eq!(json["content"], "New content");
}

#[test]
fn test_create_voice_request_serialization() {
    let req = CreateVoiceRequest::new("My Voice", "Warm, cinematic narrator with confident pacing")
        .description("A custom voice");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "My Voice");
    assert_eq!(json["from"]["type"], "text");
    assert_eq!(json["from"]["model"], "eleven_multilingual_ttv_v2");
    assert_eq!(
        json["from"]["prompt"],
        "Warm, cinematic narrator with confident pacing"
    );
    assert_eq!(json["description"], "A custom voice");
}

#[test]
fn test_preview_voice_request_serialization() {
    let req = PreviewVoiceRequest::new("Warm and expressive voice for product demos");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(
        json["prompt"],
        "Warm and expressive voice for product demos"
    );
    assert_eq!(json["model"], "eleven_multilingual_ttv_v2");
}

#[test]
fn test_usage_query_request_serialization() {
    let req = UsageQueryRequest::new()
        .start_date("2024-01-01")
        .end_date("2024-12-31");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["startDate"], "2024-01-01");
    assert_eq!(json["beforeDate"], "2024-12-31");
}

#[test]
fn test_create_realtime_session_request_serialization() {
    let req = CreateRealtimeSessionRequest::new(RealtimeAvatarInput::custom("avatar-123"))
        .max_duration(900)
        .personality("Helpful and upbeat")
        .start_script("Welcome to the session");

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["model"], "gwm1_avatars");
    assert_eq!(json["avatar"]["type"], "custom");
    assert_eq!(json["avatar"]["avatarId"], "avatar-123");
    assert_eq!(json["maxDuration"], 900);
    assert_eq!(json["personality"], "Helpful and upbeat");
    assert_eq!(json["startScript"], "Welcome to the session");
}

#[test]
fn test_update_avatar_request_serialization() {
    let req = UpdateAvatarRequest::new()
        .name("New Name")
        .personality("Updated personality")
        .reference_image("https://example.com/new.jpg")
        .voice(AvatarVoiceInput::custom("voice-123"))
        .clear_start_script();

    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "New Name");
    assert_eq!(json["personality"], "Updated personality");
    assert_eq!(json["referenceImage"], "https://example.com/new.jpg");
    assert_eq!(json["voice"]["type"], "custom");
    assert_eq!(json["voice"]["id"], "voice-123");
    assert!(json["startScript"].is_null());
}

#[cfg(feature = "unstable-endpoints")]
// ── Lip Sync ────────────────────────────────────────────────────────────
#[cfg(feature = "unstable-endpoints")]
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

#[cfg(feature = "unstable-endpoints")]
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

#[cfg(feature = "unstable-endpoints")]
// ── Image Upscale ───────────────────────────────────────────────────────
#[cfg(feature = "unstable-endpoints")]
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

#[cfg(feature = "unstable-endpoints")]
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

#[cfg(feature = "unstable-endpoints")]
// ── Task List Query ─────────────────────────────────────────────────────
#[cfg(feature = "unstable-endpoints")]
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

#[cfg(feature = "unstable-endpoints")]
#[test]
fn test_task_list_query_empty() {
    let query = TaskListQuery::new();
    let json = serde_json::to_value(&query).unwrap();
    assert!(json.get("status").is_none());
    assert!(json.get("limit").is_none());
    assert!(json.get("offset").is_none());
}

#[cfg(feature = "unstable-endpoints")]
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
    assert_eq!(list.tasks[0].status(), TaskStatus::Succeeded);
    assert_eq!(list.tasks[1].status(), TaskStatus::Running);
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

// ── Stable generation validation tests ─────────────────────────────────

#[test]
fn test_stable_generation_requests_do_not_serialize_webhook_url() {
    let req = TextToVideoGen45Request::new("test prompt", VideoRatio::Landscape, 5);
    let json = serde_json::to_value(&req).unwrap();
    assert!(json.get("webhookUrl").is_none());
}

#[test]
fn test_text_to_video_gen45_validation_rejects_invalid_ratio() {
    let req = TextToVideoGen45Request::new("test prompt", VideoRatio::Wide, 5);
    let err = req.validate().unwrap_err();
    assert!(err.to_string().contains("ratio"));
}

#[test]
fn test_image_to_video_veo31_validation_rejects_last_frame_only() {
    let req = ImageToVideoVeo31Request::new(
        PromptImageInput::Frames(vec![PromptFrame::last("https://example.com/last.png")]),
        VideoRatio::Landscape,
    );
    let err = req.validate().unwrap_err();
    assert!(err.to_string().contains("first prompt frame"));
}

#[test]
fn test_sound_effect_validation_rejects_out_of_range_duration() {
    let req = SoundEffectRequest::new("thunder").duration(31.0);
    let err = req.validate().unwrap_err();
    assert!(err.to_string().contains("duration"));
}

#[test]
fn test_voice_dubbing_validation_rejects_zero_speakers() {
    let req = VoiceDubbingRequest::new("https://example.com/audio.mp3", VoiceDubbingLanguage::Es)
        .num_speakers(0);
    let err = req.validate().unwrap_err();
    assert!(err.to_string().contains("numSpeakers"));
}

// ── Response type Serialize roundtrip tests ─────────────────────────────

#[test]
fn test_avatar_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "av_123",
        "status": "READY",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-02T00:00:00Z",
        "documentIds": [],
        "name": "Test Avatar",
        "personality": "Friendly",
        "processedImageUri": "https://example.com/processed.png",
        "referenceImageUri": "https://example.com/reference.png",
        "startScript": null,
        "voice": {
            "type": "runway-live-preset",
            "presetId": "maya",
            "name": "Maya",
            "description": "Warm preset"
        }
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
        "status": "READY",
        "createdAt": "2024-01-01T00:00:00Z",
        "name": "Test Voice",
        "description": "A test voice",
        "previewUrl": "https://example.com/preview.mp3"
    });
    let voice: runway_sdk::Voice = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&voice).unwrap();
    assert_eq!(serialized["id"], "voice_123");
}

#[test]
fn test_organization_serialize_roundtrip() {
    let json = serde_json::json!({
        "creditBalance": 42.5,
        "tier": {
            "maxMonthlyCreditSpend": 1000.0,
            "models": {
                "gen4.5": {
                    "maxConcurrentGenerations": 2,
                    "maxDailyGenerations": 100
                }
            }
        },
        "usage": {
            "models": {
                "gen4.5": {
                    "dailyGenerations": 12
                }
            }
        }
    });
    let org: runway_sdk::Organization = serde_json::from_value(json).unwrap();
    let serialized = serde_json::to_value(&org).unwrap();
    assert_eq!(serialized["creditBalance"], 42.5);
}

#[test]
fn test_workflow_serialize_roundtrip() {
    let json = serde_json::json!({
        "id": "wf_123",
        "name": "Test Workflow",
        "description": "A test workflow",
        "createdAt": "2024-01-01T00:00:00Z",
        "updatedAt": "2024-01-02T00:00:00Z",
        "version": 3,
        "graph": {
            "edges": [],
            "nodes": [],
            "version": 1
        }
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
    let succeeded = Task::Succeeded {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: vec!["https://example.com/video.mp4".to_string()],
    };
    assert!(succeeded.is_terminal());
    assert!(succeeded.is_succeeded());
    assert!(!succeeded.is_failed());
    assert!(!succeeded.is_cancelled());

    let failed = Task::Failed {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        failure: "Error".to_string(),
        failure_code: Some("ERR".to_string()),
    };
    assert!(failed.is_terminal());
    assert!(!failed.is_succeeded());
    assert!(failed.is_failed());
    assert!(!failed.is_cancelled());

    let cancelled = Task::Cancelled {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    };
    assert!(cancelled.is_terminal());
    assert!(!cancelled.is_succeeded());
    assert!(!cancelled.is_failed());
    assert!(cancelled.is_cancelled());

    let running = Task::Running {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        progress: Some(0.5),
    };
    assert!(!running.is_terminal());
    assert!(!running.is_succeeded());
    assert!(!running.is_failed());
    assert!(!running.is_cancelled());
}

#[test]
fn test_task_output_urls() {
    let task_with_output = Task::Succeeded {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        output: vec![
            "https://cdn.runway.com/video1.mp4".to_string(),
            "https://cdn.runway.com/video2.mp4".to_string(),
        ],
    };
    let urls = task_with_output.output_urls().unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://cdn.runway.com/video1.mp4");

    let task_no_output = Task::Running {
        id: uuid::Uuid::new_v4(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
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
    assert_eq!(TaskStatus::Cancelled.to_string(), "CANCELLED");
    assert_eq!(TaskStatus::Running.to_string(), "RUNNING");
    assert_eq!(TaskStatus::Succeeded.to_string(), "SUCCEEDED");
    assert_eq!(TaskStatus::Failed.to_string(), "FAILED");
}

#[test]
fn test_generation_request_partial_eq() {
    let req1 = TextToVideoGen45Request::new("A sunset", VideoRatio::Landscape, 10).seed(42);
    let req2 = TextToVideoGen45Request::new("A sunset", VideoRatio::Landscape, 10).seed(42);
    assert_eq!(req1, req2);

    let req3 = TextToVideoGen45Request::new("A sunrise", VideoRatio::Landscape, 10);
    assert_ne!(req1, req3);
}

#[test]
fn test_task_partial_eq() {
    let id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let task1 = Task::Succeeded {
        id,
        created_at: "2024-01-01".to_string(),
        output: vec!["url".to_string()],
    };
    let task2 = task1.clone();
    assert_eq!(task1, task2);
}
