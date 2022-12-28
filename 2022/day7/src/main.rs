mod part1;
mod part2;
mod tree;

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

fn main() -> Result<(), String> {
    println!("{}", part1::solve(EXAMPLE_INPUT.to_string())?);

    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    println!("{}", part1::solve(input.clone())?);

    println!("{}", part2::solve(EXAMPLE_INPUT.to_string())?);
    println!("{}", part2::solve(input)?);
    Ok(())
}
