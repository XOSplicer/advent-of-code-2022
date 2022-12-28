use aoc22;
use std::collections::BTreeSet;

fn main() {
    let lines = aoc22::read_input_lines();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        let res: usize = chars
            .windows(4)
            .enumerate()
            .find(|(idx, window)| BTreeSet::from_iter(*window).len() == 4)
            .map(|(idx, _)| idx)
            .unwrap()
            + 4;
        println!("{}", res);
    }
}
