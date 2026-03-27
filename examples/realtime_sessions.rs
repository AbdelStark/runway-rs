use runway_sdk::{CreateRealtimeSessionRequest, RunwayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Create a realtime session
    let session = client
        .realtime_sessions()
        .create(CreateRealtimeSessionRequest::new().model("gen4.5"))
        .await?;

    println!("Session created: {}", session.id);
    println!("Status: {:?}", session.status);

    // Cancel the session when done
    client.realtime_sessions().cancel(&session.id).await?;
    println!("Session cancelled");

    Ok(())
}
