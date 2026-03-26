use futures::StreamExt;
use runway_sdk::{RunwayClient, TextToVideoRequest, VideoModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let pending = client
        .text_to_video()
        .create(TextToVideoRequest::new(
            VideoModel::Gen45,
            "A cat playing piano in a jazz club",
        ))
        .await?;

    println!("Task ID: {}", pending.id());

    // Stream status updates
    let mut stream = std::pin::pin!(pending.stream_status());
    while let Some(result) = stream.next().await {
        let task = result?;
        println!(
            "Status: {:?}, Progress: {:?}",
            task.status, task.progress
        );

        if let Some(output) = task.output {
            println!("Output URLs:");
            for url in output {
                println!("  {}", url);
            }
        }
    }

    Ok(())
}
