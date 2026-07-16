use runway_sdk::{RunwayClient, VideoRatio, VideoToVideoReference, VideoToVideoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .video_to_video()
        .create(
            VideoToVideoRequest::new(
                "Transform the clip into a watercolor painting style",
                "https://example.com/input-video.mp4",
            )
            .ratio(VideoRatio::Landscape)
            .references(vec![VideoToVideoReference::image(
                "https://example.com/style-reference.png",
            )]),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Video URL: {url}");
    }
    Ok(())
}
