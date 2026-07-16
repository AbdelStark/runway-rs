use runway_sdk::{LipSyncRequest, MediaInput, RunwayClient, VideoModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .lip_sync()
        .create(
            LipSyncRequest::new(
                VideoModel::Gen45,
                MediaInput::from_url("https://example.com/talking_head.mp4"),
                MediaInput::from_url("https://example.com/new_dialogue.wav"),
            )
            .max_duration(30),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Lip-synced video URL: {url}");
    }
    Ok(())
}
