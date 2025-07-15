use std::alloc::{Layout, alloc, dealloc};

fn main() {
    let count = 10;
    let layout = Layout::array::<u32>(count).unwrap();
    let ptr = unsafe { alloc(layout) as *mut u32 };

    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }

    unsafe {
        for i in 0..count {
            ptr.add(i).write(i as u32);
        }

        for i in 0..count {
            println!("Value at index {}: {}", i, ptr.add(i).read());
        }

        dealloc(ptr as *mut u8, layout);
    }
}
