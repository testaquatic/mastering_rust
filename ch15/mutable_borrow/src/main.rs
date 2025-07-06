//! 컴파일이 가능한 코드 같아서 작성해 봤다.
//! 오류없이 컴파일 가능하다.

fn main() {
    let mut x = "Hello, Rust!".to_string();
    let y = &mut x;

    y.push_str(" You're awesome!");
    println!("{y}");
    println!("{x}");
}
