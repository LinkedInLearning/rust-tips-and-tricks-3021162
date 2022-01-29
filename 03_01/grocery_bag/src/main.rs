pub struct TheBag {
    bag: Vec<String>,
}

impl TheBag {
    pub fn new() -> Self {
        TheBag { bag: Vec::new() }
    }

    pub fn put_in_bag(&mut self, item: String) {
        println!("putting {:?} in the bag", item);

        self.bag.push(item);
    }

    pub fn pull_out_of_bag(&mut self) -> Option<String> {
        let item = self.bag.pop();

        println!("pulling {:?} out of bag", &item);

        item
    }
}

fn main() {
    println!("Program: Grocery Bag\n");

    let mut bag = TheBag::new();

    let items = vec![
        "Eggs".to_string(),
        "Potatoes".to_string(),
        "Orange".to_string(),
    ];

    for item in items {
        bag.put_in_bag(item);
    }
}
