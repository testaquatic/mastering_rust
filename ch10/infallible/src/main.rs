use std::convert::Infallible;

struct AlwaysSuccess<T> {
    value: T,
}

impl<T> AlwaysSuccess<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn get_value(self) -> Result<T, Infallible> {
        Ok(self.value)
    }
}

fn main() {
    let success = AlwaysSuccess::new(42);
    let result = success.get_value();
    match result {
        Ok(value) => println!("Success: {}", value),
    }
}
