use runway_sdk::{
    ImageRatio, RunwayClient, TextToImageGen4ImageTurboRequest, TextToImageReferenceImage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_image()
        .create(TextToImageGen4ImageTurboRequest::new(
            "A cinematic cyberpunk city street at night",
            ImageRatio::Square1024,
            vec![TextToImageReferenceImage::new(
                "https://example.com/reference-image.png",
            )],
        ))
        .await?
        .wait_for_output()
        .await?;

    println!("Image URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
