use runway_sdk::{RunwayClient, TextToVideoRequest, VideoModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Create a generation task with a webhook URL.
    // Instead of polling, Runway will POST the completed task to your webhook
    // endpoint when the generation finishes.
    let pending = client
        .text_to_video()
        .create(
            TextToVideoRequest::new(VideoModel::Gen45, "A serene mountain at sunrise")
                .webhook_url("https://your-server.com/api/runway-webhook"),
        )
        .await?;

    println!("Task submitted: {}", pending.id());
    println!("Runway will POST results to your webhook URL when complete.");
    println!("No polling needed!");

    Ok(())
}
