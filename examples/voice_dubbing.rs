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

    println!("Dubbed audio URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
