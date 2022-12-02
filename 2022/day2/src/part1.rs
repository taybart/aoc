use super::{Outcome, Shape};

fn get_outcome(opponent: Shape, us: Shape) -> Outcome {
    if opponent == us {
        Outcome::Draw
    } else if opponent.beats(us) {
        Outcome::Lose
    } else {
        Outcome::Win
    }
}

pub fn solve(input: String) -> Result<i32, String> {
    let mut score = 0;
    for round in input.split("\n") {
        if round.len() == 0 {
            continue;
        }
        let mut moves = round.split(" ");
        let opponent = Shape::from_str(moves.next().unwrap())?;
        let us = Shape::from_str(moves.next().unwrap())?;
        score += us as i32 + get_outcome(opponent, us) as i32;
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<(), String> {
        let example_input = "A Y\nB X\nC Z".to_string();
        assert_eq!(solve(example_input)?, 15);
        Ok(())
    }
    #[test]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {}", e))?;
        assert_eq!(solve(input)?, 8933);
        Ok(())
    }
}
