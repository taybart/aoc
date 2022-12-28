use crate::tree::Tree;

const MAX_SIZE: u32 = 100000;

pub fn solve(input: String) -> Result<u32, String> {
    let tree = Tree::from_string(input)?;

    let mut total = 0;
    for (_, dir) in tree.dirs.into_iter() {
        if dir < MAX_SIZE {
            total += dir;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EXAMPLE_INPUT;

    #[test]
    fn part1_example() -> Result<(), String> {
        assert_eq!(solve(EXAMPLE_INPUT.to_string())?, 95437);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 1307902);
        Ok(())
    }
}
