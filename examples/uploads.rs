use std::path::Path;

use runway_sdk::{ImageToVideoRequest, MediaInput, RunwayClient, VideoModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Upload a local file and get a runway:// URI
    let runway_uri = client
        .uploads()
        .upload_file(Path::new("input-image.png"))
        .await?;

    println!("Uploaded file URI: {}", runway_uri);

    // Use the uploaded file in a generation request
    let task = client
        .image_to_video()
        .create(ImageToVideoRequest::new(
            VideoModel::Gen45,
            "Animate this image with gentle motion",
            MediaInput::from_runway_uri(runway_uri),
        ))
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output.unwrap()[0]);
    Ok(())
}
