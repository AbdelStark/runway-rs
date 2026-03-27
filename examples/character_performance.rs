use runway_sdk::{CharacterPerformanceRequest, MediaInput, RunwayClient, VideoModel, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .character_performance()
        .create(
            CharacterPerformanceRequest::new(
                VideoModel::Gen4Turbo,
                "Wave hello and smile warmly at the camera",
                MediaInput::from_url("https://example.com/character_face.jpg"),
                MediaInput::from_url("https://example.com/motion_reference.mp4"),
            )
            .ratio(VideoRatio::Portrait)
            .duration(5),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Performance video URL: {}", task.output.unwrap()[0]);
    Ok(())
}
