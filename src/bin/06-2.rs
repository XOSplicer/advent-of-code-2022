use aoc22;
use std::collections::BTreeSet;

fn main() {
    let lines = aoc22::read_input_lines();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        let marker_len = 14;
        let res: usize = chars
            .windows(marker_len)
            .enumerate()
            .find(|(idx, window)| BTreeSet::from_iter(*window).len() == marker_len)
            .map(|(idx, _)| idx)
            .unwrap()
            + marker_len;
        println!("{}", res);
    }
}
