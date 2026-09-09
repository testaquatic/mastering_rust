use my_macro2_crate::HelloWorld;

#[derive(HelloWorld)]
struct MyStruct;

pub trait HelloWorld {
    fn hello_world();
}

fn main() {
    MyStruct::hello_world();
}
