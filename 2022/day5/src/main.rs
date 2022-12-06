mod part1;
mod part2;

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = " [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

fn main() -> Result<(), String> {
    // println!("part1 example {}", part1::solve(EXAMPLE_INPUT.to_string())?);
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    println!("part1 {}", part1::solve(input.clone())?);
    println!("part2 {}", part2::solve(input)?);
    Ok(())
}
