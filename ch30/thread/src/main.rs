use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        println!("스레드 대기 중...");
        thread::park();
        println!("스레드가 다시 실행됨!");
    });

    thread::sleep(Duration::from_secs(1));
    handle.thread().unpark();
    handle.join().unwrap();
}
