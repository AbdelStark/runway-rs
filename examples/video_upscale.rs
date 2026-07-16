use runway_sdk::{
    RunwayClient, VideoUpscaleCreateRequest, VideoUpscaleFlavor, VideoUpscaleResolution,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .video_upscale()
        .create(
            VideoUpscaleCreateRequest::new("https://example.com/input.mp4")
                .flavor(VideoUpscaleFlavor::Natural)
                .resolution(VideoUpscaleResolution::K2)
                .fps_boost(true),
        )
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Upscaled video URL: {url}");
    }
    Ok(())
}
