use runway_sdk::{
    CreateEphemeralUploadRequest, ImageToVideoGen4TurboRequest, RunwayClient, VideoRatio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let upload = client
        .uploads()
        .create_ephemeral(
            CreateEphemeralUploadRequest::new("input.png", vec![1, 2, 3, 4])
                .content_type("image/png"),
        )
        .await?;

    let task = client
        .image_to_video()
        .create(
            ImageToVideoGen4TurboRequest::new(upload.uri, VideoRatio::Landscape)
                .prompt_text("Animate the uploaded image"),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Video URL: {url}");
    }
    Ok(())
}
