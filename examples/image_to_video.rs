use runway_sdk::{ImageToVideoRequest, MediaInput, RunwayClient, VideoModel};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Upload a local file first
    let upload_uri = client
        .uploads()
        .upload_file(Path::new("./my-image.jpg"))
        .await?;

    let task = client
        .image_to_video()
        .create(
            ImageToVideoRequest::new(
                VideoModel::Gen4Turbo,
                "A gentle zoom into the scene with soft ambient lighting",
                MediaInput::from_runway_uri(upload_uri),
            )
            .duration(10),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Video: {}", task.output.unwrap()[0]);
    Ok(())
}
