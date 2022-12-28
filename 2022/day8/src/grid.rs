use std::fmt;

pub struct Grid {
    pub vec: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn from_string(input: String) -> Result<Grid, String> {
        let mut width = 0;
        if let Some(new_line) = input.chars().position(|c| c == '\n') {
            width = new_line;
        }
        if width == 0 {
            return Err("could not determine grid width".to_string());
        }
        let mut vec = Vec::new();
        for ch in input.chars() {
            if ch == '\n' {
                continue;
            }
            vec.push(ch.to_digit(10).unwrap());
        }
        Ok(Grid {
            width,
            height: vec.len()/width,
            vec,
        })
    }

    pub fn get(&self, row: usize, col: usize) -> &u32 {
        self.vec.get(self.width*row+col).unwrap()
    }

    pub fn row_visible(&self, row: usize, col: usize) -> bool {
        let mut saw_larger = false;
        let selected_val = self.get(row, col);
        for i in 0..self.width {
            if i == col {
                if !saw_larger {
                    break;
                }
                saw_larger = false;
                continue;
            }
            if self.get(row,i) >= selected_val {
                saw_larger = true;
            }
        }
        !saw_larger
    }

    pub fn col_visible(&self, row: usize, col: usize) -> bool {
        let mut saw_larger = false;
        let selected_val = self.get(row,col);
        for i in 0..self.height {
            if i == row {
                if !saw_larger {
                    break;
                }
                saw_larger = false;
                continue;
            }
            let val = self.get(i,col);
            if val >= selected_val {
                saw_larger = true;
            }
        }
        !saw_larger
    }


    pub fn border_size(&self) -> usize {
        /* top/bottom-ends */   
        ((self.width - 2) * 2) + 
            /* sides */
            (self.vec.len() / self.width) * 2
    }

    pub fn row_view_score(&self, row: usize, col: usize) -> u32 {
        let mut left_seen = 0;
        let mut right_seen = 0;
        let selected_val = self.get(row, col);
        for i in 1..col + 1 {
            left_seen += 1;
            if self.get(row, col - i) >= selected_val {
                break;
            }
        }
        for i in col + 1..self.width {
            right_seen += 1;
            if self.get(row, i) >= selected_val {
                break;
            }
        }
        left_seen * right_seen
    }

    pub fn col_view_score(&self, row: usize, col: usize) -> u32 {
        let mut above_seen = 0;
        let mut below_seen = 0;
        let selected_val = self.get(row, col);
        for i in 1..row + 1 {
            above_seen += 1;
            let val = self.get(row - i, col);
            println!("{}, {col} = {val}", row - i);
            if val >= selected_val {
                break;
            }
        }
        for i in row + 1..self.height {
            below_seen += 1;
            if self.get(i, col) >= selected_val {
                break;
            }
        }
        above_seen * below_seen
    }

    pub fn view_score(&self, row: usize, col: usize) -> u32 {
        self.row_view_score(row, col) * self.col_view_score(row, col)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, e) in self.vec.iter().enumerate() {
            if i % (self.width) == 0 && i != 0 {
                writeln!(f)?;
            }
            write!(f, "{e}")?;
        }
        Ok(())
    }
}
