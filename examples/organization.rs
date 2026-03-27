use runway_sdk::{RunwayClient, UsageQueryRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // Get organization info
    let org = client.organization().get().await?;
    println!("Organization: {} ({})", org.name, org.id);

    // Query usage for a date range
    let usage = client
        .organization()
        .usage(
            UsageQueryRequest::new()
                .start_date("2024-01-01")
                .end_date("2024-12-31"),
        )
        .await?;
    println!("Usage data: {:?}", usage.usage);

    Ok(())
}
