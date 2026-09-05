struct HoldsCallable<F: Fn()> {
    callable: F,
}

fn main() {
    let holds_callable = HoldsCallable {
        callable: || println!("Hello"),
    };

    // holds_callable.callable();
    (holds_callable.callable)();
}
