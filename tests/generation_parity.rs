use runway_sdk::*;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(base_url: &str) -> Config {
    Config::new("test-api-key-12345").base_url(base_url)
}

macro_rules! assert_string_values {
    ($type:ty, [$($expected:literal),+ $(,)?]) => {{
        let actual = <$type>::ALL
            .iter()
            .copied()
            .map(<$type>::as_str)
            .collect::<Vec<_>>();
        assert_eq!(actual, vec![$($expected),+]);
    }};
}

fn assert_validation<T>(result: Result<T, RunwayError>) {
    assert!(matches!(result, Err(RunwayError::Validation { .. })));
}

#[test]
fn text_to_image_ratio_discriminants_match_official_schema_exhaustively() {
    assert_string_values!(
        GptImage2Ratio,
        [
            "2048:880",
            "1920:1088",
            "1920:1280",
            "1920:1440",
            "1920:1536",
            "1920:1920",
            "1536:1920",
            "1440:1920",
            "1280:1920",
            "1088:1920",
            "2912:1248",
            "2560:1440",
            "2560:1712",
            "2560:1920",
            "2560:2048",
            "2560:2560",
            "2048:2560",
            "1920:2560",
            "1712:2560",
            "1440:2560",
            "3840:1648",
            "3840:2160",
            "3504:2336",
            "3264:2448",
            "3200:2560",
            "2880:2880",
            "2560:3200",
            "2448:3264",
            "2336:3504",
            "2160:3840",
            "auto",
        ]
    );
    assert_string_values!(
        GeminiImage3ProRatio,
        [
            "1344:768",
            "768:1344",
            "1024:1024",
            "1184:864",
            "864:1184",
            "1536:672",
            "832:1248",
            "1248:832",
            "896:1152",
            "1152:896",
            "2048:2048",
            "1696:2528",
            "2528:1696",
            "1792:2400",
            "2400:1792",
            "1856:2304",
            "2304:1856",
            "1536:2752",
            "2752:1536",
            "3168:1344",
            "4096:4096",
            "3392:5056",
            "5056:3392",
            "3584:4800",
            "4800:3584",
            "3712:4608",
            "4608:3712",
            "3072:5504",
            "5504:3072",
            "6336:2688",
        ]
    );
    assert_string_values!(
        GeminiImage31FlashRatio,
        [
            "512:512",
            "416:624",
            "624:416",
            "432:592",
            "592:432",
            "448:576",
            "576:448",
            "384:672",
            "672:384",
            "768:336",
            "256:1024",
            "1024:256",
            "176:1408",
            "1408:176",
            "1024:1024",
            "832:1248",
            "1248:832",
            "864:1184",
            "1184:864",
            "896:1152",
            "1152:896",
            "768:1344",
            "1344:768",
            "1536:672",
            "512:2048",
            "2048:512",
            "352:2816",
            "2816:352",
            "2048:2048",
            "1696:2528",
            "2528:1696",
            "1792:2400",
            "2400:1792",
            "1856:2304",
            "2304:1856",
            "1536:2752",
            "2752:1536",
            "3168:1344",
            "1024:4096",
            "4096:1024",
            "704:5632",
            "5632:704",
            "4096:4096",
            "3392:5056",
            "5056:3392",
            "3584:4800",
            "4800:3584",
            "3712:4608",
            "4608:3712",
            "3072:5504",
            "5504:3072",
            "6336:2688",
            "2048:8192",
            "8192:2048",
            "1408:11264",
            "11264:1408",
        ]
    );
    assert_string_values!(
        Seedream5ProRatio,
        [
            "1024:1024",
            "1184:896",
            "896:1184",
            "1376:768",
            "768:1376",
            "1296:864",
            "864:1296",
            "2048:2048",
            "2304:1728",
            "1728:2304",
            "2720:1530",
            "1530:2720",
            "2496:1664",
            "1664:2496",
            "auto_1k",
            "auto_2k",
        ]
    );
    assert_string_values!(
        Seedream5LiteRatio,
        [
            "2048:2048",
            "2304:1728",
            "1728:2304",
            "2848:1600",
            "1600:2848",
            "2496:1664",
            "1664:2496",
            "3136:1344",
            "3072:3072",
            "3456:2592",
            "2592:3456",
            "4096:2304",
            "2304:4096",
            "3744:2496",
            "2496:3744",
            "4704:2016",
        ]
    );
}

#[test]
fn text_to_image_new_variants_serialize_full_official_wire_bodies() {
    let gpt = TextToImageGptImage2Request::new("Editorial portrait", GptImage2Ratio::Auto)
        .background(GptImage2Background::Opaque)
        .output_count(10)
        .quality(GptImage2Quality::High)
        .reference_images(vec![GptImage2ReferenceImage::new(
            "https://example.com/gpt.png",
        )
        .tag("subject")]);
    assert_eq!(
        serde_json::to_value(TextToImageCreateRequest::from(gpt)).unwrap(),
        json!({
            "model": "gpt_image_2",
            "promptText": "Editorial portrait",
            "ratio": "auto",
            "background": "opaque",
            "outputCount": 10,
            "quality": "high",
            "referenceImages": [{"uri": "https://example.com/gpt.png", "tag": "subject"}]
        })
    );

    let pro =
        TextToImageGeminiImage3ProRequest::new("Character study", GeminiImage3ProRatio::R6336x2688)
            .output_count(GeminiImageOutputCount::Four)
            .reference_images(vec![GeminiImageReferenceImage::new(
                "https://example.com/human.png",
            )
            .subject(GeminiImageReferenceSubject::Human)
            .tag("hero")]);
    assert_eq!(
        serde_json::to_value(TextToImageCreateRequest::from(pro)).unwrap(),
        json!({
            "model": "gemini_image3_pro",
            "promptText": "Character study",
            "ratio": "6336:2688",
            "outputCount": 4,
            "referenceImages": [{
                "uri": "https://example.com/human.png",
                "subject": "human",
                "tag": "hero"
            }]
        })
    );

    let flash = TextToImageGeminiImage31FlashRequest::new(
        "Wide environment",
        GeminiImage31FlashRatio::R11264x1408,
    )
    .output_count(GeminiImageOutputCount::One)
    .reference_images(vec![GeminiImageReferenceImage::new(
        "https://example.com/object.png",
    )
    .subject(GeminiImageReferenceSubject::Object)]);
    assert_eq!(
        serde_json::to_value(TextToImageCreateRequest::from(flash)).unwrap(),
        json!({
            "model": "gemini_image3.1_flash",
            "promptText": "Wide environment",
            "ratio": "11264:1408",
            "outputCount": 1,
            "referenceImages": [{
                "uri": "https://example.com/object.png",
                "subject": "object"
            }]
        })
    );

    let pro = TextToImageSeedream5ProRequest::new("Product fusion", Seedream5ProRatio::Auto2k)
        .output_count(3.0)
        .output_format(SeedreamOutputFormat::Jpeg)
        .reference_images(vec![SeedreamReferenceImage::new(
            "https://example.com/product.png",
        )]);
    assert_eq!(
        serde_json::to_value(TextToImageCreateRequest::from(pro)).unwrap(),
        json!({
            "model": "seedream5_pro",
            "promptText": "Product fusion",
            "ratio": "auto_2k",
            "outputCount": 3.0,
            "outputFormat": "jpeg",
            "referenceImages": [{"uri": "https://example.com/product.png"}]
        })
    );

    let lite =
        TextToImageSeedream5LiteRequest::new("Campaign image", Seedream5LiteRatio::R4704x2016)
            .output_count(2.0)
            .output_format(SeedreamOutputFormat::Png)
            .reference_images(vec![SeedreamReferenceImage::new(
                "https://example.com/campaign.png",
            )]);
    assert_eq!(
        serde_json::to_value(TextToImageCreateRequest::from(lite)).unwrap(),
        json!({
            "model": "seedream5_lite",
            "promptText": "Campaign image",
            "ratio": "4704:2016",
            "outputCount": 2.0,
            "outputFormat": "png",
            "referenceImages": [{"uri": "https://example.com/campaign.png"}]
        })
    );
}

#[test]
fn text_to_image_validates_official_bounds() {
    assert_validation(
        TextToImageGptImage2Request::new("x", GptImage2Ratio::Auto)
            .output_count(0)
            .validate(),
    );
    assert_validation(
        TextToImageGptImage2Request::new("x".repeat(32_001), GptImage2Ratio::Auto).validate(),
    );
    assert_validation(
        TextToImageGptImage2Request::new("x", GptImage2Ratio::Auto)
            .reference_images(
                (0..17)
                    .map(|index| GptImage2ReferenceImage::new(format!("https://x/{index}")))
                    .collect(),
            )
            .validate(),
    );

    let six_humans = (0..6)
        .map(|index| {
            GeminiImageReferenceImage::new(format!("https://x/human-{index}"))
                .subject(GeminiImageReferenceSubject::Human)
        })
        .collect();
    assert_validation(
        TextToImageGeminiImage3ProRequest::new("x", GeminiImage3ProRatio::R1024x1024)
            .reference_images(six_humans)
            .validate(),
    );

    let ten_objects = (0..10)
        .map(|index| {
            GeminiImageReferenceImage::new(format!("https://x/object-{index}"))
                .subject(GeminiImageReferenceSubject::Object)
        })
        .collect();
    assert_validation(
        TextToImageGeminiImage31FlashRequest::new("x", GeminiImage31FlashRatio::R512x512)
            .reference_images(ten_objects)
            .validate(),
    );
    assert_validation(
        TextToImageGeminiImage3ProRequest::new("x", GeminiImage3ProRatio::R1024x1024)
            .reference_images(
                (0..15)
                    .map(|index| GeminiImageReferenceImage::new(format!("https://x/{index}")))
                    .collect(),
            )
            .validate(),
    );
    assert_validation(
        TextToImageSeedream5ProRequest::new("x".repeat(4_001), Seedream5ProRatio::R1024x1024)
            .validate(),
    );
    assert_validation(
        TextToImageSeedream5ProRequest::new("x", Seedream5ProRatio::R1024x1024)
            .output_count(f64::NAN)
            .validate(),
    );
    assert_validation(
        TextToImageSeedream5LiteRequest::new("x", Seedream5LiteRatio::R2048x2048)
            .reference_images(vec![SeedreamReferenceImage::new(" ")])
            .validate(),
    );
    assert!(
        TextToImageCreateRequest::from(TextToImageGemini25FlashRequest::new(
            "x".repeat(1_001),
            ImageRatio::Square1024,
        ))
        .validate()
        .is_ok()
    );
    assert!(serde_json::from_value::<GeminiImageOutputCount>(json!(2)).is_err());
}

#[test]
fn seed_audio_and_legacy_sound_serialize_exact_wire_bodies() {
    let seed_sound = SeedAudioSoundEffectRequest::new("A storm with distant dialogue")
        .loudness_rate(-50.0)
        .output_format(SeedAudioOutputFormat::Wav)
        .pitch_rate(-12.0)
        .reference_audios(vec!["https://example.com/reference.wav".into()])
        .sample_rate(SeedAudioSampleRate::Hz8000)
        .speech_rate(-50.0);
    assert_eq!(
        serde_json::to_value(SoundEffectCreateRequest::from(seed_sound)).unwrap(),
        json!({
            "model": "seed_audio",
            "promptText": "A storm with distant dialogue",
            "loudnessRate": -50.0,
            "outputFormat": "wav",
            "pitchRate": -12.0,
            "referenceAudios": ["https://example.com/reference.wav"],
            "sampleRate": 8000,
            "speechRate": -50.0
        })
    );

    let seed_speech = SeedAudioTextToSpeechRequest::new("Speak this line")
        .loudness_rate(2.0)
        .output_format(SeedAudioOutputFormat::OggOpus)
        .pitch_rate(1.5)
        .sample_rate(SeedAudioSampleRate::Hz48000)
        .speech_rate(-2.0)
        .voice(SeedAudioReferenceVoice::new(
            "https://example.com/voice.wav",
        ));
    assert_eq!(
        serde_json::to_value(TextToSpeechCreateRequest::from(seed_speech)).unwrap(),
        json!({
            "model": "seed_audio",
            "promptText": "Speak this line",
            "loudnessRate": 2.0,
            "outputFormat": "ogg_opus",
            "pitchRate": 1.5,
            "sampleRate": 48000,
            "speechRate": -2.0,
            "voice": {
                "audioUri": "https://example.com/voice.wav",
                "type": "reference-audio"
            }
        })
    );

    let legacy = SoundEffectRequest::new("A seamless ocean loop")
        .duration(4.5)
        .loop_output(true);
    assert_eq!(
        serde_json::to_value(SoundEffectCreateRequest::from(legacy)).unwrap(),
        json!({
            "model": "eleven_text_to_sound_v2",
            "promptText": "A seamless ocean loop",
            "duration": 4.5,
            "loop": true
        })
    );
}

#[test]
fn audio_generation_validates_reference_and_numeric_contracts() {
    assert_eq!(
        SeedAudioSampleRate::ALL
            .iter()
            .copied()
            .map(SeedAudioSampleRate::as_hz)
            .collect::<Vec<_>>(),
        vec![8_000, 16_000, 24_000, 32_000, 44_100, 48_000]
    );
    assert_validation(SeedAudioSoundEffectRequest::new(" ").validate());
    assert_validation(
        SeedAudioSoundEffectRequest::new("x")
            .reference_audios(vec!["a".into(), "b".into(), "c".into(), "d".into()])
            .validate(),
    );
    assert_validation(
        SeedAudioSoundEffectRequest::new("x")
            .speech_rate(f64::INFINITY)
            .validate(),
    );
    assert_validation(
        ElevenTextToSoundV2Request::new("x")
            .duration(30.1)
            .validate(),
    );
    assert_validation(
        SeedAudioTextToSpeechRequest::new("x")
            .voice(SeedAudioReferenceVoice::new(" "))
            .validate(),
    );
    assert!(serde_json::from_value::<SeedAudioSampleRate>(json!(44_000)).is_err());
}

#[test]
fn upscale_requests_serialize_full_official_wire_bodies() {
    let image = ImageUpscaleCreateRequest::new("https://example.com/image.jpg")
        .flavor(ImageUpscaleFlavor::PhotoDenoiser)
        .scale_factor(ImageUpscaleScaleFactor::X16)
        .sharpen(12.5)
        .smart_grain(25.0)
        .ultra_detail(100.0);
    assert_eq!(
        serde_json::to_value(image).unwrap(),
        json!({
            "imageUri": "https://example.com/image.jpg",
            "model": "magnific_precision_upscaler_v2",
            "flavor": "photo_denoiser",
            "scaleFactor": 16,
            "sharpen": 12.5,
            "smartGrain": 25.0,
            "ultraDetail": 100.0
        })
    );

    let video = VideoUpscaleCreateRequest::new("https://example.com/video.mp4")
        .creativity(0.0)
        .flavor(VideoUpscaleFlavor::Vivid)
        .fps_boost(true)
        .resolution(VideoUpscaleResolution::P720)
        .sharpen(50.0)
        .smart_grain(100.0);
    assert_eq!(
        serde_json::to_value(video).unwrap(),
        json!({
            "model": "magnific_video_upscaler_creative",
            "videoUri": "https://example.com/video.mp4",
            "creativity": 0.0,
            "flavor": "vivid",
            "fpsBoost": true,
            "resolution": "720p",
            "sharpen": 50.0,
            "smartGrain": 100.0
        })
    );

    assert_eq!(
        ImageUpscaleScaleFactor::ALL
            .iter()
            .copied()
            .map(ImageUpscaleScaleFactor::as_u8)
            .collect::<Vec<_>>(),
        vec![2, 4, 8, 16]
    );
    assert_eq!(
        serde_json::to_value([
            VideoUpscaleResolution::P720,
            VideoUpscaleResolution::K1,
            VideoUpscaleResolution::K2,
            VideoUpscaleResolution::K4,
        ])
        .unwrap(),
        json!(["720p", "1k", "2k", "4k"])
    );
}

#[test]
fn upscale_requests_validate_uri_and_zero_to_one_hundred_ranges() {
    assert!(ImageUpscaleCreateRequest::new("image").validate().is_ok());
    assert!(VideoUpscaleCreateRequest::new("video")
        .creativity(0.0)
        .sharpen(100.0)
        .smart_grain(50.5)
        .validate()
        .is_ok());
    assert_validation(ImageUpscaleCreateRequest::new(" ").validate());
    assert_validation(
        ImageUpscaleCreateRequest::new("image")
            .ultra_detail(100.1)
            .validate(),
    );
    assert_validation(
        VideoUpscaleCreateRequest::new("video")
            .creativity(-0.1)
            .validate(),
    );
    assert_validation(
        VideoUpscaleCreateRequest::new("video")
            .smart_grain(f64::NAN)
            .validate(),
    );
    assert!(serde_json::from_value::<ImageUpscaleScaleFactor>(json!(3)).is_err());
}

async fn mount_task_mock(
    server: &MockServer,
    endpoint: &'static str,
    idempotency_key: &'static str,
    body: serde_json::Value,
    task_id: &'static str,
) {
    Mock::given(method("POST"))
        .and(path(endpoint))
        .and(header("Idempotency-Key", idempotency_key))
        .and(body_json(body))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"id": task_id})))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn generation_resources_send_exact_bodies_and_preserve_with_options_metadata() {
    let server = MockServer::start().await;
    mount_task_mock(
        &server,
        "/v1/text_to_image",
        "text-image",
        json!({"model": "gpt_image_2", "promptText": "x", "ratio": "auto"}),
        "10000000-0000-4000-8000-000000000001",
    )
    .await;
    mount_task_mock(
        &server,
        "/v1/sound_effect",
        "sound",
        json!({
            "model": "eleven_text_to_sound_v2",
            "promptText": "loop",
            "loop": true
        }),
        "10000000-0000-4000-8000-000000000002",
    )
    .await;
    mount_task_mock(
        &server,
        "/v1/text_to_speech",
        "speech",
        json!({"model": "seed_audio", "promptText": "hello"}),
        "10000000-0000-4000-8000-000000000003",
    )
    .await;
    mount_task_mock(
        &server,
        "/v1/image_upscale",
        "image-upscale",
        json!({
            "imageUri": "https://example.com/image.jpg",
            "model": "magnific_precision_upscaler_v2"
        }),
        "10000000-0000-4000-8000-000000000004",
    )
    .await;
    mount_task_mock(
        &server,
        "/v1/video_upscale",
        "video-upscale",
        json!({
            "model": "magnific_video_upscaler_creative",
            "videoUri": "https://example.com/video.mp4"
        }),
        "10000000-0000-4000-8000-000000000005",
    )
    .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let text_image = client
        .text_to_image()
        .create_with_options(
            TextToImageGptImage2Request::new("x", GptImage2Ratio::Auto),
            RequestOptions::new().idempotency_key("text-image"),
        )
        .await
        .unwrap();
    let sound = client
        .sound_effect()
        .create_with_options(
            SoundEffectRequest::new("loop").loop_output(true),
            RequestOptions::new().idempotency_key("sound"),
        )
        .await
        .unwrap();
    let speech = client
        .text_to_speech()
        .create_with_options(
            SeedAudioTextToSpeechRequest::new("hello"),
            RequestOptions::new().idempotency_key("speech"),
        )
        .await
        .unwrap();
    let image = client
        .image_upscale()
        .create_with_options(
            ImageUpscaleCreateRequest::new("https://example.com/image.jpg"),
            RequestOptions::new().idempotency_key("image-upscale"),
        )
        .await
        .unwrap();
    let video = client
        .video_upscale()
        .create_with_options(
            VideoUpscaleCreateRequest::new("https://example.com/video.mp4"),
            RequestOptions::new().idempotency_key("video-upscale"),
        )
        .await
        .unwrap();

    for response in [&text_image, &sound, &speech, &image, &video] {
        assert_eq!(response.response.status, 202);
    }
    assert_eq!(
        text_image.data.id().to_string(),
        "10000000-0000-4000-8000-000000000001"
    );
    assert_eq!(
        video.data.id().to_string(),
        "10000000-0000-4000-8000-000000000005"
    );
}
