use std::collections::VecDeque;

mod part1;
mod part2;

const EXAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

pub fn solve(input: String, signal_size: usize) -> Result<usize, String> {
    let mut buf: VecDeque<char> = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        if i >= signal_size {
            // check for duplicates
            let mut cp: Vec<&char> = buf.iter().collect::<Vec<&char>>();
            cp.sort();
            cp.dedup();
            if cp.len() == signal_size {
                println!("{buf:?}");
                return Ok(i);
            }
            // remove old signal
            buf.pop_back();
        }
        // add next signal
        buf.push_front(c);
    }
    Ok(0)
}

fn main() -> Result<(), String> {
    println!("{}", solve(EXAMPLE_INPUT.to_string(), part1::SIGNAL_SIZE)?);
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    println!("{}", solve(input.clone(), part1::SIGNAL_SIZE)?);
    println!("{}", solve(input, part2::SIGNAL_SIZE)?);
    Ok(())
}
