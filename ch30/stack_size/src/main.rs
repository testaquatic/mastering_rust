use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .stack_size(4 * 1024 * 1024)
        .name("CustomThread".into());
    let handle = builder
        .spawn(|| {
            println!("스레드 이름: {:?}", thread::current().name());
        })
        .unwrap();
    handle.join().unwrap();
}
