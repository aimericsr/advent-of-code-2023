pub fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let time = lines
        .get(0)
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    dbg!(&time);

    let distance_max = lines
        .get(1)
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut res = 0_u32;
    for hold_time in 0..time + 1 {
        let speed = hold_time;
        let remaining_time = time - hold_time;
        let distance = speed * remaining_time;
        if distance > distance_max {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_2() -> std::io::Result<()> {
        let input = include_str!("../data/part1.txt");
        let output = process(input);
        assert_eq!(output, 71503);
        Ok(())
    }

    #[test]
    fn test_real_process_2() -> std::io::Result<()> {
        let input = include_str!("../data/part1-real.txt");
        let output = process(input);
        assert_eq!(output, 40651271);
        Ok(())
    }
}
