use std::panic::{self, UnwindSafe};

#[allow(dead_code)]
struct MyType {
    value: i32,
}

impl std::panic::UnwindSafe for MyType {}

fn run_test<F: FnOnce() + UnwindSafe>(test: F) {
    let test_ = panic::AssertUnwindSafe(test);

    let result = panic::catch_unwind(test_);

    match result {
        Err(_) => println!("Test failed due to panic."),
        Ok(_) => println!("Test passed."),
    }
}

fn main() {
    #[allow(unused_variables)]
    let my_instance = MyType { value: 10 };

    run_test(|| {
        println!("Running test...");
        panic!("Intentional failure");
    });
}
