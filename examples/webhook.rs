use runway_sdk::{RequestOptions, RunwayClient, TextToVideoGen45Request, VideoRatio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let request =
        TextToVideoGen45Request::new("A serene mountain at sunrise", VideoRatio::Landscape, 5);

    let pending = client
        .text_to_video()
        .create_with_options(
            request,
            RequestOptions::new().idempotency_key("example-request-1"),
        )
        .await?
        .data;

    println!("Task submitted: {}", pending.id());
    Ok(())
}
