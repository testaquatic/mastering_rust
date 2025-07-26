use std::panic;

fn main() {
    panic::set_hook(Box::new(|info| {
        println!("Custom panic handler:");

        if let Some(location) = info.location() {
            println!(
                "Panic occured in file '{}' at line {}",
                location.file(),
                location.line()
            );
        } else {
            println!("Panic occured but location is unknown.");
        }
    }));

    panic!("This is a panic!");
}
