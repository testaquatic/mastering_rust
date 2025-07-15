use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

fn main() {
    let mut people = HashSet::new();

    people.insert(Person {
        name: "Alice".to_string(),
        age: 30,
    });

    people.insert(Person {
        name: "Bob".to_string(),
        age: 25,
    });

    people.insert(Person {
        name: "Alice".to_string(),
        age: 30,
    });

    println!("{people:?}");
}
