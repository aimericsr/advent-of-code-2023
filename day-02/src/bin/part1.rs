use std::fs::read_to_string;

use day_02::part1::process;

fn main() -> std::io::Result<()> {
    let input = read_to_string("part1.txt")?;
    let output = process(input);
    dbg!(output);
    Ok(())
}
