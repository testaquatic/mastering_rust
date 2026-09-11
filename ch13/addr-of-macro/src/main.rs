use std::ptr::addr_of;

#[repr(C)]
struct Data {
    a: i32,
    b: i32,
}

fn main() {
    let data = Data { a: 10, b: 20 };
    let b_ptr = addr_of!(data.b);

    unsafe {
        println!("b: {}", *b_ptr);
    }
}
