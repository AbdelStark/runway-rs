use runway_sdk::{RunwayClient, SoundEffectRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let task = client
        .sound_effect()
        .create(
            SoundEffectRequest::new("thunder rumbling in the distance followed by heavy rain")
                .duration(10),
        )
        .await?
        .wait_for_output()
        .await?;

    println!("Sound effect URL: {}", task.output.unwrap()[0]);
    Ok(())
}
