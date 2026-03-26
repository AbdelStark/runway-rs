use runway_sdk::{RunWorkflowRequest, RunwayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // List workflows
    let workflows = client.workflows().list().await?;
    println!("Available workflows: {}", workflows.workflows.len());

    for wf in &workflows.workflows {
        println!("  - {} ({})", wf.name, wf.id);
    }

    // Run a workflow (if any exist)
    if let Some(wf) = workflows.workflows.first() {
        let run = client
            .workflows()
            .run(
                &wf.id,
                RunWorkflowRequest::new().param("prompt", serde_json::json!("hello world")),
            )
            .await?;
        println!("Workflow run started: {}", run.id);

        // Check invocation status
        let invocation = client.workflow_invocations().get(&run.id).await?;
        println!("Invocation status: {:?}", invocation.status);
    }

    Ok(())
}
