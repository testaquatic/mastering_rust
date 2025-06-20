use std::ffi::{CStr, c_char};

unsafe fn print_c_string(c_string: *const c_char) {
    if c_string.is_null() {
        println!("Null pointer provided!");
        return;
    }

    unsafe {
        let rust_string = CStr::from_ptr(c_string).to_string_lossy();
        println!("{}", rust_string);
    }
}

fn main() {
    let c_string = c"Hello, C String!".as_ptr();
    unsafe {
        print_c_string(c_string);
    }
}