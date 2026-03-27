use runway_sdk::{
    AvatarVoiceInput, CreateAvatarRequest, CursorPageQuery, RunwayClient, UpdateAvatarRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let avatar = client
        .avatars()
        .create(CreateAvatarRequest::new(
            "My Avatar",
            "Helpful and concise",
            "https://example.com/avatar.png",
            AvatarVoiceInput::runway_live_preset("maya"),
        ))
        .await?;
    println!("Created avatar: {} ({})", avatar.name(), avatar.id());

    let list = client.avatars().list(CursorPageQuery::new()).await?;
    println!("Total avatars on this page: {}", list.data.len());

    let updated = client
        .avatars()
        .update(
            avatar.id(),
            UpdateAvatarRequest::new().name("Renamed Avatar"),
        )
        .await?;
    println!("Updated avatar name: {}", updated.name());

    client.avatars().delete(avatar.id()).await?;
    println!("Deleted avatar {}", avatar.id());

    Ok(())
}
