use std::fs;

use day_01::part1::process;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/part1.txt")?;
    let output = process(input);
    dbg!(output);
    Ok(())
}
