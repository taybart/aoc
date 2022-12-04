pub fn matches(ranges: [u32; 4]) -> bool {
    let start1 = ranges[0];
    let end1 = ranges[1];
    let start2 = ranges[2];
    let end2 = ranges[3];
    start1 <= end2 && start2 <= end1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve, EXAMPLE_INPUT};

    #[test]
    fn part1_example() -> Result<(), String> {
        let example_input = EXAMPLE_INPUT.to_string();
        assert_eq!(solve(example_input, matches)?, 4);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input, matches)?, 924);
        Ok(())
    }
}
