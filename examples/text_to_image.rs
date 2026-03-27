use runway_sdk::{ImageModel, RunwayClient, TextToImageRequest, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_image()
        .create(
            TextToImageRequest::new(
                ImageModel::Gen4ImageTurbo,
                "A cyberpunk cityscape at night with neon lights reflecting on wet streets",
            )
            .ratio(VideoRatio::Wide)
            .seed(42),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Image URL: {}", task.output.unwrap()[0]);
    Ok(())
}
