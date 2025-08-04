use std::{
    hint::spin_loop,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Waker},
};

struct MyFuture {
    is_ready: AtomicBool,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_ready.load(Ordering::Relaxed) {
            Poll::Ready("Task Completed")
        } else {
            let mut waker = self.waker.lock().unwrap();
            *waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn complete_task(future_state: Arc<AtomicBool>, waker: Arc<Mutex<Option<Waker>>>) {
    while let Err(false) =
        future_state.compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::Relaxed)
    {
        spin_loop();
    }

    let waker = waker.lock().unwrap();
    if let Some(waker) = &*waker {
        waker.wake_by_ref();
    }
}

fn main() {
    println!("Hello, world!");
}
