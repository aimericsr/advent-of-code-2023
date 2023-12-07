pub fn process(input: &str) -> u32 {
    45
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1.txt");
        let output = process(input);
        assert_eq!(output, 13);
        Ok(())
    }

    #[test]
    fn test_real_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1-real.txt");
        let output = process(input);
        assert_eq!(output, 21568);
        Ok(())
    }
}
