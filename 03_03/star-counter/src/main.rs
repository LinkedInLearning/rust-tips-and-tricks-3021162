use std::env;

fn main() {
    let sum: i32 = env::args().skip(1).fold(0, |acc, x| {
        acc + x.parse::<i32>().expect("All args must be integers")
    });

    println!("Sum: {}", sum);
}
