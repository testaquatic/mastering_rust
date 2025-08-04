use std::{
    sync::{Arc, Mutex, atomic::AtomicBool},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct SharedState {
    completed: AtomicBool,
    waker: Mutex<Option<Waker>>,
}

struct MyFuture {
    shared_state: Arc<SharedState>,
}

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self
            .shared_state
            .completed
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            return std::task::Poll::Ready("Future completed!");
        }

        let mut waker = self.shared_state.waker.lock().unwrap();
        if let Some(existing_waker) = waker.take() {
            existing_waker.wake();
        }

        *waker = Some(cx.waker().clone());

        std::task::Poll::Pending
    }
}

fn main() {
    let shared_state = Arc::new(SharedState {
        completed: AtomicBool::new(false),
        waker: Mutex::new(None),
    });

    let future = MyFuture {
        shared_state: shared_state.clone(),
    };

    let waker_thread = {
        let shared_state = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(2));
            shared_state
                .completed
                .store(true, std::sync::atomic::Ordering::SeqCst);

            if let Some(waker) = shared_state.waker.lock().unwrap().take() {
                waker.wake();
            }
        })
    };

    let mut future = Box::pin(future);
    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(msg) => {
                println!("{}", msg);
                break;
            }
            Poll::Pending => {
                println!("Future not ready, Waiting...");
                thread::sleep(Duration::from_millis(500));
            }
        }
    }

    waker_thread.join().unwrap();
}
