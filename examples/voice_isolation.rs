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

    println!("Isolated audio URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
