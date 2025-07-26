use std::{thread, time::Duration};

use crossbeam::scope;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;

    sum += scope(|s| {
        let sum1 = s.spawn(|_| {
            println!("첫 번째 스레드 시작");
            let partial_sum = numbers[..3].iter().sum::<i32>();
            thread::sleep(Duration::from_secs(1));
            println!("첫 번째 스레드에서 부분 합계: {partial_sum}");
            partial_sum
        });

        let sum2 = s.spawn(|_| {
            println!("두 번째 스레드 시작");
            let partial_sum = numbers[3..].iter().sum::<i32>();
            thread::sleep(Duration::from_secs(1));
            println!("두 번째 스레드에서 부분 합계: {partial_sum}");
            partial_sum
        });

        let inter_sum = sum1.join().unwrap() + sum2.join().unwrap();
        println!("내부 합계 {inter_sum}");
        inter_sum
    })
    .unwrap();

    println!("전체 합계: {sum}");
}
