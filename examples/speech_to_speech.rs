use runway_sdk::{MediaInput, RunwayClient, SpeechToSpeechRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .speech_to_speech()
        .create(
            SpeechToSpeechRequest::new(MediaInput::from_url(
                "https://example.com/input-speech.mp3",
            ))
            .voice_id("voice-456"),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Converted speech URL: {}", task.output.unwrap()[0]);
    Ok(())
}
