use anyhow;
use aoc22;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => self.0 as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.0 as u32 - 'A' as u32 + 27,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack {
    inner: BTreeSet<Item>,
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        let inner = line.chars().map(Item).collect();
        Rucksack { inner }
    }
}

struct Group<'a>(&'a [Rucksack] /* len: 3 */);

impl<'a> Group<'a> {
    fn intersection_priority(&self) -> u32 {
        let intersection_0_1: BTreeSet<Item> = self.0[0]
            .inner
            .intersection(&self.0[1].inner)
            .cloned()
            .collect();
        intersection_0_1
            .intersection(&self.0[2].inner)
            .map(|item| item.priority())
            .next()
            .unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc22::read_input_lines();
    let rucksacks: Vec<Rucksack> = lines.map(|line| Rucksack::from_line(&line)).collect();
    let res: u32 = rucksacks
        .chunks(3)
        .map(|c| Group(c).intersection_priority())
        .sum();
    println!("{}", res);
    Ok(())
}
