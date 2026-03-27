use runway_sdk::{CreateAvatarRequest, RunwayClient, UpdateAvatarRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Create an avatar
    let avatar = client
        .avatars()
        .create(CreateAvatarRequest::new("My Avatar").description("A test avatar"))
        .await?;
    println!("Created avatar: {} ({})", avatar.name, avatar.id);

    // List all avatars
    let list = client.avatars().list().await?;
    println!("Total avatars: {}", list.avatars.len());

    // Update the avatar
    let updated = client
        .avatars()
        .update(
            &avatar.id,
            UpdateAvatarRequest::new().name("Renamed Avatar"),
        )
        .await?;
    println!("Updated avatar name: {}", updated.name);

    // Delete the avatar
    client.avatars().delete(&avatar.id).await?;
    println!("Avatar deleted");

    Ok(())
}
