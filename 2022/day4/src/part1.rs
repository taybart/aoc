pub fn matches(ranges: [u32; 4]) -> bool {
    let start1 = ranges[0];
    let end1 = ranges[1];
    let start2 = ranges[2];
    let end2 = ranges[3];
    (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn part1_example() -> Result<(), String> {
        let example_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(solve(example_input, matches)?, 2);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input, matches)?, 562);
        Ok(())
    }
}
