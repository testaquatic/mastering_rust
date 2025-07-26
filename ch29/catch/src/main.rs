use std::panic;

fn main() {
    #[allow(unreachable_code)]
    let result = panic::catch_unwind(|| {
        println!("Before panic");
        panic!("Something went wrong!");
        println!("This will not run");
    });

    match result {
        Ok(_) => println!("Code execute without panic."),
        Err(e) => {
            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("Panic message: {msg}");
            }
        }
    }

    println!("Program continues after panic.");
}
