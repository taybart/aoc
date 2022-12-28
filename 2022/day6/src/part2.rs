pub const SIGNAL_SIZE: usize = 14;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve, EXAMPLE_INPUT};

    #[test]
    fn part2_example() -> Result<(), String> {
        let tests = [
            (EXAMPLE_INPUT, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for test in tests {
            assert_eq!(solve(test.0.to_string(), SIGNAL_SIZE)?, test.1);
        }
        Ok(())
    }
    #[test]
    fn part2() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input, SIGNAL_SIZE)?, 2263);
        Ok(())
    }
}
