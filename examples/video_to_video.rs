use runway_sdk::{MediaInput, RunwayClient, VideoModel, VideoToVideoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .video_to_video()
        .create(
            VideoToVideoRequest::new(
                VideoModel::Gen4Turbo,
                "Transform into a watercolor painting style",
                MediaInput::from_url("https://example.com/input-video.mp4"),
            )
            .duration(5),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Video: {}", task.output.unwrap()[0]);
    Ok(())
}
