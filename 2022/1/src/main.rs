fn main() -> Result<(), String> {
    let input = std::fs::read_to_string("./src/input.txt")
        .map_err(|e| format!("read file failed {}", e))?;

    let mut elfs: Vec<i32> = vec![];
    for (i, elf) in input.split("\n\n").enumerate() {
        elfs.push(0);
        for n in elf.split("\n") {
            // sometimes splits are empty
            if n.len() > 0 {
                match n.parse::<i32>() {
                    Ok(num) => {
                        elfs[i] += num;
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        }
    }

    elfs.sort();

    // part 1
    println!("most cals! {}", elfs.last().unwrap());
    // part2
    println!(
        "top three combined! {}",
        elfs[elfs.len() - 3..elfs.len()].iter().sum::<i32>()
    );
    Ok(())
}
