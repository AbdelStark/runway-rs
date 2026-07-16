use runway_sdk::{RunwayClient, SoundEffectRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .sound_effect()
        .create(SoundEffectRequest::new("Thunder rolling over a distant city").duration(10.0))
        .await?
        .wait_for_output()
        .await?;

    if let Some(url) = task.output_urls().and_then(|urls| urls.first()) {
        println!("Audio URL: {url}");
    }
    Ok(())
}
