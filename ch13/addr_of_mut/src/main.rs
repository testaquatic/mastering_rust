use std::ptr::addr_of_mut;

fn main() {
    let mut x = 43;
    let p = addr_of_mut!(x);

    unsafe {
        *p = 100;
        println!("x is now: {}", *p);
    }
}
