use std::time::Duration;

use futures::stream::{AbortHandle, Abortable};
use tokio::time::{Instant, sleep};

#[tokio::main]
async fn main() {
    let (abortable_task, abort_handle) = create_abortable_task();

    let timeout_task = async {
        sleep(Duration::from_secs(2)).await;
        println!("Timeout 작업 완료");
    };

    tokio::select! {
        _ = abortable_task => println!("Abortable 작업 완료"),
        _ = timeout_task => {
            println!("Timeout 작업 선택됨");
            abort_handle.abort();
        }
    }
}

fn create_abortable_task() -> (
    Abortable<impl std::future::Future<Output = ()>>,
    AbortHandle,
) {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();

    let task = async {
        let start = Instant::now();

        while start.elapsed() < Duration::from_secs(10) {
            sleep(Duration::from_secs(1)).await;
            println!("긴 작업 진행 중...");
        }

        println!("긴 작업 완료");
    };

    (Abortable::new(task, abort_registration), abort_handle)
}
