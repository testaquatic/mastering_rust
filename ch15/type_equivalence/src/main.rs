fn foo(_x: &str, _y: &'static str) {}

fn main() {
    let s = "hello".to_string();
    let r = "world";

    foo(&s, r);
}
