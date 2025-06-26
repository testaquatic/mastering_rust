use function_like_macro::make_json;
use serde_json::json;

fn main() {
    let data = make_json!(json!(
        {
            "name": "Alice",
            "age": 30
        }
    ));

    println!("{}", data);
}
