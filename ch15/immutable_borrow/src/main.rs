fn main() {
    let x = String::from("Hello, Rust");
    let y = &x;

    println!("x: {}", x);
    println!("y: {}", y);

    let mut x = String::from("hello, Rust");
    let y = &mut x;

    y.push_str("You're awesome!");
    println!("y: {}", y);
    println!("x: {}", x);
}
