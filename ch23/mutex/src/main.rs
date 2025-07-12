use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let l1 = lock1.clone();
    let l2 = lock2.clone();

    let handle1 = thread::spawn(move || {
        let _lock1 = l1.lock().unwrap();
        println!("Thread 1 acquired lock1");

        std::thread::sleep(std::time::Duration::from_millis(50));

        let _lock2 = l2.lock().unwrap();
        println!("Thread 1 acquired lock2");
    });

    let l1 = lock1.clone();
    let l2 = lock2.clone();

    let handle2 = thread::spawn(move || {
        let _lock2 = l2.lock().unwrap();
        println!("Thread 2 acquired lock2");

        std::thread::sleep(std::time::Duration::from_millis(50));

        let _lock1 = l1.lock().unwrap();
        println!("Thread 2 acquired lock1");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
