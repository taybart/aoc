mod part1;
mod part2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    /*
     * Rock<->Scissors  (1 - 3 = -2)
     * Paper<->Rock     (2 - 1 =  1)
     * Scissors<->Paper (3 - 2 =  1)
     */
    pub fn beats(self, m: Shape) -> bool {
        let diff = self as i32 - m as i32;
        diff == 1 || diff == -2
    }
    pub fn from_str(input: &str) -> Result<Shape, String> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(format!("unknown move {}", input)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Outcome {
    pub fn from_str(input: &str) -> Result<Outcome, String> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("unknown move {input}")),
        }
    }
}

fn main() -> Result<(), String> {
    // let input = "A Y\nB X\nC Z".to_string();
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    println!("part1 score: {}", part1::solve(input.clone())?);
    println!("part2 score: {}", part2::solve(input)?);
    Ok(())
}
