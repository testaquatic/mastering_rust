use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = pair2.as_ref();
        let mut started = lock.lock().unwrap();
        let timeout_result = cvar.wait_timeout(started, Duration::from_secs(3)).unwrap();

        started = timeout_result.0;

        if *started {
            println!("작업 시작!")
        } else {
            println!("타임아웃 발생, 조건이 만족되지 않음");
        }
    });

    let producer = thread::spawn(move || {
        let (lock, cvar) = pair.as_ref();
        thread::sleep(Duration::from_secs(1));

        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    consumer.join().unwrap();
    producer.join().unwrap();
}
