use day_01::part1::process;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let input = File::open("data/part1-real.txt")?;
    let output = process(&input);
    dbg!(output);
    Ok(())
}
