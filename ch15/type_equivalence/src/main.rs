fn bar<'a>(_x: &'a str, _y: &'static str) {}

fn main() {
    let s = String::from("hello");
    let r: &'static str = "world";

    bar(&s, r);
}
