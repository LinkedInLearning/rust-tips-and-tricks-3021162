fn commercials(hour: u32) -> String {
    match hour {
        0..=7 => "Classic video bundle commercials".to_string(),
        8 | 12 | 18 => "Food commercials".to_string(),
        9..=11 | 13..=17 => "Clothing commercials".to_string(),
        19..=24 => "Season ticket commercials".to_string(),
        _ => "NOT A VALID HOUR".to_string(),
    }
}

fn main() {
    for hour in 0..=25 {
        println!("Hour: {}, result: {}", hour, commercials(hour));
    }
}