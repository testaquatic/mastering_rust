use attribute_macro::log;

#[log]
fn my_function() {
    println!("이것은 실제 함수 동작입니다.");
}

fn main() {
    my_function();
}
