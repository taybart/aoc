use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

fn build_stacks(input: String) -> HashMap<u32, VecDeque<String>> {
    let mut stacks = HashMap::new();
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut rows: Vec<&str> = split.first().unwrap().split('\n').collect();
    let col_names = rows.pop();
    for col_name in col_names.unwrap().split(' ') {
        if col_name.is_empty() {
            continue;
        }
        let idx = col_name.parse::<u32>();
        stacks.insert(idx.unwrap(), VecDeque::<String>::new());
    }
    // fix for blank columns
    let re = Regex::new(r"\s\s\s\s").unwrap();
    for row in rows.iter() {
        let mut col_num = 1;
        for column_entry in re.replace_all(row, " ").split(' ') {
            let col = stacks.get_mut(&col_num).unwrap();
            if !column_entry.is_empty() {
                let letter = column_entry.chars().nth(1).unwrap();
                col.push_front(letter.to_string());
            }
            col_num += 1;
            if col_num > stacks.len() as u32 {
                break;
            }
        }
    }
    stacks
}
fn build_moves(input: String) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let split: Vec<&str> = input.split("\n\n").collect();
    // bottom
    let instructions: Vec<&str> = split.get(1).unwrap().split('\n').collect();
    for instruction in instructions.iter() {
        if instruction.is_empty() {
            continue;
        }
        let mut m = Move::default();

        // remove non numbers
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(instruction).unwrap();
        m.amount = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        m.from = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        m.to = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();

        let test = format!("move {} from {} to {}", m.amount, m.from, m.to);
        if test != *instruction.to_string() {
            panic!("{} != {}", test, instruction)
        }
        moves.push(m);
    }
    moves
}

pub fn solve(input: String) -> Result<String, String> {
    let mut stacks = build_stacks(input.clone());
    let moves = build_moves(input);

    for m in moves {
        let mut update = String::new();
        for _ in 0..m.amount {
            if let Some(from) = stacks.get_mut(&m.from) {
                update.push_str(&from.pop_back().unwrap());
            }
        }
        if !update.is_empty() {
            if let Some(to) = stacks.get_mut(&m.to) {
                for c in update.chars().rev() {
                    to.push_back(c.to_string());
                }
            }
        }
    }

    let mut solution = String::new();
    for i in 1..stacks.len() + 1 {
        let c = stacks.get(&(i as u32)).unwrap().back().unwrap();
        solution.push_str(c);
    }

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EXAMPLE_INPUT;

    #[test]
    fn part2_example() -> Result<(), String> {
        let example_input = EXAMPLE_INPUT.to_string();
        assert_eq!(solve(example_input)?, "MCD");
        Ok(())
    }
    #[test]
    fn part2() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, "VLCWHTDSZ");
        Ok(())
    }
}
