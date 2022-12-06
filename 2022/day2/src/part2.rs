use super::{
    Outcome,
    Outcome::{Draw, Lose, Win},
    Shape,
    Shape::{Paper, Rock, Scissors},
};

fn choose_shape(shape: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Win => match shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Lose => match shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Draw => shape,
    }
}

pub fn solve(input: String) -> Result<i32, String> {
    let mut score = 0;
    for round in input.split('\n') {
        if round.is_empty() {
            continue;
        }
        let mut moves = round.split(' ');
        let thiers = Shape::from_str(moves.next().unwrap())?;
        let outcome = Outcome::from_str(moves.next().unwrap())?;
        let ours = choose_shape(thiers, outcome);
        score += ours as i32 + outcome as i32;
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() -> Result<(), String> {
        let example_input = "A Y\nB X\nC Z".to_string();
        assert_eq!(solve(example_input)?, 12);
        Ok(())
    }
    #[test]
    fn part2() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 11998);
        Ok(())
    }
}
