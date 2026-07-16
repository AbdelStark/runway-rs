use futures::StreamExt;
use runway_sdk::*;
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(base_url: &str) -> Config {
    Config::new("test-api-key-12345").base_url(base_url)
}

#[test]
fn avatar_conversation_query_serializes_official_filter_names() {
    let query = AvatarConversationListQuery::new()
        .cursor("cursor-2")
        .limit(25)
        .avatar("avatar-1")
        .start_date("2026-07-01T00:00:00Z")
        .end_date("2026-07-16T00:00:00Z");

    assert_eq!(
        serde_json::to_value(query).unwrap(),
        json!({
            "cursor": "cursor-2",
            "limit": 25,
            "avatar": "avatar-1",
            "startDate": "2026-07-01T00:00:00Z",
            "endDate": "2026-07-16T00:00:00Z"
        })
    );
}

#[test]
fn avatar_conversation_deserializes_failed_state_and_transcript_tools() {
    let conversation: AvatarConversation = serde_json::from_value(json!({
        "id": "conversation-1",
        "status": "failed",
        "avatar": {
            "type": "custom",
            "id": null,
            "imageUrl": null,
            "name": null
        },
        "createdAt": "2026-07-16T10:00:00Z",
        "duration": 12,
        "endedAt": "2026-07-16T10:00:12Z",
        "failure": "Backend tool failed",
        "failureCode": "TOOL_ERROR",
        "maxDuration": 300,
        "name": "Support call",
        "recordingUrl": null,
        "startedAt": "2026-07-16T10:00:00Z",
        "tools": [{
            "description": "Look up an order",
            "name": "lookup_order",
            "type": "backend_rpc"
        }],
        "transcript": [{
            "content": null,
            "role": "assistant",
            "timestamp": "2026-07-16T10:00:05Z",
            "toolCalls": [{
                "arguments": {"orderId": "order-7"},
                "name": "lookup_order",
                "id": "call-1"
            }],
            "toolResults": [{
                "name": "lookup_order",
                "id": "call-1",
                "durationMs": 82.5,
                "error": "upstream unavailable",
                "result": null
            }]
        }]
    }))
    .unwrap();

    assert_eq!(conversation.id(), "conversation-1");
    assert_eq!(conversation.status(), AvatarConversationStatus::Failed);
    let AvatarConversation::Failed(failed) = conversation else {
        panic!("expected failed conversation")
    };
    assert_eq!(failed.failure_code, "TOOL_ERROR");
    assert_eq!(failed.transcript[0].tool_calls.as_ref().unwrap().len(), 1);
}

#[test]
fn avatar_video_request_serializes_typed_avatar_speech_and_voice() {
    let request = AvatarVideoCreateRequest::new(
        AvatarVideoAvatar::runway_preset(AvatarVideoPresetId::FashionDesigner),
        AvatarVideoSpeech::text_with_voice(
            "Welcome to the collection.",
            AvatarVideoVoice::preset(AvatarVideoVoicePresetId::Victoria),
        ),
    );

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "avatar": {"type": "runway-preset", "presetId": "fashion-designer"},
            "model": "gwm1_avatars",
            "speech": {
                "type": "text",
                "text": "Welcome to the collection.",
                "voice": {"type": "preset", "presetId": "victoria"}
            }
        })
    );
}

#[test]
fn recipe_requests_serialize_official_discriminants_and_field_names() {
    let localization = RecipeAdLocalizationRequest::new(
        RecipeImage::new("https://example.com/ad.png"),
        RecipeAdLocalizationLanguage::ZhHant,
        RecipeVersion::V2026_06,
    );
    assert_eq!(
        serde_json::to_value(localization).unwrap(),
        json!({
            "referenceImage": {"uri": "https://example.com/ad.png"},
            "targetLanguage": "zh-Hant",
            "version": "2026-06"
        })
    );

    let marketing = RecipeMarketingStockImageRequest::new(
        "A premium studio product photograph",
        RecipeVersion::UnsafeLatest,
    )
    .output_count(4)
    .quality(RecipeImageQuality::High)
    .reference_image(RecipeImage::new("https://example.com/logo.png"));
    assert_eq!(
        serde_json::to_value(marketing).unwrap(),
        json!({
            "prompt": "A premium studio product photograph",
            "version": "unsafe-latest",
            "outputCount": 4,
            "quality": "high",
            "referenceImage": {"uri": "https://example.com/logo.png"}
        })
    );

    let multi_shot = RecipeMultiShotVideoRequest::custom(
        vec![
            RecipeMultiShot::new(5, "Wide establishing shot"),
            RecipeMultiShot::new(5, "Product close-up"),
            RecipeMultiShot::new(5, "Closing hero shot"),
        ],
        RecipeVersion::V2026_06,
    )
    .audio(true)
    .duration(RecipeMultiShotDuration::Fifteen)
    .first_frame(RecipeImage::new("https://example.com/first.png"))
    .ratio(RecipeMultiShotRatio::Portrait1080p);
    assert_eq!(
        serde_json::to_value(multi_shot).unwrap(),
        json!({
            "mode": "custom",
            "shots": [
                {"duration": 5, "prompt": "Wide establishing shot"},
                {"duration": 5, "prompt": "Product close-up"},
                {"duration": 5, "prompt": "Closing hero shot"}
            ],
            "version": "2026-06",
            "audio": true,
            "duration": 15,
            "firstFrame": {"uri": "https://example.com/first.png"},
            "ratio": "1080:1920"
        })
    );

    let product_ad = RecipeProductAdRequest::new(
        vec![RecipeImage::new("https://example.com/product.png")],
        ProductAdRecipeVersion::V2026_07,
    )
    .ratio(RecipeProductAdRatio::Portrait1248x1664)
    .user_concept("Editorial lighting");
    assert_eq!(
        serde_json::to_value(product_ad).unwrap(),
        json!({
            "productImages": [{"uri": "https://example.com/product.png"}],
            "version": "2026-07",
            "ratio": "1248:1664",
            "userConcept": "Editorial lighting"
        })
    );

    let campaign = RecipeProductCampaignImageRequest::new(
        RecipeImage::new("https://example.com/shoe.png"),
        "High-key fashion editorial",
        RecipeVersion::V2026_06,
    );
    assert_eq!(
        serde_json::to_value(campaign).unwrap(),
        json!({
            "image": {"uri": "https://example.com/shoe.png"},
            "prompt": "High-key fashion editorial",
            "version": "2026-06"
        })
    );

    let swap = RecipeProductSwapRequest::new(
        vec![RecipeProductSwapImage::new("https://example.com/new.png")
            .view(RecipeProductView::Front)],
        RecipeImage::new("https://example.com/original.png"),
        RecipeVideo::new("https://example.com/reference.mp4"),
        RecipeVersion::V2026_06,
    )
    .resolution(RecipeProductSwapResolution::P1080);
    assert_eq!(
        serde_json::to_value(swap).unwrap(),
        json!({
            "newProductImages": [{
                "uri": "https://example.com/new.png",
                "view": "front"
            }],
            "originalProductImage": {"uri": "https://example.com/original.png"},
            "referenceVideo": {"uri": "https://example.com/reference.mp4"},
            "version": "2026-06",
            "resolution": "1080p"
        })
    );

    let ugc = RecipeProductUgcRequest::new(
        RecipeImage::new("https://example.com/person.png"),
        RecipeImage::new("https://example.com/product.png"),
        RecipeVersion::V2026_06,
    )
    .duration(15)
    .ratio(RecipeProductUgcRatio::Portrait1080p)
    .product_info("A lightweight trail shoe");
    assert_eq!(
        serde_json::to_value(ugc).unwrap(),
        json!({
            "characterImage": {"uri": "https://example.com/person.png"},
            "productImage": {"uri": "https://example.com/product.png"},
            "version": "2026-06",
            "duration": 15,
            "productInfo": "A lightweight trail shoe",
            "ratio": "1080:1920"
        })
    );
}

#[test]
fn voice_update_request_distinguishes_omitted_set_and_cleared_description() {
    assert_eq!(
        serde_json::to_value(UpdateVoiceRequest::new()).unwrap(),
        json!({})
    );
    assert_eq!(
        serde_json::to_value(UpdateVoiceRequest::new().description("Warm narrator")).unwrap(),
        json!({"description": "Warm narrator"})
    );
    assert_eq!(
        serde_json::to_value(UpdateVoiceRequest::new().clear_description()).unwrap(),
        json!({"description": null})
    );
}

#[test]
fn voice_create_supports_audio_text_and_explicitly_cleared_description() {
    assert_eq!(
        serde_json::to_value(CreateVoiceRequest::from_audio(
            "Cloned voice",
            "https://example.com/source.wav",
        ))
        .unwrap(),
        json!({
            "from": {"type": "audio", "audio": "https://example.com/source.wav"},
            "name": "Cloned voice"
        })
    );
    assert_eq!(
        serde_json::to_value(
            CreateVoiceRequest::new(
                "Designed voice",
                "A warm and confident documentary narrator",
            )
            .clear_description(),
        )
        .unwrap(),
        json!({
            "from": {
                "type": "text",
                "model": "eleven_ttv_v3",
                "prompt": "A warm and confident documentary narrator"
            },
            "name": "Designed voice",
            "description": null
        })
    );
}

#[tokio::test]
async fn voice_audio_create_uses_exact_wire_body_and_prompt_minimum_is_local() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/voices"))
        .and(body_json(json!({
            "from": {"type": "audio", "audio": "https://example.com/source.wav"},
            "name": "Cloned voice"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "voice-audio"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let voice = client
        .voices()
        .create(CreateVoiceRequest::from_audio(
            "Cloned voice",
            "https://example.com/source.wav",
        ))
        .await
        .unwrap();
    assert_eq!(voice.id, "voice-audio");

    let create_error = client
        .voices()
        .create(CreateVoiceRequest::new("Too short", "short"))
        .await
        .unwrap_err();
    let preview_error = client
        .voices()
        .preview(PreviewVoiceRequest::new("also short"))
        .await
        .unwrap_err();
    assert!(matches!(create_error, RunwayError::Validation { .. }));
    assert!(matches!(preview_error, RunwayError::Validation { .. }));
}

#[tokio::test]
async fn avatar_usage_uses_official_path_query_and_response_shape() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/avatar_usage"))
        .and(query_param("startDate", "2026-07-01T00:00:00Z"))
        .and(query_param("endDate", "2026-07-16T00:00:00Z"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "avgDurationSeconds": 42,
            "byDay": [{"date": "2026-07-01", "seconds": 84, "sessions": 2}],
            "totalSeconds": 84,
            "totalSessions": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let usage = client
        .avatars()
        .get_usage(AvatarUsageQuery::new(
            "2026-07-01T00:00:00Z",
            "2026-07-16T00:00:00Z",
        ))
        .await
        .unwrap();

    assert_eq!(usage.avg_duration_seconds, 42);
    assert_eq!(usage.by_day[0].sessions, 2);
}

#[tokio::test]
async fn voice_update_uses_patch_and_percent_encoded_identifier() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/voices/voice%2Fwith%20space"))
        .and(body_json(json!({"name": "Updated voice"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "voice/with space",
            "status": "READY",
            "createdAt": "2026-07-01T00:00:00Z",
            "description": null,
            "name": "Updated voice",
            "previewUrl": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let voice = client
        .voices()
        .update(
            "voice/with space",
            UpdateVoiceRequest::new().name("Updated voice"),
        )
        .await
        .unwrap();

    assert_eq!(voice.id(), "voice/with space");
}

#[tokio::test]
async fn avatar_conversations_use_all_official_paths() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/avatar_conversations"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "nextCursor": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/avatar_conversations/conversation%2Fone"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "conversation/one",
            "status": "in_progress",
            "avatar": null,
            "createdAt": "2026-07-16T10:00:00Z",
            "duration": null,
            "maxDuration": null,
            "name": "Conversation one",
            "recordingUrl": null,
            "startedAt": null,
            "tools": [],
            "transcript": []
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v1/avatar_conversations/conversation%2Ftwo"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let list = client
        .avatar_conversations()
        .list(AvatarConversationListQuery::new().limit(10))
        .await
        .unwrap();
    let conversation = client
        .avatar_conversations()
        .retrieve("conversation/one")
        .await
        .unwrap();
    client
        .avatar_conversations()
        .delete("conversation/two")
        .await
        .unwrap();

    assert!(list.data.is_empty());
    assert_eq!(conversation.id(), "conversation/one");
}

#[tokio::test]
async fn avatar_videos_create_uses_official_path_and_task_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/avatar_videos"))
        .and(body_json(json!({
            "avatar": {"type": "custom", "avatarId": "avatar-1"},
            "model": "gwm1_avatars",
            "speech": {"type": "audio", "audio": "https://example.com/speech.wav"}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "10000000-0000-4000-8000-000000000001"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let pending = client
        .avatar_videos()
        .create(AvatarVideoCreateRequest::new(
            AvatarVideoAvatar::custom("avatar-1"),
            AvatarVideoSpeech::audio("https://example.com/speech.wav"),
        ))
        .await
        .unwrap();

    assert_eq!(
        pending.id().to_string(),
        "10000000-0000-4000-8000-000000000001"
    );
}

#[tokio::test]
async fn recipes_use_all_seven_official_paths() {
    let server = MockServer::start().await;
    let paths = [
        "/v1/recipes/ad_localization",
        "/v1/recipes/marketing_stock_image",
        "/v1/recipes/multi_shot_video",
        "/v1/recipes/product_ad",
        "/v1/recipes/product_campaign_image",
        "/v1/recipes/product_swap",
        "/v1/recipes/product_ugc",
    ];
    for (index, route) in paths.iter().enumerate() {
        Mock::given(method("POST"))
            .and(path(*route))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": format!("20000000-0000-4000-8000-{index:012}")
            })))
            .expect(1)
            .mount(&server)
            .await;
    }

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();
    let recipes = client.recipes();

    let tasks = [
        recipes
            .ad_localization(RecipeAdLocalizationRequest::new(
                RecipeImage::new("https://example.com/ad.png"),
                RecipeAdLocalizationLanguage::Fr,
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
        recipes
            .marketing_stock_image(RecipeMarketingStockImageRequest::new(
                "A campaign image",
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
        recipes
            .multi_shot_video(RecipeMultiShotVideoRequest::auto(
                "A five-shot product story",
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
        recipes
            .product_ad(RecipeProductAdRequest::new(
                vec![RecipeImage::new("https://example.com/product.png")],
                ProductAdRecipeVersion::V2026_07,
            ))
            .await
            .unwrap(),
        recipes
            .product_campaign_image(RecipeProductCampaignImageRequest::new(
                RecipeImage::new("https://example.com/product.png"),
                "Editorial campaign",
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
        recipes
            .product_swap(RecipeProductSwapRequest::new(
                vec![RecipeProductSwapImage::new(
                    "https://example.com/new-product.png",
                )],
                RecipeImage::new("https://example.com/original-product.png"),
                RecipeVideo::new("https://example.com/reference.mp4"),
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
        recipes
            .product_ugc(RecipeProductUgcRequest::new(
                RecipeImage::new("https://example.com/person.png"),
                RecipeImage::new("https://example.com/product.png"),
                RecipeVersion::V2026_06,
            ))
            .await
            .unwrap(),
    ];

    for (index, task) in tasks.into_iter().enumerate() {
        assert_eq!(
            task.id().to_string(),
            format!("20000000-0000-4000-8000-{index:012}")
        );
    }
}

#[tokio::test]
async fn avatar_and_voice_page_streams_follow_non_empty_cursors_only() {
    let server = MockServer::start().await;

    for resource in ["avatars", "voices"] {
        let route = format!("/v1/{resource}");
        Mock::given(method("GET"))
            .and(path(route.clone()))
            .and(query_param("limit", "1"))
            .and(query_param_is_missing("cursor"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [],
                "nextCursor": "cursor-2"
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(route))
            .and(query_param("limit", "1"))
            .and(query_param("cursor", "cursor-2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [],
                "nextCursor": ""
            })))
            .expect(1)
            .mount(&server)
            .await;
    }

    let client = RunwayClient::with_config(test_config(&server.uri())).unwrap();

    let avatar_pages = client.avatars().list_pages(CursorPageQuery::new().limit(1));
    futures::pin_mut!(avatar_pages);
    let mut avatar_page_count = 0;
    while let Some(page) = avatar_pages.next().await {
        page.unwrap();
        avatar_page_count += 1;
    }

    let voice_pages = client.voices().list_pages(CursorPageQuery::new().limit(1));
    futures::pin_mut!(voice_pages);
    let mut voice_page_count = 0;
    while let Some(page) = voice_pages.next().await {
        page.unwrap();
        voice_page_count += 1;
    }

    assert_eq!(avatar_page_count, 2);
    assert_eq!(voice_page_count, 2);
}

#[tokio::test]
async fn avatar_and_voice_lists_reject_zero_limit_before_transport() {
    let client = RunwayClient::with_config(test_config("http://127.0.0.1:9")).unwrap();

    let avatar_error = client
        .avatars()
        .list(CursorPageQuery::new().limit(0))
        .await
        .unwrap_err();
    let voice_error = client
        .voices()
        .list(CursorPageQuery::new().limit(0))
        .await
        .unwrap_err();

    assert!(matches!(avatar_error, RunwayError::Validation { .. }));
    assert!(matches!(voice_error, RunwayError::Validation { .. }));
}
