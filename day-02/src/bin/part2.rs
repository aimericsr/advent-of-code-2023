use std::fs::File;

use day_02::part1::process;

fn main() -> std::io::Result<()> {
    let input = File::open("src/part1.txt")?;
    let output = process(&input);
    dbg!(output);
    Ok(())
}
