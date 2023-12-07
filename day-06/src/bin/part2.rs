use day_06::part1::process;

fn main() -> std::io::Result<()> {
    let input = include_str!("../../data/part1.txt");
    let output = process(input);
    dbg!(output);
    Ok(())
}
