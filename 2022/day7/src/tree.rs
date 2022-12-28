use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
enum Line {
    Nop,
    Cd,
    Ls,
    Directory,
    File,
}

impl Line {
    fn parse(line: &str) -> Result<(Line, Vec<String>), String> {
        let ch: Vec<char> = line.chars().collect();
        if let Some(first) = ch.first() {
            if *first == '$' {
                let cmd = ch.get(..4).unwrap().iter().collect::<String>();
                if cmd == "$ ls" {
                    return Ok((Line::Ls, vec![]));
                } else if cmd == "$ cd" {
                    let s = ch.get(5..).unwrap().iter().collect::<String>();
                    return Ok((Line::Cd, vec![s]));
                }
            } else if *first == 'd' {
                let re = Regex::new(r"dir (.*)").unwrap();
                let caps = re.captures(line).unwrap();
                let dir = caps.get(1).unwrap().as_str();
                return Ok((Line::Directory, vec![dir.to_string()]));
            } else if first.is_ascii_digit() {
                let re = Regex::new(r"(\d+) (.*)").unwrap();
                let caps = re.captures(line).unwrap();
                let size = caps.get(1).unwrap().as_str();
                let name = caps.get(2).unwrap().as_str();
                return Ok((Line::File, vec![size.to_string(), name.to_string()]));
            }
        }
        Ok((Line::Nop, vec!["".to_string()]))
    }
}

pub struct Tree {
    pub pwd: PathBuf,
    pub dirs: HashMap<PathBuf, u32>,
}

impl Tree {
    pub fn from_string(input: String) -> Result<Tree, String> {
        let mut tree = Tree {
            pwd: PathBuf::new(),
            dirs: HashMap::new(),
        };
        for line in input.split('\n') {
            match Line::parse(line)? {
                (Line::Cd, contents) => {
                    let folder = contents.get(0).unwrap();
                    if folder == ".." {
                        tree.pwd.pop();
                    } else {
                        tree.pwd.push(folder);
                        tree.dirs.entry(tree.pwd.clone()).or_insert(0);
                    }
                    // println!("{:?}", tree.pwd);
                }
                (Line::Directory, _) | (Line::Ls, _) | (Line::Nop, _) => {
                    // println!("ls");
                }

                (Line::File, contents) => {
                    // let name = contents.get(1).unwrap().to_string();
                    let size: u32 = contents.get(0).unwrap().parse().unwrap();
                    if let Some(dir) = tree.dirs.get_mut(&tree.pwd) {
                        // println!("{:?} {name} {size}", tree.pwd);
                        *dir += size;
                    }
                    // double add up the tree...i wish i had noticed this little line earlier...
                    let mut tmp_path = tree.pwd.clone();
                    while tmp_path.pop() {
                        if let Some(dir) = tree.dirs.get_mut(&tmp_path) {
                            *dir += size;
                        }
                    }
                }
            };
        }
        Ok(tree)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (path, dir) in self.dirs.iter() {
            writeln!(f, "{} {}", path.to_str().unwrap(), dir)?;
        }
        Ok(())
    }
}
