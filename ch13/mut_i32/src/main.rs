fn main() {
    let mut value = 42;
    let ptr: *mut i32 = &mut value;

    unsafe {
        *ptr = 50;
    }

    println!("Updated value: {value}");
}
