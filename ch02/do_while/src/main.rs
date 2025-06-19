//! 사용할 일이 많은 코드는 아닌 것 같지만 왜 이런 방법을 그동안 생각하지 못 했나 싶다.

fn main() {
    let mut count = 0;
    while {
        println!("Count: {}", count);
        count += 1;
        count < 10
    } { /* EMPTY */ }
}
