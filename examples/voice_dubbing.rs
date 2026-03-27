use runway_sdk::{MediaInput, RunwayClient, VoiceDubbingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .voice_dubbing()
        .create(
            VoiceDubbingRequest::new(MediaInput::from_url(
                "https://example.com/english_speech.mp3",
            ))
            .target_language("es"),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Dubbed audio URL: {}", task.output.unwrap()[0]);
    Ok(())
}
