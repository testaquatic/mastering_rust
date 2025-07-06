trait X {
    fn method_x(&self);
}

trait Y {
    fn method_y(&self);
}

trait Z: X + Y {
    fn method_z(&self);
}

struct MyStruct;

impl X for MyStruct {
    fn method_x(&self) {
        println!("Method X");
    }
}

impl Y for MyStruct {
    fn method_y(&self) {
        println!("Method Y");
    }
}

impl Z for MyStruct {
    fn method_z(&self) {
        println!("Method Z");
    }
}

fn main() {
    let my_struct = MyStruct;
    my_struct.method_x();
    my_struct.method_y();
    my_struct.method_z();
}
