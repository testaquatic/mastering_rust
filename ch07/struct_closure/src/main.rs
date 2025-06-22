struct HoldCallable<F: Fn()> {
    callable: F,
}

fn main() {
    let holds_callable = HoldCallable {
        callable: || println!("Hello"),
    };

    (holds_callable.callable)();
}
