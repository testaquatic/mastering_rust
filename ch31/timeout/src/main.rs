use std::{pin::pin, time::Duration};

use futures::{FutureExt, select};
use tokio::time::{sleep, timeout};

async fn long_running_task() -> &'static str {
    sleep(Duration::from_secs(5)).await;
    "Task completed"
}

#[tokio::main]
async fn main() {
    let task_fut = long_running_task().fuse();
    let mut timeout_fut = pin!(timeout(Duration::from_secs(3), task_fut).fuse());

    select! {
        result = timeout_fut => match result {
            Ok(value) => println!("Success {value}"),
            Err(_) => println!("Timeout! Task canceled"),
        },
        complete => println!("All futures completed"),
    };
}
