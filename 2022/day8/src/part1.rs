use crate::grid::Grid;

pub fn solve(input: String) -> Result<u32, String> {
    let grid = Grid::from_string(input)?;

    let mut visible = 0;
    for i in 0..grid.vec.len() {
        if i < grid.width || i >= (grid.width - 1) * grid.height {
            continue;
        }
        let row = i / grid.width;
        let col = i % grid.width;
        if col == 0 || col == grid.width - 1 {
            continue;
        }

        if grid.row_visible(row, col) || grid.col_visible(row, col) {
            visible += 1;
        }
    }

    let total = grid.border_size() + visible;
    Ok(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<(), String> {
        let example = std::fs::read_to_string("./src/example.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(example)?, 21);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 1843);
        Ok(())
    }
}
