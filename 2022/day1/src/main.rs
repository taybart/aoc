fn main() -> Result<(), String> {
    let elfs = parse_input()?;
    println!("most cals! {}", part1(&elfs));
    println!("top three combined! {}", part2(&elfs));
    Ok(())
}

fn parse_input() -> Result<Vec<i32>, String> {
    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;

    let mut elfs: Vec<i32> = vec![];
    for (i, elf) in input.split("\n\n").enumerate() {
        elfs.push(0);
        for n in elf.split("\n") {
            // sometimes splits are empty
            if n.len() > 0 {
                elfs[i] += n
                    .parse::<i32>()
                    .map_err(|e| format!("unable to parse string {e}"))?;
            }
        }
    }

    elfs.sort();
    Ok(elfs)
}

fn part1(elfs: &Vec<i32>) -> i32 {
    match elfs.last() {
        Some(i) => *i,
        None => -1,
    }
}
fn part2(elfs: &Vec<i32>) -> i32 {
    elfs[elfs.len() - 3..elfs.len()].iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctness() -> Result<(), String> {
        let elfs = parse_input()?;
        assert_eq!(part1(&elfs), 69281);
        assert_eq!(part2(&elfs), 201524);
        Ok(())
    }
}
