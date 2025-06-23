fn main() {
    let nums = vec![7, 8, 9];
    match nums.as_slice() {
        [_first @ 1..=3, rest @ ..] => println!("{:?}", rest),
        [single] if single == &5 || single == &6 => (),
        [_, _] => (),
        s => println!("one element, or 2+ elements {:?}", s),
    }
}
