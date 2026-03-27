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

    println!("Converted audio URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
