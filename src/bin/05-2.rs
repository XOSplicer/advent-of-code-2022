use aoc22;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(line: &str) -> Move {
        let mut parts = line.split_whitespace();
        parts.next();
        let amount = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from = parts.next().unwrap().parse().unwrap();
        parts.next();
        let to = parts.next().unwrap().parse().unwrap();
        Move { amount, from, to }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn apply_move(&mut self, m: &Move) {
        let mut popped = Vec::new();
        for _ in 0..m.amount {
            let item = self.0[m.from - 1].pop().unwrap();
            popped.push(item);
        }
        popped.reverse();
        self.0[m.to - 1].extend_from_slice(&popped)
    }
    fn tops(&self) -> String {
        self.0.iter().filter_map(|v| v.last()).collect()
    }
}

fn main() {
    let lines = aoc22::read_input_lines();
    let mut crate_lines = Vec::new();
    let mut num_stacks: usize = 0;
    let mut moves = Vec::new();
    for line in lines {
        if line.contains('[') {
            crate_lines.push(line);
        } else if line.contains("move") {
            moves.push(Move::from_line(&line));
        } else if line.contains("1") {
            num_stacks = line
                .split_whitespace()
                .map(|part| part.parse().unwrap())
                .max()
                .unwrap();
        }
    }
    println!("{}", num_stacks);

    let mut stacks = vec![Vec::new(); num_stacks];
    for n in 0..num_stacks {
        for line in crate_lines.iter().rev() {
            let idx = 4 * n + 1;
            if let Some(c) = line.chars().nth(idx) {
                if c.is_alphabetic() {
                    stacks[n].push(c);
                } else {
                    break;
                }
            }
        }
    }
    let mut stacks = Stacks(stacks);
    println!("{:?}", stacks);

    for m in moves {
        stacks.apply_move(&m)
    }
    println!("{:?}", stacks);

    let tops = stacks.tops();
    println!("{}", tops);
}
