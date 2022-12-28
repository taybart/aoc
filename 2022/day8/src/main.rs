mod part1;
mod part2;

mod grid;

fn main() -> Result<(), String> {
    let example = std::fs::read_to_string("./src/example.txt")
        .map_err(|e| format!("read file failed {e}"))?;
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;

    println!("{}", part1::solve(example.clone())?);
    println!("{}", part1::solve(input.clone())?);
    println!("{}", part2::solve(example)?);
    println!("{}", part2::solve(input)?);
    Ok(())
}
