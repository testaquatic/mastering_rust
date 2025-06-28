use std::alloc::{Layout, alloc, dealloc};

fn main() {
    let layout = Layout::new::<i32>();
    let ptr = unsafe { alloc(layout) as *mut i32 };

    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }

    unsafe {
        *ptr = 42;
        println!("Value at ptr: {}", *ptr);
        dealloc(ptr as *mut u8, layout);
    }
}
