mod part1;
mod part2;

pub fn solve(input: String, matches: fn([u32; 4]) -> bool) -> Result<u32, String> {
    let mut res = 0;
    for assignments in input.split("\n") {
        if assignments.is_empty() {
            continue;
        }
        let mut ranges = [0; 4];
        let mut i = 0;
        for assigned_zones in assignments.split(",") {
            if assigned_zones.is_empty() {
                continue;
            }
            for zones in assigned_zones.split("-") {
                if zones.is_empty() {
                    continue;
                }
                ranges[i] = zones.parse::<u32>().unwrap();
                i += 1;
            }
        }
        if matches(ranges) {
            res += 1;
        }
    }
    Ok(res)
}

fn main() -> Result<(), String> {
    let _input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        .to_string();

    let input =
        std::fs::read_to_string("./src/input.txt").map_err(|e| format!("read file failed {e}"))?;
    let res = solve(input.clone(), part1::matches)?;
    println!("res: {res}");
    let res = solve(input, part2::matches)?;
    println!("res: {res}");
    Ok(())
}
