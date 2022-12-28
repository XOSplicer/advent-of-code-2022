use anyhow;
use aoc22;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;

impl std::cmp::PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (&Rock, &Rock) => Ordering::Equal,
            (&Rock, &Paper) => Ordering::Less,
            (&Rock, &Scissors) => Ordering::Greater,
            (&Paper, &Rock) => Ordering::Greater,
            (&Paper, &Paper) => Ordering::Equal,
            (&Paper, &Scissors) => Ordering::Less,
            (&Scissors, &Paper) => Ordering::Greater,
            (&Scissors, &Rock) => Ordering::Less,
            (&Scissors, &Scissors) => Ordering::Equal,
        })
    }
}

impl std::cmp::Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        Shape::partial_cmp(&self, other).unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Round {
    opponent: Shape,
    me: Shape,
}

impl Round {
    fn score(&self) -> u32 {
        let my_score: u32 = match self.me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let outcome_score: u32 = match self.me.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        my_score + outcome_score
    }

    fn from_line(line: &str) -> Round {
        let mut parts = line.split_whitespace();
        let opponent = match parts.next().expect("Could not parse line") {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Unexpected input"),
        };
        let me = match parts.next().expect("Could not parse line") {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unexpected input"),
        };
        Round { opponent, me }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc22::read_input_lines();
    let score: u32 = lines.map(|line| Round::from_line(&line).score()).sum();
    println!("{}", score);
    Ok(())
}
