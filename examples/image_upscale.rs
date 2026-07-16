use runway_sdk::{
    ImageUpscaleCreateRequest, ImageUpscaleFlavor, ImageUpscaleScaleFactor, RunwayClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .image_upscale()
        .create(
            ImageUpscaleCreateRequest::new("https://example.com/low_res_photo.jpg")
                .flavor(ImageUpscaleFlavor::Photo)
                .scale_factor(ImageUpscaleScaleFactor::X4)
                .ultra_detail(65.0),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Upscaled image URL: {url}");
    }
    Ok(())
}
