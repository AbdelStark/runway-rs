use runway_sdk::{CreateDocumentRequest, RunwayClient, UpdateDocumentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // List existing documents
    let docs = client.documents().list().await?;
    println!("Documents: {}", docs.documents.len());
    for doc in &docs.documents {
        println!("  - {} ({})", doc.name, doc.id);
    }

    // Create a new document
    let new_doc = client
        .documents()
        .create(
            CreateDocumentRequest::new("Shot List")
                .content("Scene 1: Wide establishing shot\nScene 2: Close-up dialogue")
                .description("Production shot list for episode 1"),
        )
        .await?;
    println!("Created document: {} ({})", new_doc.name, new_doc.id);

    // Update the document
    let updated = client
        .documents()
        .update(
            &new_doc.id,
            UpdateDocumentRequest::new().name("Shot List v2"),
        )
        .await?;
    println!("Updated document: {}", updated.name);

    // Delete the document
    client.documents().delete(&new_doc.id).await?;
    println!("Deleted document: {}", new_doc.id);

    Ok(())
}
