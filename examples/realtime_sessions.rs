use runway_sdk::{CreateRealtimeSessionRequest, RealtimeAvatarInput, RunwayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let session = client
        .realtime_sessions()
        .create(CreateRealtimeSessionRequest::new(
            RealtimeAvatarInput::custom("avatar_123"),
        ))
        .await?;

    println!("Session created: {}", session.id);

    let current = client.realtime_sessions().retrieve(&session.id).await?;
    println!("Session status: {:?}", current.status());

    client.realtime_sessions().cancel(&session.id).await?;
    Ok(())
}
