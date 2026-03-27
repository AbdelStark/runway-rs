use runway_sdk::{MediaInput, RunwayClient, VoiceIsolationRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .voice_isolation()
        .create(VoiceIsolationRequest::new(MediaInput::from_url(
            "https://example.com/noisy_recording.wav",
        )))
        .await?
        .wait_for_output()
        .await?;

    println!("Isolated voice URL: {}", task.output.unwrap()[0]);
    Ok(())
}
