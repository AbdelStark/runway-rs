use runway_sdk::{RunwayClient, RunwayPresetVoice, RunwayPresetVoiceId, TextToSpeechRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_speech()
        .create(TextToSpeechRequest::new(
            "Welcome to the future of AI-powered media generation.",
            RunwayPresetVoice::new(RunwayPresetVoiceId::Maya),
        ))
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Audio URL: {url}");
    }
    Ok(())
}
