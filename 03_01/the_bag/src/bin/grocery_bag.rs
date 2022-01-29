use the_bag::TheBag;

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