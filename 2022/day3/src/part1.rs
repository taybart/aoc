use super::make_scores;
use std::collections::HashSet;

fn get_intersection(rucksack: &str) -> Result<char, String> {
    let first: HashSet<char> = rucksack[0..rucksack.len() / 2].chars().collect();
    let second: HashSet<char> = rucksack[rucksack.len() / 2..].chars().collect();

    let c = first.intersection(&second).copied().collect::<Vec<_>>();
    match c.len() {
        1 => Ok(c[0]),
        0 => Err("no unique items between compartments".to_string()),
        _ => Err("more than one unique item between compartments".to_string()),
    }
}

pub fn solve(input: String) -> Result<u32, String> {
    let scores = make_scores();
    let mut score = 0;
    for rucksack in input.split("\n") {
        if rucksack.is_empty() {
            continue;
        }
        let common = get_intersection(rucksack)?;
        score += scores.get(&common).unwrap();
    }

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<(), String> {
        let example_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        assert_eq!(solve(example_input)?, 157);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 7908);
        Ok(())
    }
}
