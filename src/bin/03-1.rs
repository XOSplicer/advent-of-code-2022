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
    left: BTreeSet<Item>,
    right: BTreeSet<Item>,
}

impl Rucksack {
    fn intersected_items(&self) -> impl Iterator<Item = &Item> {
        self.left.intersection(&self.right)
    }
    fn intersection_priority(&self) -> u32 {
        self.intersected_items().next().unwrap().priority()
    }
    fn from_line(line: &str) -> Self {
        let (left, right) = line.split_at(line.len() / 2);
        let left = left.chars().map(Item).collect();
        let right = right.chars().map(Item).collect();
        Rucksack { left, right }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc22::read_input_lines();
    let res: u32 = lines.map(|line| Rucksack::from_line(&line).intersection_priority()).sum();
    println!("{}", res);
    Ok(())
}