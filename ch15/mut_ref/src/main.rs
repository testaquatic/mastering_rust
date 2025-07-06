use std::sync::Mutex;

struct Struct {
    mutex: Mutex<String>,
}

impl Struct {
    fn get_string(&mut self) -> &str {
        self.mutex.get_mut().unwrap()
    }

    fn mutate_string(&self) {
        *self.mutex.lock().unwrap() = "suprise".to_string()
    }
}

fn main() {
    let mut s = Struct {
        mutex: Mutex::new("string".to_string()),
    };

    let str_ref = s.get_string();
    s.mutate_string();
    dbg!(str_ref);
}
