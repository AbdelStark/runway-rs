use runway_sdk::{
    CharacterPerformanceCharacter, CharacterPerformanceReference, CharacterPerformanceRequest,
    RunwayClient, VideoRatio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .character_performance()
        .create(
            CharacterPerformanceRequest::new(
                CharacterPerformanceCharacter::image("https://example.com/character-face.jpg"),
                CharacterPerformanceReference::video("https://example.com/motion-reference.mp4"),
            )
            .ratio(VideoRatio::Portrait)
            .expression_intensity(4),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Performance video URL: {url}");
    }
    Ok(())
}
