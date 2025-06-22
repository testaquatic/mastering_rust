#![allow(dead_code)]

enum Never {}

fn process(value: Never) {
    match value {}
}

fn main() {
    println!("Hi!");
}
