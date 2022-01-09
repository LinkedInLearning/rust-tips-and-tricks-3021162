use bag::Bag;

fn main() {
    println!("Program: Bookbag\n");

    let mut bag = Bag::new();

    let items = vec![
        "Pencils".to_string(),
        "Notebook".to_string(),
        "The Rust Book".to_string(),
    ];

    for item in items {
        bag.put_in_bag(item);
    }
}