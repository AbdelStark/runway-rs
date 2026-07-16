use runway_sdk::{
    RunwayClient, RunwayPresetVoice, RunwayPresetVoiceId, SpeechToSpeechMedia,
    SpeechToSpeechRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .speech_to_speech()
        .create(SpeechToSpeechRequest::new(
            SpeechToSpeechMedia::audio("https://example.com/input-speech.mp3"),
            RunwayPresetVoice::new(RunwayPresetVoiceId::Maya),
        ))
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Converted audio URL: {url}");
    }
    Ok(())
}
