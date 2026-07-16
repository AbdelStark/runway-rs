use runway_sdk::{
    Aleph2EditRange, Aleph2Keyframe, Aleph2TargetAspectRatio, Config, ContentModeration,
    GeminiOmniFlashImageReference, HappyhorseResolution, ImageToVideoGeminiOmniFlashRequest,
    ImageToVideoGen45Request, ImageToVideoGen4TurboRequest, ImageToVideoHappyhorse10Request,
    ImageToVideoSeedance2FastRequest, ImageToVideoSeedance2MiniRequest,
    ImageToVideoSeedance2Request, ImageToVideoVeo31FastRequest, ImageToVideoVeo31Request,
    ImageToVideoVeo3Request, PromptFrame, PromptImageInput, RequestOptions, RunwayClient,
    Seedance2AudioReference, Seedance2ImageReference, Seedance2PromptImage,
    Seedance2PromptImageInput, Seedance2VideoReference, TextToVideoGeminiOmniFlashRequest,
    TextToVideoGen45Request, TextToVideoHappyhorse10Request, TextToVideoSeedance2FastRequest,
    TextToVideoSeedance2MiniRequest, TextToVideoSeedance2Request, TextToVideoVeo31FastRequest,
    TextToVideoVeo31Request, TextToVideoVeo3Request, VideoRatio, VideoToVideoAleph2Request,
    VideoToVideoGeminiOmniFlashRequest, VideoToVideoSeedance2FastRequest,
    VideoToVideoSeedance2MiniRequest, VideoToVideoSeedance2Request,
};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn text_to_video_happyhorse_serializes_official_wire_shape() {
    let request = TextToVideoHappyhorse10Request::new("A horse running through snow")
        .duration(7)
        .ratio(VideoRatio::R1108x832);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "model": "happyhorse_1_0",
            "promptText": "A horse running through snow",
            "duration": 7,
            "ratio": "1108:832"
        })
    );
}

#[test]
fn video_to_video_aleph2_serializes_timed_keyframes() {
    let request = VideoToVideoAleph2Request::new("https://example.com/input.mp4")
        .content_moderation(ContentModeration::new())
        .keyframes(vec![Aleph2Keyframe::seconds(
            "https://example.com/guide.png",
            2.0,
        )
        .range(Aleph2EditRange::new(1, 5))])
        .prompt_text("Replace the sky")
        .seed(42)
        .target_aspect_ratio(Aleph2TargetAspectRatio::Wide16x9);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "model": "aleph2",
            "videoUri": "https://example.com/input.mp4",
            "contentModeration": {},
            "keyframes": [{
                "seconds": 2.0,
                "uri": "https://example.com/guide.png",
                "range": {"start_seconds": 1, "end_seconds": 5}
            }],
            "promptText": "Replace the sky",
            "seed": 42,
            "targetAspectRatio": "16:9"
        })
    );
}

#[test]
fn video_to_video_seedance_and_gemini_variants_match_official_shapes() {
    let seedance = VideoToVideoSeedance2Request::new("https://example.com/input.mp4")
        .audio(true)
        .duration(7.5)
        .prompt_text("Follow the references")
        .ratio(VideoRatio::R2206x946)
        .reference_audio(vec![Seedance2AudioReference::new(
            "https://example.com/audio.mp3",
        )])
        .references(vec![Seedance2ImageReference::new(
            "https://example.com/image.png",
        )])
        .reference_videos(vec![Seedance2VideoReference::new(
            "https://example.com/reference.mp4",
        )]);
    let fast = VideoToVideoSeedance2FastRequest::new("https://example.com/fast.mp4")
        .ratio(VideoRatio::R1112x834);
    let mini = VideoToVideoSeedance2MiniRequest::new("https://example.com/mini.mp4").audio(false);
    let gemini = VideoToVideoGeminiOmniFlashRequest::new(
        "Turn daytime into night",
        "https://example.com/gemini.mp4",
    )
    .references(vec![GeminiOmniFlashImageReference::new(
        "https://example.com/moon.png",
    )]);

    assert_eq!(
        serde_json::to_value(seedance).unwrap(),
        serde_json::json!({
            "model": "seedance2",
            "promptVideo": "https://example.com/input.mp4",
            "audio": true,
            "duration": 7.5,
            "promptText": "Follow the references",
            "ratio": "2206:946",
            "referenceAudio": [{"type": "audio", "uri": "https://example.com/audio.mp3"}],
            "references": [{"uri": "https://example.com/image.png"}],
            "referenceVideos": [{"type": "video", "uri": "https://example.com/reference.mp4"}]
        })
    );
    assert_eq!(
        serde_json::to_value(fast).unwrap(),
        serde_json::json!({
            "model": "seedance2_fast",
            "promptVideo": "https://example.com/fast.mp4",
            "ratio": "1112:834"
        })
    );
    assert_eq!(
        serde_json::to_value(mini).unwrap(),
        serde_json::json!({
            "model": "seedance2_mini",
            "promptVideo": "https://example.com/mini.mp4",
            "audio": false
        })
    );
    assert_eq!(
        serde_json::to_value(gemini).unwrap(),
        serde_json::json!({
            "model": "gemini_omni_flash",
            "promptText": "Turn daytime into night",
            "videoUri": "https://example.com/gemini.mp4",
            "references": [{"uri": "https://example.com/moon.png"}]
        })
    );
}

#[test]
fn image_to_video_new_model_family_uses_official_wire_shapes() {
    let happyhorse = ImageToVideoHappyhorse10Request::new("https://example.com/horse.png")
        .duration(6)
        .prompt_text("Gallop forward")
        .resolution(HappyhorseResolution::P1080);
    let fast = ImageToVideoSeedance2FastRequest::new(vec![Seedance2PromptImage::reference(
        "https://example.com/style.png",
    )])
    .prompt_text("Use this style")
    .ratio(VideoRatio::R1470x630);
    let mini = ImageToVideoSeedance2MiniRequest::new("https://example.com/start.png");
    let gemini = ImageToVideoGeminiOmniFlashRequest::new("https://example.com/gemini.png")
        .duration(10)
        .prompt_text("Slowly zoom in")
        .ratio(VideoRatio::Landscape);

    assert_eq!(
        serde_json::to_value(happyhorse).unwrap(),
        serde_json::json!({
            "model": "happyhorse_1_0",
            "promptImage": "https://example.com/horse.png",
            "duration": 6,
            "promptText": "Gallop forward",
            "resolution": "1080P"
        })
    );
    assert_eq!(
        serde_json::to_value(fast).unwrap(),
        serde_json::json!({
            "model": "seedance2_fast",
            "promptImage": [{"uri": "https://example.com/style.png"}],
            "promptText": "Use this style",
            "ratio": "1470:630"
        })
    );
    assert_eq!(
        serde_json::to_value(mini).unwrap(),
        serde_json::json!({
            "model": "seedance2_mini",
            "promptImage": "https://example.com/start.png"
        })
    );
    assert_eq!(
        serde_json::to_value(gemini).unwrap(),
        serde_json::json!({
            "model": "gemini_omni_flash",
            "promptImage": "https://example.com/gemini.png",
            "duration": 10,
            "promptText": "Slowly zoom in",
            "ratio": "1280:720"
        })
    );
}

#[test]
fn image_to_video_seedance2_serializes_keyframes_and_audio_context() {
    let prompt_image = Seedance2PromptImageInput::images(vec![
        Seedance2PromptImage::first("https://example.com/first.png"),
        Seedance2PromptImage::last("https://example.com/last.png"),
    ]);
    let request = ImageToVideoSeedance2Request::new(prompt_image)
        .audio(true)
        .duration(8)
        .prompt_text("Move between the keyframes")
        .ratio(VideoRatio::R3840x1646)
        .reference_audio(vec![Seedance2AudioReference::new(
            "https://example.com/score.mp3",
        )]);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "model": "seedance2",
            "promptImage": [
                {"uri": "https://example.com/first.png", "position": "first"},
                {"uri": "https://example.com/last.png", "position": "last"}
            ],
            "audio": true,
            "duration": 8,
            "promptText": "Move between the keyframes",
            "ratio": "3840:1646",
            "referenceAudio": [{"type": "audio", "uri": "https://example.com/score.mp3"}]
        })
    );
}

#[test]
fn image_to_video_established_current_variants_match_official_shapes() {
    let gen45 = ImageToVideoGen45Request::new(
        "Open the book",
        "https://example.com/book.png",
        VideoRatio::Square,
        5,
    )
    .seed(11);
    let turbo =
        ImageToVideoGen4TurboRequest::new("https://example.com/car.png", VideoRatio::Ultrawide)
            .duration(6)
            .prompt_text("Drive forward");
    let veo31 = ImageToVideoVeo31Request::new(
        PromptImageInput::first_frame("https://example.com/first.png")
            .with_last_frame("https://example.com/last.png"),
        VideoRatio::HdLandscape,
    )
    .audio(true)
    .duration(8)
    .negative_prompt("")
    .prompt_text("Transition smoothly");
    let veo31_fast =
        ImageToVideoVeo31FastRequest::new("https://example.com/fast.png", VideoRatio::Portrait)
            .negative_prompt("blur");
    let veo3 = ImageToVideoVeo3Request::new("https://example.com/veo3.png", VideoRatio::Landscape)
        .negative_prompt("text")
        .prompt_text("A gentle orbit");

    assert_eq!(
        serde_json::to_value(gen45).unwrap(),
        serde_json::json!({
            "duration": 5,
            "model": "gen4.5",
            "promptImage": "https://example.com/book.png",
            "promptText": "Open the book",
            "ratio": "960:960",
            "seed": 11
        })
    );
    assert_eq!(
        serde_json::to_value(turbo).unwrap(),
        serde_json::json!({
            "model": "gen4_turbo",
            "promptImage": "https://example.com/car.png",
            "ratio": "1584:672",
            "duration": 6,
            "promptText": "Drive forward"
        })
    );
    assert_eq!(
        serde_json::to_value(veo31).unwrap(),
        serde_json::json!({
            "model": "veo3.1",
            "promptImage": [
                {"position": "first", "uri": "https://example.com/first.png"},
                {"position": "last", "uri": "https://example.com/last.png"}
            ],
            "ratio": "1920:1080",
            "audio": true,
            "duration": 8,
            "negativePrompt": "",
            "promptText": "Transition smoothly"
        })
    );
    assert_eq!(
        serde_json::to_value(veo31_fast).unwrap(),
        serde_json::json!({
            "model": "veo3.1_fast",
            "promptImage": "https://example.com/fast.png",
            "ratio": "720:1280",
            "negativePrompt": "blur"
        })
    );
    assert_eq!(
        serde_json::to_value(veo3).unwrap(),
        serde_json::json!({
            "duration": 8,
            "model": "veo3",
            "promptImage": "https://example.com/veo3.png",
            "ratio": "1280:720",
            "negativePrompt": "text",
            "promptText": "A gentle orbit"
        })
    );
}

#[test]
fn text_to_video_veo_variants_serialize_negative_prompt() {
    let veo31 = TextToVideoVeo31Request::new("A sunny beach", VideoRatio::HdLandscape)
        .audio(true)
        .duration(6)
        .negative_prompt("rain");
    let fast = TextToVideoVeo31FastRequest::new("A fast pan", VideoRatio::Landscape)
        .negative_prompt("camera shake");
    let veo3 = TextToVideoVeo3Request::new("A quiet forest", VideoRatio::Portrait)
        .negative_prompt("people");

    assert_eq!(
        serde_json::to_value(veo31).unwrap(),
        serde_json::json!({
            "model": "veo3.1",
            "promptText": "A sunny beach",
            "ratio": "1920:1080",
            "audio": true,
            "duration": 6,
            "negativePrompt": "rain"
        })
    );
    assert_eq!(
        serde_json::to_value(fast).unwrap(),
        serde_json::json!({
            "model": "veo3.1_fast",
            "promptText": "A fast pan",
            "ratio": "1280:720",
            "negativePrompt": "camera shake"
        })
    );
    assert_eq!(
        serde_json::to_value(veo3).unwrap(),
        serde_json::json!({
            "duration": 8,
            "model": "veo3",
            "promptText": "A quiet forest",
            "ratio": "720:1280",
            "negativePrompt": "people"
        })
    );
}

#[test]
fn text_to_video_gen45_serializes_official_wire_shape() {
    let request = TextToVideoGen45Request::new("A paper boat", VideoRatio::Portrait, 4)
        .content_moderation(ContentModeration::new())
        .seed(9);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "duration": 4,
            "model": "gen4.5",
            "promptText": "A paper boat",
            "ratio": "720:1280",
            "contentModeration": {},
            "seed": 9
        })
    );
}

#[test]
fn text_to_video_new_model_family_uses_official_discriminants() {
    let fast = TextToVideoSeedance2FastRequest::new("Fast animation").ratio(VideoRatio::R992x432);
    let mini = TextToVideoSeedance2MiniRequest::new("Compact animation").audio(false);
    let gemini = TextToVideoGeminiOmniFlashRequest::new("A cinematic city")
        .duration(3)
        .ratio(VideoRatio::Portrait);

    assert_eq!(
        serde_json::to_value(fast).unwrap(),
        serde_json::json!({
            "model": "seedance2_fast",
            "promptText": "Fast animation",
            "ratio": "992:432"
        })
    );
    assert_eq!(
        serde_json::to_value(mini).unwrap(),
        serde_json::json!({
            "model": "seedance2_mini",
            "promptText": "Compact animation",
            "audio": false
        })
    );
    assert_eq!(
        serde_json::to_value(gemini).unwrap(),
        serde_json::json!({
            "model": "gemini_omni_flash",
            "promptText": "A cinematic city",
            "duration": 3,
            "ratio": "720:1280"
        })
    );
}

#[test]
fn text_to_video_seedance2_serializes_typed_references() {
    let request = TextToVideoSeedance2Request::new("Animate the references")
        .audio(true)
        .duration(10)
        .ratio(VideoRatio::R3840x2160)
        .reference_audio(vec![Seedance2AudioReference::new(
            "https://example.com/audio.mp3",
        )])
        .references(vec![Seedance2ImageReference::new(
            "https://example.com/image.png",
        )])
        .reference_videos(vec![Seedance2VideoReference::new(
            "https://example.com/video.mp4",
        )]);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        serde_json::json!({
            "model": "seedance2",
            "promptText": "Animate the references",
            "audio": true,
            "duration": 10,
            "ratio": "3840:2160",
            "referenceAudio": [{"type": "audio", "uri": "https://example.com/audio.mp3"}],
            "references": [{"uri": "https://example.com/image.png"}],
            "referenceVideos": [{"type": "video", "uri": "https://example.com/video.mp4"}]
        })
    );
}

#[test]
fn validation_enforces_only_documented_prompt_and_duration_constraints() {
    let exact_happyhorse_limit = "😀".repeat(1250);
    assert!(TextToVideoHappyhorse10Request::new(exact_happyhorse_limit)
        .validate()
        .is_ok());
    assert!(TextToVideoHappyhorse10Request::new("😀".repeat(1251))
        .validate()
        .is_err());

    assert!(TextToVideoVeo31Request::new("scene", VideoRatio::Landscape)
        .negative_prompt("")
        .validate()
        .is_ok());
    assert!(
        ImageToVideoVeo3Request::new("https://example.com/frame.png", VideoRatio::Landscape)
            .negative_prompt("")
            .validate()
            .is_ok()
    );

    assert!(
        ImageToVideoHappyhorse10Request::new("https://example.com/frame.png")
            .duration(-2.5)
            .prompt_text("")
            .validate()
            .is_ok()
    );
    assert!(
        ImageToVideoSeedance2Request::new("https://example.com/frame.png")
            .duration(0)
            .prompt_text("")
            .validate()
            .is_ok()
    );

    assert!(TextToVideoGeminiOmniFlashRequest::new("scene")
        .duration(2)
        .validate()
        .is_err());
    assert!(TextToVideoGeminiOmniFlashRequest::new("scene")
        .duration(3)
        .validate()
        .is_ok());
    assert!(
        ImageToVideoGeminiOmniFlashRequest::new("https://example.com/frame.png")
            .duration(11)
            .validate()
            .is_err()
    );
}

#[test]
fn validation_tracks_model_specific_ratio_sets_and_seedance_image_modes() {
    let happyhorse_ratios = [
        VideoRatio::Landscape,
        VideoRatio::Portrait,
        VideoRatio::Square,
        VideoRatio::R1108x832,
        VideoRatio::R832x1108,
        VideoRatio::HdLandscape,
        VideoRatio::HdPortrait,
        VideoRatio::R1440x1440,
        VideoRatio::R1662x1248,
        VideoRatio::R1248x1662,
    ];
    for ratio in happyhorse_ratios {
        assert!(TextToVideoHappyhorse10Request::new("scene")
            .ratio(ratio)
            .validate()
            .is_ok());
    }

    assert!(TextToVideoSeedance2Request::new("scene")
        .ratio(VideoRatio::R3840x2160)
        .validate()
        .is_ok());
    assert!(TextToVideoSeedance2FastRequest::new("scene")
        .ratio(VideoRatio::R3840x2160)
        .validate()
        .is_err());
    assert!(TextToVideoSeedance2FastRequest::new("scene")
        .ratio(VideoRatio::R1470x630)
        .validate()
        .is_ok());

    let mixed = ImageToVideoSeedance2Request::new(vec![
        Seedance2PromptImage::first("https://example.com/first.png"),
        Seedance2PromptImage::reference("https://example.com/reference.png"),
    ]);
    assert!(mixed.validate().is_err());

    let only_last = ImageToVideoSeedance2Request::new(vec![Seedance2PromptImage::last(
        "https://example.com/last.png",
    )]);
    assert!(only_last.validate().is_ok());

    assert!(
        ImageToVideoSeedance2Request::new(Vec::<Seedance2PromptImage>::new())
            .validate()
            .is_ok()
    );

    let many_references = (0..12)
        .map(|index| Seedance2PromptImage::reference(format!("https://example.com/{index}.png")))
        .collect::<Vec<_>>();
    assert!(ImageToVideoSeedance2Request::new(many_references)
        .validate()
        .is_ok());
}

#[test]
fn validation_enforces_reference_and_aleph_keyframe_relationships() {
    assert!(
        ImageToVideoSeedance2Request::new("https://example.com/frame.png")
            .reference_audio(vec![Seedance2AudioReference::new(
                "https://example.com/audio.mp3",
            )])
            .validate()
            .is_err()
    );
    assert!(
        VideoToVideoSeedance2Request::new("https://example.com/input.mp4")
            .reference_audio(vec![Seedance2AudioReference::new(
                "https://example.com/audio.mp3",
            )])
            .validate()
            .is_err()
    );

    let last_only = ImageToVideoVeo31Request::new(
        PromptImageInput::Frames(vec![PromptFrame::last("https://example.com/last.png")]),
        VideoRatio::Landscape,
    );
    assert!(last_only.validate().is_err());

    assert!(
        VideoToVideoAleph2Request::new("https://example.com/input.mp4")
            .keyframes(vec![
                Aleph2Keyframe::seconds("https://example.com/a.png", 2.0)
                    .range(Aleph2EditRange::new(1, 4)),
                Aleph2Keyframe::at("https://example.com/b.png", 0.5),
            ])
            .validate()
            .is_err()
    );
    assert!(
        VideoToVideoAleph2Request::new("https://example.com/input.mp4")
            .keyframes(vec![Aleph2Keyframe::at("https://example.com/a.png", 1.1,)])
            .validate()
            .is_err()
    );
    assert!(
        VideoToVideoAleph2Request::new("https://example.com/input.mp4")
            .keyframes(vec![Aleph2Keyframe::seconds(
                "https://example.com/a.png",
                5.0,
            )
            .range(Aleph2EditRange::new(1, 5))])
            .validate()
            .is_err()
    );
    assert!(
        VideoToVideoAleph2Request::new("https://example.com/input.mp4")
            .keyframes(
                (0..6)
                    .map(|index| {
                        Aleph2Keyframe::at(
                            format!("https://example.com/{index}.png"),
                            f64::from(index) / 5.0,
                        )
                    })
                    .collect(),
            )
            .validate()
            .is_err()
    );
}

#[tokio::test]
async fn all_video_generation_endpoints_forward_request_options_and_metadata() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .and(header("Idempotency-Key", "text-parity"))
        .and(body_json(serde_json::json!({
            "model": "happyhorse_1_0",
            "promptText": "Options text"
        })))
        .respond_with(
            ResponseTemplate::new(201)
                .append_header("x-parity-endpoint", "text")
                .set_body_json(serde_json::json!({
                    "id": "10000000-0000-4000-8000-000000000001"
                })),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/image_to_video"))
        .and(header("Idempotency-Key", "image-parity"))
        .and(body_json(serde_json::json!({
            "model": "gemini_omni_flash",
            "promptImage": "https://example.com/frame.png"
        })))
        .respond_with(
            ResponseTemplate::new(202)
                .append_header("x-parity-endpoint", "image")
                .set_body_json(serde_json::json!({
                    "id": "20000000-0000-4000-8000-000000000002"
                })),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/video_to_video"))
        .and(header("Idempotency-Key", "video-parity"))
        .and(body_json(serde_json::json!({
            "model": "aleph2",
            "videoUri": "https://example.com/input.mp4"
        })))
        .respond_with(
            ResponseTemplate::new(203)
                .append_header("x-parity-endpoint", "video")
                .set_body_json(serde_json::json!({
                    "id": "30000000-0000-4000-8000-000000000003"
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(
        Config::new("test-key")
            .base_url(server.uri())
            .max_retries(0),
    )
    .unwrap();

    let text = client
        .text_to_video()
        .create_with_options(
            TextToVideoHappyhorse10Request::new("Options text"),
            RequestOptions::new().idempotency_key("text-parity"),
        )
        .await
        .unwrap();
    let image = client
        .image_to_video()
        .create_with_options(
            ImageToVideoGeminiOmniFlashRequest::new("https://example.com/frame.png"),
            RequestOptions::new().idempotency_key("image-parity"),
        )
        .await
        .unwrap();
    let video = client
        .video_to_video()
        .create_with_options(
            VideoToVideoAleph2Request::new("https://example.com/input.mp4"),
            RequestOptions::new().idempotency_key("video-parity"),
        )
        .await
        .unwrap();

    assert_eq!(text.response.status, 201);
    assert_eq!(
        text.data.id().to_string(),
        "10000000-0000-4000-8000-000000000001"
    );
    assert_eq!(text.response.headers["x-parity-endpoint"], "text");
    assert_eq!(image.response.status, 202);
    assert_eq!(
        image.data.id().to_string(),
        "20000000-0000-4000-8000-000000000002"
    );
    assert_eq!(image.response.headers["x-parity-endpoint"], "image");
    assert_eq!(video.response.status, 203);
    assert_eq!(
        video.data.id().to_string(),
        "30000000-0000-4000-8000-000000000003"
    );
    assert_eq!(video.response.headers["x-parity-endpoint"], "video");
}
