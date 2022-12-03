use std::collections::HashMap;

mod part1;
mod part2;

fn make_scores() -> HashMap<char, u32> {
    let mut lower = "abcdefghijklmnopqrstuvwxyz".to_string();
    let upper = lower.to_uppercase();
    lower.push_str(&upper);
    let mut scores: HashMap<char, u32> = HashMap::new();
    for (i, c) in lower.chars().enumerate() {
        scores.insert(c, i as u32 + 1);
    }
    scores
}

fn main() -> Result<(), String> {
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    let mut score = part1::solve(input.clone())?;
    println!("{}", score);
    score = part2::solve(input)?;
    println!("{}", score);
    Ok(())
}
