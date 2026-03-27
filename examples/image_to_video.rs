use runway_sdk::{ImageToVideoGen4TurboRequest, RunwayClient, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .image_to_video()
        .create(
            ImageToVideoGen4TurboRequest::new(
                "https://example.com/input-image.jpg",
                VideoRatio::Landscape,
            )
            .prompt_text("Animate the clouds and push the camera in slowly")
            .duration(10),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
