use anyhow;
use aoc22;

struct RangePair {
    start_0: u32,
    end_incl_0: u32,
    start_1: u32,
    end_incl_1: u32,
}

impl RangePair {
    fn from_line(line: &str) -> RangePair {
        let mut parts = line.split(&['-', ','][..]);
        RangePair {
            start_0: parts.next().unwrap().parse().unwrap(),
            end_incl_0: parts.next().unwrap().parse().unwrap(),
            start_1: parts.next().unwrap().parse().unwrap(),
            end_incl_1: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn is_fully_contained(&self) -> bool {
        (self.start_0 <= self.start_1 && self.end_incl_1 <= self.end_incl_0)
            || (self.start_1 <= self.start_0 && self.end_incl_0 <= self.end_incl_1)
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc22::read_input_lines();
    let res = lines
        .map(|line| RangePair::from_line(&line))
        .filter(|pair| pair.is_fully_contained())
        .count();
    println!("{}", res);
    Ok(())
}
