use std::alloc::Layout;
use std::alloc::{self, GlobalAlloc, System};

struct MyAllocator;

impl MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        dbg!("Allocating {} bytes...", layout.size());
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        dbg!("Deallocating {} bytes...", layout.size());
        unsafe { alloc::dealloc(ptr, layout) }
    }
}

struct MyAllocatorGlobal;

unsafe impl GlobalAlloc for MyAllocatorGlobal {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        dbg!("Allocating {} bytes...", layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        dbg!("Deallocating {} bytes...", layout.size());
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: MyAllocatorGlobal = MyAllocatorGlobal;

fn main() {
    let allocator = MyAllocator;

    let layout = Layout::from_size_align(1024, 8).unwrap();

    unsafe {
        let ptr = allocator.alloc(layout);

        ptr.write_bytes(0, 1024);

        allocator.dealloc(ptr, layout);
    }

    let x = Box::new(42);
    println!("Allocated value: {}", x);
}
