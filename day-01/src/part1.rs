pub fn process(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_char = line
                .chars()
                .find(|c| c.is_digit(10))
                .expect("There should be a digit");
            let last_char = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .expect("There should be a digit");

            let mut res = String::with_capacity(2);
            res.push(first_char);
            res.push(last_char);
            res.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process() -> std::io::Result<()> {
        let input = fs::read_to_string("data/part1.txt")?;
        let output = process(input);
        assert_eq!(output, 142);
        Ok(())
    }

    #[test]
    fn test_real_process() -> std::io::Result<()> {
        let input = fs::read_to_string("data/part1-real.txt")?;
        let output = process(input);
        assert_eq!(output, 54561);
        Ok(())
    }
}
