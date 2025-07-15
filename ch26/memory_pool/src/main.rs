use std::{
    alloc::{self, Layout},
    ptr,
};

struct MemoryPool {
    pool: *mut u8,
    size: usize,
    used: usize,
}

impl MemoryPool {
    fn new(size: usize) -> MemoryPool {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let pool = unsafe { alloc::alloc(layout) };

        MemoryPool {
            pool,
            size,
            used: 0,
        }
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let required_size = layout.size();

        if self.used + required_size > self.size {
            return ptr::null_mut();
        }

        unsafe {
            let ptr = self.pool.add(self.used);
            self.used += required_size;

            ptr
        }
    }

    #[allow(dead_code)]
    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {}
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            alloc::dealloc(self.pool, layout);
        }
    }
}

fn main() {
    let mut pool = MemoryPool::new(1024);
    let layout = Layout::from_size_align(100, 8).unwrap();

    unsafe {
        let ptr = pool.alloc(layout);

        if !ptr.is_null() {
            ptr.write_bytes(1, 100);
        }
    }
}
