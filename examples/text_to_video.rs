use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_video()
        .create(TextToVideoGen45Request::new(
            "A serene mountain landscape at sunrise with mist rolling through the valleys",
            VideoRatio::Landscape,
            5,
        ))
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
