use runway_sdk::{RunwayClient, TaskListQuery, TaskStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RunwayClient::new()?;

    // List all tasks
    let all_tasks = client.tasks().list(TaskListQuery::new()).await?;
    println!("Total tasks: {}", all_tasks.tasks.len());

    // List only running tasks with pagination
    let running = client
        .tasks()
        .list(
            TaskListQuery::new()
                .status(TaskStatus::Running)
                .limit(10)
                .offset(0),
        )
        .await?;

    for task in &running.tasks {
        println!(
            "Task {} — {:?} (progress: {:?})",
            task.id(),
            task.status(),
            task.progress()
        );
    }

    if running.has_more == Some(true) {
        println!("More tasks available...");
    }

    Ok(())
}
