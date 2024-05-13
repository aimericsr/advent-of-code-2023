use day_03::part1::process;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("../../data/part1.txt")?;
    let output = process(input);
    dbg!(output);
    Ok(())
}
