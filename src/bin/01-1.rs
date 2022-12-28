use anyhow;

use aoc22;

fn main() -> anyhow::Result<()> {
    let mut lines = aoc22::read_input_lines();
    let mut parsed: Vec<Vec<u32>> = Vec::new();
    parsed.push(Vec::new());
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line != "" {
            let n: u32 = line.parse()?;
            parsed.last_mut().unwrap().push(n);
        } else {
            parsed.push(Vec::new());
        }
    }
    println!("{:?}", &parsed);
    let max: u32 = parsed.iter().map(|v| v.iter().sum()).max().expect("No max found");
    println!("{}", max);

    Ok(())
}