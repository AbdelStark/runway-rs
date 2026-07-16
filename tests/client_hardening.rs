use std::time::Duration;

use runway_sdk::{
    Config, CreateEphemeralUploadRequest, RequestOptions, RunwayClient, RunwayError,
    TextToVideoGen45Request, VideoRatio, WaitOptions,
};
use tokio_util::sync::CancellationToken;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_client(server: &MockServer) -> RunwayClient {
    RunwayClient::with_config(
        Config::new("test-key")
            .base_url(server.uri())
            .max_retries(2),
    )
    .unwrap()
}

#[test]
fn client_rejects_invalid_configuration_at_construction() {
    assert!(matches!(
        RunwayClient::with_api_key("   "),
        Err(RunwayError::MissingApiKey)
    ));
    assert!(matches!(
        RunwayClient::with_config(Config::new("key").base_url("ftp://example.com")),
        Err(RunwayError::Validation { .. })
    ));
    assert!(matches!(
        RunwayClient::with_config(Config::new("key").base_url("https://user:pass@example.com")),
        Err(RunwayError::Validation { .. })
    ));
    assert!(matches!(
        RunwayClient::with_config(Config::new("key").timeout(Duration::ZERO)),
        Err(RunwayError::Validation { .. })
    ));
}

#[test]
fn client_defaults_reject_one_shot_idempotency_and_cancellation_controls() {
    let client = RunwayClient::with_api_key("test-key").unwrap();
    assert!(matches!(
        client.with_options(RequestOptions::new().idempotency_key("reused-key")),
        Err(RunwayError::Validation { .. })
    ));
    assert!(matches!(
        client.with_options(RequestOptions::new().cancellation_token(CancellationToken::new())),
        Err(RunwayError::Validation { .. })
    ));
    assert!(matches!(
        Config::new("test-key").default_header("Idempotency-Key", "reused-key"),
        Err(RunwayError::Validation { .. })
    ));
    assert!(matches!(
        RequestOptions::new().header("Idempotency-Key", "manual-key"),
        Err(RunwayError::Validation { .. })
    ));
}

#[test]
fn debug_output_redacts_headers_queries_urls_and_idempotency_keys() {
    let options = RequestOptions::new()
        .header("Authorization", "Bearer secret-token")
        .unwrap()
        .header("X-Customer-Secret", "custom-header-secret")
        .unwrap()
        .query_param("token", "query-secret")
        .base_url("https://user:password@example.com/v1?token=url-secret")
        .idempotency_key("idempotency-secret");
    let output = format!("{options:?}");

    for secret in [
        "secret-token",
        "custom-header-secret",
        "query-secret",
        "password",
        "url-secret",
        "idempotency-secret",
    ] {
        assert!(!output.contains(secret), "Debug output leaked {secret}");
    }
}

#[tokio::test]
async fn malformed_idempotency_key_is_rejected_before_network_io() {
    let server = MockServer::start().await;
    let client = test_client(&server);
    let result = client
        .organization()
        .retrieve_with_options(RequestOptions::new().idempotency_key("bad\nkey"))
        .await;

    assert!(matches!(result, Err(RunwayError::Validation { .. })));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn billable_post_is_not_retried_without_explicit_safety() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .respond_with(
            ResponseTemplate::new(500)
                .append_header("retry-after-ms", "1")
                .append_header("x-should-retry", "true")
                .set_body_json(serde_json::json!({"error": {"message": "try again"}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let result = test_client(&server)
        .text_to_video()
        .create(TextToVideoGen45Request::new(
            "A safe retry test",
            VideoRatio::Landscape,
            5,
        ))
        .await;

    assert!(matches!(result, Err(RunwayError::Api { status: 500, .. })));
}

#[tokio::test]
async fn idempotency_key_allows_configured_post_retries() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .and(header("Idempotency-Key", "generation-123"))
        .respond_with(
            ResponseTemplate::new(500)
                .append_header("retry-after-ms", "1")
                .set_body_json(serde_json::json!({"error": {"message": "try again"}})),
        )
        .expect(3)
        .mount(&server)
        .await;

    let result = test_client(&server)
        .text_to_video()
        .create_with_options(
            TextToVideoGen45Request::new("A safe retry test", VideoRatio::Landscape, 5),
            RequestOptions::new().idempotency_key("generation-123"),
        )
        .await;

    assert!(matches!(result, Err(RunwayError::Api { status: 500, .. })));
}

#[tokio::test]
async fn response_decode_error_preserves_status_and_redacts_excerpt_in_debug() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_string("secret malformed body"))
        .mount(&server)
        .await;

    let error = test_client(&server)
        .organization()
        .retrieve()
        .await
        .unwrap_err();
    match &error {
        RunwayError::ResponseDecode {
            status,
            body_excerpt,
            ..
        } => {
            assert_eq!(*status, 200);
            assert_eq!(body_excerpt.expose(), "secret malformed body");
        }
        other => panic!("unexpected error: {other}"),
    }
    assert!(!format!("{error:?}").contains("secret malformed body"));
}

#[tokio::test]
async fn unauthorized_error_retains_server_context() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(
            ResponseTemplate::new(401)
                .append_header("x-request-id", "request-123")
                .append_header("x-sensitive", "response-header-secret")
                .set_body_json(serde_json::json!({
                    "error": {"message": "invalid credential", "code": "invalid_api_key"}
                })),
        )
        .mount(&server)
        .await;

    let error = test_client(&server)
        .organization()
        .retrieve()
        .await
        .unwrap_err();
    assert!(!format!("{error:?}").contains("response-header-secret"));
    match error {
        RunwayError::Unauthorized {
            message,
            code,
            headers,
        } => {
            assert_eq!(message, "invalid credential");
            assert_eq!(code.as_deref(), Some("invalid_api_key"));
            assert_eq!(headers["x-request-id"], "request-123");
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[tokio::test]
async fn oversized_retry_after_headers_cannot_panic_the_client() {
    for header_name in ["retry-after-ms", "retry-after"] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/organization"))
            .respond_with(
                ResponseTemplate::new(429)
                    .append_header(header_name, "1e308")
                    .set_body_json(serde_json::json!({
                        "error": {"message": "slow down", "code": "rate_limited"}
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
        let error = client.organization().retrieve().await.unwrap_err();

        match error {
            RunwayError::RateLimited { retry_after, .. } => assert_eq!(retry_after, None),
            other => panic!("unexpected error for {header_name}: {other}"),
        }
    }
}

#[tokio::test]
async fn transport_errors_strip_request_and_presigned_storage_urls() {
    let query_secret = "query-secret-that-must-not-leak";
    let client = RunwayClient::with_config(
        Config::new("test-key")
            .base_url("http://127.0.0.1:1")
            .max_retries(0)
            .timeout(Duration::from_secs(1)),
    )
    .unwrap();
    let query_error = client
        .organization()
        .retrieve_with_options(RequestOptions::new().query_param("token", query_secret))
        .await
        .unwrap_err();
    for rendered in [format!("{query_error}"), format!("{query_error:?}")] {
        assert!(!rendered.contains(query_secret));
        assert!(!rendered.contains("token="));
    }

    let server = MockServer::start().await;
    let signature = "presigned-storage-secret-that-must-not-leak";
    Mock::given(method("POST"))
        .and(path("/v1/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "runwayUri": "runway://uploads/example",
            "uploadUrl": format!("http://127.0.0.1:1/storage?signature={signature}"),
            "fields": {}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let upload_error = test_client(&server)
        .uploads()
        .create_ephemeral(CreateEphemeralUploadRequest::new("safe.png", [0_u8; 4]))
        .await
        .unwrap_err();
    for rendered in [format!("{upload_error}"), format!("{upload_error:?}")] {
        assert!(!rendered.contains(signature));
        assert!(!rendered.contains("signature="));
    }
}

#[tokio::test]
async fn pending_task_preserves_submission_routing_headers_and_query() {
    let origin = MockServer::start().await;
    let override_server = MockServer::start().await;
    let task_id = "550e8400-e29b-41d4-a716-446655440000";

    Mock::given(method("POST"))
        .and(path("/v1/text_to_video"))
        .and(header("X-Tenant", "tenant-7"))
        .and(wiremock::matchers::query_param("gateway", "private"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": task_id
        })))
        .expect(1)
        .mount(&override_server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/tasks/{task_id}")))
        .and(header("X-Tenant", "tenant-7"))
        .and(wiremock::matchers::query_param("gateway", "private"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": task_id,
            "status": "SUCCEEDED",
            "createdAt": "2026-07-16T00:00:00Z",
            "output": ["https://example.com/output.mp4"]
        })))
        .expect(1)
        .mount(&override_server)
        .await;

    let client = test_client(&origin);
    let options = RequestOptions::new()
        .header("X-Tenant", "tenant-7")
        .unwrap()
        .query_param("gateway", "private")
        .base_url(override_server.uri());
    let task = client
        .text_to_video()
        .create_with_options(
            TextToVideoGen45Request::new("Continuation context", VideoRatio::Landscape, 5),
            options,
        )
        .await
        .unwrap()
        .data
        .wait_with_options(
            WaitOptions::new()
                .poll_interval(Duration::from_millis(1))
                .timeout(Duration::from_secs(1)),
        )
        .await
        .unwrap();

    assert!(task.is_succeeded());
    assert!(origin.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn cancellation_aborts_an_in_flight_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(5))
                .set_body_json(serde_json::json!({
                    "creditBalance": 0,
                    "tier": "standard",
                    "usage": {}
                })),
        )
        .mount(&server)
        .await;

    let token = CancellationToken::new();
    let cancel = token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(25)).await;
        cancel.cancel();
    });

    let result = test_client(&server)
        .organization()
        .retrieve_with_options(RequestOptions::new().cancellation_token(token))
        .await;
    assert!(matches!(result, Err(RunwayError::RequestAborted)));
}

#[tokio::test]
async fn client_sends_a_versioned_rust_user_agent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .and(header(
            "User-Agent",
            concat!("runway-sdk-rust/", env!("CARGO_PKG_VERSION")),
        ))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;

    let _ = test_client(&server).organization().retrieve().await;
}
