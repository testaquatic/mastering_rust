use std::ptr::addr_of;

fn main() {
    let x = 42;
    let p = addr_of!(x);

    unsafe {
        println!("Pointer address: {p:p}");
        println!("Pointer value: {}", *p);
    }
}
