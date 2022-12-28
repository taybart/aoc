mod part1;

fn main() -> Result<(), String> {
    let example = std::fs::read_to_string("./src/example.txt")
        .map_err(|e| format!("read file failed {e}"))?;
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;

    println!("{}", part1::solve(example)?);
    // println!("{}", part1::solve(input.clone())?);
    Ok(())
}
