use aoc22;
use core::panic;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cd {
    MoveIn(String),
    MoveOut,
    GotoRoot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct LsOutput {
    files: Vec<File>,
    dirs: Vec<String>,
}

impl LsOutput {
    fn sum_file_sizes(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Path(Vec<String>);

impl Path {
    fn root() -> Self {
        Path(vec!["/".into()])
    }
    fn apply_cd(&mut self, cd: Cd) {
        use Cd::*;
        match cd {
            MoveIn(s) => self.0.push(s),
            MoveOut => {
                self.0.pop();
                if self.0.is_empty() {
                    self.0.push("/".into());
                }
            }
            GotoRoot => {
                self.0.clear();
                self.0.push("/".into());
            }
        }
    }
    fn append(&self, s: String) -> Self {
        let mut this = self.clone();
        this.0.push(s);
        this
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Fs(BTreeMap<Path, LsOutput>);

fn parse_input() -> Fs {
    let mut lines = aoc22::read_input_lines().peekable();
    let mut current_dir = Path::root();
    let mut fs = Fs::default();
    while let Some(line) = lines.next() {
        if line.starts_with("$ ls") {
            let mut ls_output = LsOutput::default();
            while let Some(next_line) = lines.peek() {
                if next_line.starts_with("$") {
                    break;
                } else {
                    let line = lines.next().unwrap();
                    let mut parts = line.split_whitespace();
                    let part0 = parts.next().unwrap();
                    let name = parts.next().unwrap().to_owned();
                    if part0 == "dir" {
                        ls_output.dirs.push(name);
                    } else {
                        let file = File {
                            name,
                            size: part0.parse().unwrap(),
                        };
                        ls_output.files.push(file);
                    }
                }
            }
            println!("{:?} => {:?}", &current_dir, &ls_output);
            fs.0.insert(current_dir.clone(), ls_output);
        } else if line.starts_with("$ cd") {
            let cd = match line.trim_start_matches("$ cd").trim() {
                "/" => Cd::GotoRoot,
                ".." => Cd::MoveOut,
                s => Cd::MoveIn(s.into()),
            };
            current_dir.apply_cd(cd);
            println!("{:?}", &current_dir);
        } else {
            panic!("could not parse command")
        }
    }
    fs
}

impl Fs {
    fn total_size(&self, path: &Path) -> Option<u64> {
        let mut summed = 0;
        let mut stack = vec![path.clone()];
        while let Some(path) = stack.pop() {
            let entry = self.0.get(&path)?;
            summed += entry.sum_file_sizes();
            for dir in &entry.dirs {
                stack.push(path.append(dir.clone()));
            }
        }
        Some(summed)
    }
}

fn main() {
    let fs = parse_input();
    println!();

    let fs_space_total: i64 = 70_000_000;
    let fs_space_needed: i64 = 30_000_000;
    let fs_space_used = fs.total_size(&Path::root()).unwrap() as i64;
    let fs_space_available = fs_space_total - fs_space_used;
    if fs_space_available >= fs_space_needed {
        panic!("No freeing needed");
    }
    let fs_space_to_be_freed = fs_space_needed - fs_space_available;
    let fs_space_to_be_freed = fs_space_to_be_freed as u64;
    println!("need to free {}", fs_space_to_be_freed);

    let mut candidates: Vec<u64> =
        fs.0.keys()
            .filter_map(|path| {
                let total_size = fs.total_size(path).unwrap();
                println!("{:?} => {}", &path, total_size);
                if total_size >= fs_space_to_be_freed {
                    Some(total_size)
                } else {
                    None
                }
            })
            .collect();

    candidates.sort();

    println!();
    println!("{}", candidates[0]);
}
