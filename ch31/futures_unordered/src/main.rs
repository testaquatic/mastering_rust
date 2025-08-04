use std::time::Duration;

use futures::{FutureExt, StreamExt, stream::FuturesUnordered};
use tokio::time::sleep;

async fn generate_task(id: usize) -> usize {
    sleep(Duration::from_secs(1)).await;
    println!("Task {id} completed");

    id
}

#[tokio::main]
async fn main() {
    let mut tasks = FuturesUnordered::new();
    let mut total_tasks = 3;
    let max_tasks = 10;

    for i in 1..=3 {
        tasks.push(generate_task(i).fuse());
    }

    while let Some(result) = tasks.next().await {
        println!("Processing result: {result}");

        if result % 2 == 0 && total_tasks < max_tasks {
            let new_task_id = result + 10;
            tasks.push(generate_task(new_task_id).fuse());
            total_tasks += 1;
        }
    }

    println!("All tasks completed");
}
