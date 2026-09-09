use my_macro_crate::make_json;
use serde_json::json;

fn main() {
    let data = make_json!(json!({"name": "Alice", "age": 30, "is_student": false}));
    println!("{}", data);
}
