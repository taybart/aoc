use std::fmt;

// #[derive(Debug, Clone, Copy)]
// enum Dir {
//     L,
//     R,
//     U,
//     D,
// }

// impl Dir {
//     fn from_char(ch: char) -> Result<Dir, String> {
//         match ch {
//             'L' => Ok(Dir::L),
//             'R' => Ok(Dir::R),
//             'U' => Ok(Dir::U),
//             'D' => Ok(Dir::D),
//             _ => Err("unknown direction".to_string()),
//         }
//     }
// }

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn is_touching(self, c: Coordinate) -> bool {
        let x = (c.x - self.x).abs();
        let y = (c.y - self.y).abs();
        (x == 0 || x == 1) && (y == 0 || y == 1)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn draw(head: Coordinate, tail: Coordinate) {
    for i in 0..5 {
        for j in 0..6 {
            if 4 - i == head.y && j == head.x {
                print!("H")
            } else if 4 - i == tail.y && j == tail.x {
                print!("T")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

fn draw_unique(locations: Vec<Coordinate>) {
    for i in 0..5 {
        for j in 0..6 {
            if 4 - i == head.y && j == head.x {
                print!("H")
            } else if 4 - i == tail.y && j == tail.x {
                print!("T")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

fn parse(input: String) -> Result<Vec<(char, i32)>, String> {
    let mut instructions = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        // let dir = Dir::from_char(line.chars().next().unwrap())?;
        let dir = line.chars().next().unwrap();
        let distance = line.chars().nth(2).unwrap().to_digit(10).unwrap() as i32;
        instructions.push((dir, distance));
    }
    Ok(instructions)
}

pub fn solve(input: String) -> Result<usize, String> {
    let instructions = parse(input)?;

    let mut head = Coordinate::default();
    let mut tail = Coordinate::default();

    let mut tail_locations: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }];
    for inst in instructions.iter() {
        println!("{inst:?}");
        for i in 0..inst.1 {
            match inst.0 {
                'L' => {
                    head.x -= 1;
                    if i != 0 && !head.is_touching(tail) {
                        tail.x -= 1;
                        if tail.y != head.y {
                            tail.y = head.y;
                        }
                        tail_locations.push(tail);
                    }
                }
                'R' => {
                    head.x += 1;
                    if i != 0 && !head.is_touching(tail) {
                        tail.x += 1;
                        if tail.y != head.y {
                            tail.y = head.y;
                        }
                        tail_locations.push(tail);
                    }
                }
                'U' => {
                    head.y += 1;
                    if i != 0 && !head.is_touching(tail) {
                        tail.y += 1;
                        if tail.x != head.x {
                            tail.x = head.x;
                        }
                        tail_locations.push(tail);
                    }
                }
                'D' => {
                    head.y -= 1;
                    // TODO: fix if snake doubles back
                    if i != 0 && !head.is_touching(tail) {
                        tail.y -= 1;
                        if tail.x != head.x {
                            tail.x = head.x;
                        }
                        tail_locations.push(tail);
                    }
                }
                _ => println!("unknown direction {:?}", inst.0),
            }
            // println!("T{tail} H{head} {}", head.is_touching(tail));
            // println!();
            draw(head, tail);
        }
    }
    tail_locations.sort_by_key(|k| (k.x, k.y));
    // println!("{tail_locations:?}");
    tail_locations.dedup();
    for loc in tail_locations.iter() {
        println!("{loc}");
    }
    // 3147 was too low
    Ok(tail_locations.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() -> Result<(), String> {
        let example = std::fs::read_to_string("./src/example.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(example)?, 13);
        Ok(())
    }
    #[test]
    #[ignore]
    fn part1() -> Result<(), String> {
        let input = std::fs::read_to_string("./src/input.txt")
            .map_err(|e| format!("read file failed {e}"))?;
        assert_eq!(solve(input)?, 0);
        Ok(())
    }
}
