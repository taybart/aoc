pub const SIGNAL_SIZE: usize = 4;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve, EXAMPLE_INPUT};

    #[test]
    fn part1_example() -> Result<(), String> {
        let tests = [
            (EXAMPLE_INPUT, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for test in tests {
            assert_eq!(solve(test.0.to_string(), SIGNAL_SIZE)?, test.1);
        }

        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input, SIGNAL_SIZE)?, 1134);
        Ok(())
    }
}
