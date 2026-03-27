use runway_sdk::{RunwayClient, TextToVideoRequest, VideoModel, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .text_to_video()
        .create(
            TextToVideoRequest::new(
                VideoModel::Gen45,
                "A serene mountain landscape at sunrise with mist rolling through the valleys",
            )
            .ratio(VideoRatio::Landscape)
            .duration(5),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Video URL: {}", task.output.unwrap()[0]);
    Ok(())
}
