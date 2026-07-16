use runway_sdk::{RunwayClient, VoiceDubbingLanguage, VoiceDubbingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .voice_dubbing()
        .create(VoiceDubbingRequest::new(
            "https://example.com/english-speech.mp3",
            VoiceDubbingLanguage::Es,
        ))
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Dubbed audio URL: {url}");
    }
    Ok(())
}
