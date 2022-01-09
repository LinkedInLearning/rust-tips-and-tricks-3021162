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

fn main() {
    let person_1 = Person::new("B".to_string(), 2);
    let person_2 = Person::new("A".to_string(), 3);

    dbg!(person_2 > person_1);
}
