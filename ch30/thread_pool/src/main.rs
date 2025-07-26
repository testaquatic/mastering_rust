use std::{
    hint::spin_loop,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self},
    time::Duration,
};

use crossbeam_channel::{Sender, bounded};

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
    closed: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = bounded::<Box<dyn FnOnce() + Send + 'static>>(size);

        let receiver = Arc::new(receiver);
        let closed = Arc::new(AtomicBool::new(false));

        let workers = (0..size)
            .map(|_| {
                let reciever_clone = Arc::clone(&receiver);
                let closed_clone = Arc::clone(&closed);

                let handle = thread::spawn(move || {
                    while !closed_clone.load(Ordering::Relaxed) {
                        while let Ok(task) = reciever_clone.try_recv() {
                            task();
                        }
                    }
                });

                handle
            })
            .collect::<Vec<_>>();

        ThreadPool {
            workers,
            sender,
            closed,
        }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(task)).unwrap();
    }

    pub fn close(&self) {
        while !self.sender.is_empty() {
            spin_loop();
        }
        self.closed.store(true, Ordering::SeqCst);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.close();
        self.workers
            .drain(..)
            .for_each(|worker| worker.join().unwrap());
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    (0..8).for_each(|i| {
        pool.execute(move || {
            println!(
                "Task {} is running on thread: {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(500));
        })
    });

    println!("All tasks have been assigned");
}
