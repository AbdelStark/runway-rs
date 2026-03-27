use runway_sdk::{RunwayClient, TextToSpeechRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_speech()
        .create(
            TextToSpeechRequest::new("Welcome to the future of AI-powered media generation.")
                .voice_id("voice-123"),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Speech audio URL: {}", task.output.unwrap()[0]);
    Ok(())
}
