use runway_sdk::{CreateDocumentRequest, CursorPageQuery, RunwayClient, UpdateDocumentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let docs = client.documents().list(CursorPageQuery::new()).await?;
    println!("Documents on this page: {}", docs.data.len());
    for doc in &docs.data {
        println!("  - {} ({})", doc.name, doc.id);
    }

    let new_doc = client
        .documents()
        .create(CreateDocumentRequest::new(
            "Shot List",
            "Scene 1: Wide establishing shot\nScene 2: Close-up dialogue",
        ))
        .await?;
    println!("Created document: {} ({})", new_doc.name, new_doc.id);

    client
        .documents()
        .update(
            &new_doc.id,
            UpdateDocumentRequest::new().name("Shot List v2"),
        )
        .await?;
    println!("Updated document {}", new_doc.id);

    client.documents().delete(&new_doc.id).await?;
    println!("Deleted document {}", new_doc.id);

    Ok(())
}
