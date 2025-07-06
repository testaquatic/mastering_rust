use std::any::{Any, TypeId};

fn main() {
    let x = 42;
    let y = 3.14;

    if TypeId::of::<i32>() == TypeId::of::<f64>() {
        println!("Both ar i32");
    }

    println!("x type_id: {:?}", x.type_id());
    println!("y type_id: {:?}", y.type_id());
}
