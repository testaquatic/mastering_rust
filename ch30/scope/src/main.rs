use std::thread;

fn main() {
    let mut data = vec![10, 20, 30];

    thread::scope(|s| {
        s.spawn(|| {
            for i in &mut data {
                *i += 1;
            }

            println!("첫 번째 스레드에서 변경된 데이터: {data:?}");
        });
    });

    println!("메인 스레드에서 변경된 데이터: {data:?}");
}
