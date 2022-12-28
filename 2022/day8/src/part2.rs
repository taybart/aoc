use crate::grid::Grid;

pub fn solve(input: String) -> Result<u32, String> {
    let grid = Grid::from_string(input)?;

    let mut highest_score = 0;

    for i in 0..grid.vec.len() {
        if i < grid.width || i >= (grid.width - 1) * grid.height {
            continue;
        }
        let row = i / grid.width;
        let col = i % grid.width;
        if col == 0 || col == grid.width - 1 {
            continue;
        }
        let score = grid.view_score(row, col);
        println!("{row}, {col} score {score}");
        if score > highest_score {
            highest_score = score;
        }
    }

    Ok(highest_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() -> Result<(), String> {
        let example = std::fs::read_to_string("./src/example.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(example)?, 8);
        Ok(())
    }
    #[test]
    fn part2() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 180000);
        Ok(())
    }
}
