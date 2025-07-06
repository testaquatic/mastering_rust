static mut SECRET: &'static str = "12345678";

fn main() {
    unsafe {
        SECRET = "87654321";
        println!("{}", SECRET);
    }
}
