#[derive(Debug, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Self {name, age}
    }
}

fn main() {}
