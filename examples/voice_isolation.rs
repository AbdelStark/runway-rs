use runway_sdk::{RunwayClient, VoiceIsolationRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .voice_isolation()
        .create(VoiceIsolationRequest::new(
            "https://example.com/noisy-recording.wav",
        ))
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Isolated audio URL: {url}");
    }
    Ok(())
}
