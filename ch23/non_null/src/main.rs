use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::NonNull,
};

struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let raw_ptr = alloc(layout) as *mut T;
            if raw_ptr.is_null() {
                panic!("Allocation failed");
            }

            raw_ptr.write(value);
            NonNull::new(raw_ptr).unwrap()
        };

        MyBox { ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

fn main() {
    let _my_boy = MyBox::new(42);
}
