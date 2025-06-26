use main_crate::HelloWorld;

#[derive(HelloWorld)]
struct MyStruct;

fn main() {
    MyStruct::hello_world();
}
