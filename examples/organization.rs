use runway_sdk::{RunwayClient, UsageQueryRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let org = client.organization().retrieve().await?;
    println!("Credit balance: {}", org.credit_balance);

    let usage = client
        .organization()
        .retrieve_usage(
            UsageQueryRequest::new()
                .start_date("2024-01-01")
                .before_date("2024-02-01"),
        )
        .await?;
    println!("Usage models: {}", usage.models.len());
    println!("Usage rows: {}", usage.results.len());

    Ok(())
}
