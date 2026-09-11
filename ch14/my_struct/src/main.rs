struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("{} is being dropped", self.name);
    }
}

fn main() {
    let a = MyStruct {
        name: "A".to_string(),
    };
    std::mem::drop(a);
    let _b = MyStruct {
        name: "B".to_string(),
    };
}
