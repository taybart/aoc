use crate::tree::Tree;
use std::path::PathBuf;

const DISK_SPACE: u32 = 70000000;
const REQUIRED_FREE: u32 = 30000000;

pub fn solve(input: String) -> Result<u32, String> {
    let tree = Tree::from_string(input)?;

    // calc disk params
    let total_used = tree.dirs.get(&PathBuf::from("/")).unwrap();
    let minimum_required = REQUIRED_FREE - (DISK_SPACE - total_used);

    let mut candidates: Vec<u32> = Vec::new();
    for (_, dir) in tree.dirs.into_iter() {
        if dir >= minimum_required {
            candidates.push(dir)
        }
    }
    if let Some(min) = candidates.into_iter().min() {
        return Ok(min);
    }
    Err("could not get minimum required free space".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EXAMPLE_INPUT;

    #[test]
    fn part2_example() -> Result<(), String> {
        assert_eq!(solve(EXAMPLE_INPUT.to_string())?, 24933642);
        Ok(())
    }
    #[test]
    fn part2() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 7068748);
        Ok(())
    }
}
