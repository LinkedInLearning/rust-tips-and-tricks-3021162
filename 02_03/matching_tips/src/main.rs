fn commercials(hour: u32) -> String {
    match hour {
        0 => "Classic video bundle commerials".to_string(),
        1 => "Classic video bundle commerials".to_string(),
        2 => "Classic video bundle commerials".to_string(),
        3 => "Classic video bundle commerials".to_string(),
        4 => "Classic video bundle commerials".to_string(),
        5 => "Classic video bundle commerials".to_string(),
        6 => "Classic video bundle commerials".to_string(),
        7 => "Classic video bundle commerials".to_string(),
        8 => "Food commercials".to_string(),
        9 => "Clothing commercials".to_string(),
        10 => "Clothing commercials".to_string(),
        11 => "Clothing commercials".to_string(),
        12 => "Food commercials".to_string(),
        13 => "Clothing commercials".to_string(),
        14 => "Clothing commercials".to_string(),
        15 => "Clothing commercials".to_string(),
        16 => "Clothing commercials".to_string(),
        17 => "Clothing commercials".to_string(),
        18 => "Food commercials".to_string(),
        19 => "Season ticket commercials".to_string(),
        20 => "Season ticket commercials".to_string(),
        21 => "Season ticket commercials".to_string(),
        22 => "Season ticket commercials".to_string(),
        23 => "Season ticket commercials".to_string(),
        24 => "Season ticket commercials".to_string(),
        _ => "NOT A VALID HOUR".to_string(),
    }
}

fn main() {
    for hour in 0..=25 {
        println!("Hour: {}, result: {}", hour, commercials(hour));
    }
}