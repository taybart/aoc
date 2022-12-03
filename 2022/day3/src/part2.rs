use super::make_scores;
use std::collections::HashSet;

fn get_intersection(elf_group: Vec<HashSet<char>>) -> Result<char, String> {
    let first = elf_group.get(0).unwrap();
    let second = elf_group.get(1).unwrap();
    let third = elf_group.get(2).unwrap();

    let mut c = first.intersection(&second).copied().collect::<Vec<char>>();

    let hs_c = c.into_iter().collect::<HashSet<char>>();
    c = hs_c.intersection(&third).copied().collect::<Vec<char>>();

    match c.len() {
        1 => Ok(c[0]),
        0 => Err("no unique items between elfs".to_string()),
        _ => Err("more than one unique item between elfs".to_string()),
    }
}

pub fn solve(input: String) -> Result<u32, String> {
    let scores = make_scores();
    let mut score = 0;

    let mut elf_group: Vec<HashSet<char>> = Vec::with_capacity(3);
    for rucksack in input.split("\n") {
        if rucksack.is_empty() {
            continue;
        }

        let id = rucksack.chars().collect::<HashSet<char>>();

        elf_group.push(id);

        if elf_group.len() == 3 {
            let inter = get_intersection(elf_group)?;
            score += scores.get(&inter).unwrap();
            elf_group = Vec::with_capacity(3);
        }
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_example() -> Result<(), String> {
        let example_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        assert_eq!(solve(example_input)?, 70);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 2838);
        Ok(())
    }
}
