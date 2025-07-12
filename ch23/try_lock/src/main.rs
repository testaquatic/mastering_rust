use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut lock = data_clone.lock().unwrap();
        *lock += 10;
        println!("Thread: Data incremented to {}", *lock);
        thread::sleep(Duration::from_secs(2));
    });

    thread::sleep(Duration::from_millis(500));

    match data.try_lock() {
        Ok(mut lock) => {
            *lock += 1;
            println!("Main: Data incremented to {}", *lock);
        }
        Err(_) => println!("Main: Could not acquire lock, performing alternative action"),
    }

    handle.join().unwrap();
}
