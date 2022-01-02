fn recursive_solution(n: u64) -> u64 {
    match n {
        0 => panic!("Input must be 1 or greater"),
        1 => 5,
        _ => recursive_solution(n - 1) + 3
    }
}

fn main() {
    
}
