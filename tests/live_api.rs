#![cfg(feature = "live-tests")]

use std::path::PathBuf;
use std::sync::Once;
use std::time::Duration;

use runway_sdk::{
    AvatarListQuery, CreateEphemeralUploadRequest, DocumentListQuery, RequestOptions, RunwayClient,
    RunwayError, RunwayPresetVoice, RunwayPresetVoiceId, TextToSpeechRequest, UsageQueryRequest,
    VoiceListQuery, WaitOptions,
};
use uuid::Uuid;

static LOAD_ENV: Once = Once::new();

const ONE_BY_ONE_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 156, 99, 248, 255, 255, 63, 0, 5,
    254, 2, 254, 167, 53, 129, 132, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];

fn load_env() {
    LOAD_ENV.call_once(|| {
        let _ = dotenvy::from_filename(".env");
    });
}

fn live_client() -> Result<RunwayClient, RunwayError> {
    load_env();
    RunwayClient::new()
}

fn temp_png_path() -> PathBuf {
    std::env::temp_dir().join(format!("runway-live-{}.png", Uuid::new_v4()))
}

#[tokio::test]
async fn live_organization_round_trip_and_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;

    let response = client
        .organization()
        .retrieve_with_options(
            RequestOptions::new()
                .timeout(Duration::from_secs(30))
                .idempotency_key("live-org-retrieve"),
        )
        .await?;

    assert_eq!(response.response.status, 200);
    assert!(
        response.response.headers.contains_key("date"),
        "expected date header in organization response"
    );
    assert!(response.data.credit_balance >= 0.0);

    Ok(())
}

#[tokio::test]
async fn live_usage_endpoint_returns_data_or_empty_report() -> Result<(), Box<dyn std::error::Error>>
{
    let client = live_client()?;

    let response = client
        .organization()
        .retrieve_usage_with_options(
            UsageQueryRequest::new()
                .start_date("2026-01-01")
                .before_date("2026-03-27"),
            RequestOptions::new().timeout(Duration::from_secs(30)),
        )
        .await?;

    assert_eq!(response.response.status, 200);
    assert!(
        !response.data.models.is_empty() || !response.data.results.is_empty(),
        "usage endpoint returned no models and no results"
    );

    Ok(())
}

#[tokio::test]
async fn live_management_lists_and_optional_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;

    let avatars = client
        .avatars()
        .list(AvatarListQuery::new().limit(1))
        .await?;
    if let Some(avatar) = avatars.items().first() {
        let retrieved = client.avatars().retrieve(avatar.id()).await?;
        assert_eq!(retrieved.id(), avatar.id());
    }

    let documents = client
        .documents()
        .list(DocumentListQuery::new().limit(1))
        .await?;
    if let Some(document) = documents.items().first() {
        let retrieved = client.documents().retrieve(&document.id).await?;
        assert_eq!(retrieved.id, document.id);
    }

    let voices = client.voices().list(VoiceListQuery::new().limit(1)).await?;
    if let Some(voice) = voices.items().first() {
        let retrieved = client.voices().retrieve(voice.id()).await?;
        assert_eq!(retrieved.id(), voice.id());
    }

    Ok(())
}

#[tokio::test]
async fn live_workflows_list_and_optional_retrieve() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;

    let response = client
        .workflows()
        .list_with_options(RequestOptions::new().timeout(Duration::from_secs(30)))
        .await?;

    assert_eq!(response.response.status, 200);

    if let Some(version) = response
        .data
        .data
        .iter()
        .flat_map(|group| group.versions.iter())
        .next()
    {
        let workflow = client.workflows().retrieve(&version.id).await?;
        assert_eq!(workflow.id, version.id);
    }

    Ok(())
}

#[tokio::test]
async fn live_upload_file_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;
    let path = temp_png_path();

    tokio::fs::write(&path, ONE_BY_ONE_PNG).await?;

    let upload_result = client.uploads().upload_file(&path).await;

    tokio::fs::remove_file(&path).await?;

    match upload_result {
        Ok(uri) => {
            assert!(
                uri.starts_with("runway://"),
                "expected runway:// URI from upload, got {uri}"
            );
        }
        Err(RunwayError::Api {
            status, message, ..
        }) if status == 403 && message.contains("credit purchase") => {
            println!("Ephemeral uploads disabled for this account until a credit purchase is made");
        }
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

#[tokio::test]
async fn live_billable_text_to_speech_or_insufficient_credits(
) -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;
    let organization = client.organization().retrieve().await?;

    let request = TextToSpeechRequest::new(
        format!("Live SDK verification {}", Uuid::new_v4()),
        RunwayPresetVoice::new(RunwayPresetVoiceId::Maya),
    );

    match client.text_to_speech().create(request).await {
        Ok(pending) => {
            let task = pending
                .wait_with_options(
                    WaitOptions::new()
                        .poll_interval(Duration::from_secs(3))
                        .timeout(Duration::from_secs(180)),
                )
                .await?;

            let outputs = task.output_urls().unwrap_or(&[]);
            assert!(!outputs.is_empty(), "expected text-to-speech output URL");

            let retrieved = client.tasks().retrieve(task.id()).await?;
            assert!(
                retrieved.is_succeeded(),
                "expected retrieved task to succeed"
            );
        }
        Err(RunwayError::Api {
            status, message, ..
        }) if status == 400 && message.contains("enough credits") => {
            println!(
                "Billable task submission blocked by account credits; credit_balance={}",
                organization.credit_balance
            );
            assert!(organization.credit_balance <= 0.0);
        }
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

#[tokio::test]
async fn live_raw_upload_flow_with_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client()?;

    let upload_result = client
        .uploads()
        .create_ephemeral_with_options(
            CreateEphemeralUploadRequest::new("live-api-check.png", ONE_BY_ONE_PNG.to_vec())
                .content_type("image/png")
                .file_metadata("{\"purpose\":\"sdk-live-test\"}"),
            RequestOptions::new().timeout(Duration::from_secs(60)),
        )
        .await;

    match upload_result {
        Ok(response) if response.response.status == 200 => {
            assert!(
                response.data.uri.starts_with("runway://"),
                "expected runway:// URI from raw upload flow, got {}",
                response.data.uri
            );
        }
        Err(RunwayError::Api {
            status, message, ..
        }) if status == 403 && message.contains("credit purchase") => {
            println!("Raw ephemeral upload flow is disabled for this account until a credit purchase is made");
        }
        Err(err) => return Err(err.into()),
        Ok(response) => {
            return Err(format!("unexpected upload status {}", response.response.status).into());
        }
    }

    Ok(())
}
