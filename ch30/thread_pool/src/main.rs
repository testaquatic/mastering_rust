use std::{sync::Arc, thread, time::Duration};

use crossbeam::channel::{Sender, bounded, never, unbounded};

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = bounded::<Box<dyn FnOnce() + Send>>(size);

        let receiver = Arc::new(receiver);
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let receiver_clone = Arc::clone(&receiver);
            let handle = thread::spawn(move || {
                while let Ok(task) = receiver_clone.recv() {
                    task();
                }
            });

            workers.push(handle);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(task)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let (empty_sender, _) = bounded(0);
        let sender = std::mem::replace(&mut self.sender, empty_sender);
        drop(sender);

        for worker in self.workers.drain(..) {
            worker.join().unwrap()
        }
    }
}
fn main() {
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!(
                "Task {} is running on thread {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(Duration::from_secs(1));
        });
    }
}
