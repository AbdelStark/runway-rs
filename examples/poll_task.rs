use runway_sdk::{RunwayClient, TextToVideoGen45Request, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let pending = client
        .text_to_video()
        .create(TextToVideoGen45Request::new(
            "A neon-lit alley in the rain",
            VideoRatio::Landscape,
            5,
        ))
        .await?;

    println!("Task ID: {}", pending.id());

    let task = pending.wait_for_output().await?;
    println!("Output URL: {}", task.output_urls().unwrap()[0]);
    Ok(())
}
