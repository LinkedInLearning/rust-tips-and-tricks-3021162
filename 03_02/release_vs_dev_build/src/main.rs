fn is_palindrome(value: &str) -> bool {
    let mut result = true;

    let chars: Vec<char> = value.chars().collect();

    for (index, rev_item) in value.chars().rev().enumerate() {
        if chars[index] != rev_item {
            result = false;
            break;
        }
    }

    result
}

fn main() {
    let mut result = 0;

    for x in 100..=1000 {
        for y in 100..=1000 {
            let product = x * y;
            let product_string = product.to_string();

            if is_palindrome(&product_string) {
                if product > result {
                    result = product
                }
            }
        }
    }

    println!("Answer: {}", result);
}
