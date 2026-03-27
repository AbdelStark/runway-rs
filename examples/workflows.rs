use runway_sdk::{PrimitiveNodeValue, RunWorkflowRequest, RunwayClient, WorkflowNodeOutputValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    let workflows = client.workflows().list().await?;
    println!("Workflow groups: {}", workflows.data.len());

    if let Some(group) = workflows.data.first() {
        println!("Workflow family: {}", group.name);

        if let Some(version) = group.versions.first() {
            let invocation = client
                .workflows()
                .run_pending(
                    &version.id,
                    RunWorkflowRequest::new().node_output(
                        "prompt-node",
                        "prompt",
                        WorkflowNodeOutputValue::Primitive {
                            value: PrimitiveNodeValue::from("hello world"),
                        },
                    ),
                )
                .await?;

            println!("Workflow invocation started: {}", invocation.id());
        }
    }

    Ok(())
}
