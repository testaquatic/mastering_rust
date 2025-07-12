fn returns_fuction() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x * 2)
}

fn main() {
    let double = returns_fuction();
    let result = double(10);
    println!("Result: {}", result);
    println!("{}", |l, r| -> i32 { l + r }(1, 2));
}
