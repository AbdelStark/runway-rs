use runway_sdk::{ImageModel, ImageUpscaleRequest, MediaInput, RunwayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .image_upscale()
        .create(
            ImageUpscaleRequest::new(
                ImageModel::Gen4ImageTurbo,
                MediaInput::from_url("https://example.com/low_res_photo.jpg"),
            )
            .resolution(4096),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Upscaled image URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
