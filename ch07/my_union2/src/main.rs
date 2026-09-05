use std::mem::ManuallyDrop;

#[allow(dead_code)]
union MyUnion {
    int_value: i32,
    float_value: f32,
    reference: *const i32,
    manually_drop: ManuallyDrop<String>,
}

fn main() {
    let u = MyUnion { int_value: 42 };
    unsafe {
        println!("int value: {}", u.int_value);
    }
}
