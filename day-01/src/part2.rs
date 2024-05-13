pub fn process(input: String) -> u32 {
    let valid_digit_spell = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    input
        .lines()
        .map(|line| {
            let first_char_digit = line.chars().enumerate().find(|(_, c)| c.is_digit(10));

            let mut first_char_speel: Option<(usize, char)> = None;
            for (str_val, c_val) in valid_digit_spell.iter() {
                let search = line.find(*str_val);
                if let Some(idx) = search {
                    let current_idx = first_char_speel.map(|(idx, _)| idx).unwrap_or(usize::MAX);
                    if idx <= current_idx {
                        first_char_speel = Some((idx, *c_val));
                    }
                }
            }

            let mut first_occurence: (usize, char) = (usize::MAX, char::default());
            if let Some((idx, val)) = first_char_digit {
                if idx <= first_occurence.0 {
                    first_occurence = (idx, val);
                }
            }
            if let Some((idx, val)) = first_char_speel {
                if idx <= first_occurence.0 {
                    first_occurence = (idx, val);
                }
            }

            let last_char_digit = line
                .chars()
                .rev()
                .enumerate()
                .find(|(_, c)| c.is_digit(10))
                .map(|(idx, val)| (line.len() - idx - 1, val));

            dbg!(last_char_digit);

            let mut last_char_speel: Option<(usize, char)> = None;
            for (str_val, c_val) in valid_digit_spell.iter() {
                let search = line.find(*str_val);
                if let Some(idx) = search {
                    let current_idx = last_char_speel.map(|(idx, _)| idx).unwrap_or(0);
                    if idx >= current_idx {
                        last_char_speel = Some((idx, *c_val));
                    }
                }
            }

            let mut last_occurence: (usize, char) = (0, char::default());
            if let Some((idx, val)) = last_char_digit {
                if idx >= last_occurence.0 {
                    last_occurence = (idx, val);
                }
            }
            if let Some((idx, val)) = last_char_speel {
                if idx >= last_occurence.0 {
                    last_occurence = (idx, val);
                }
            }

            dbg!(first_occurence);
            dbg!(last_occurence);
            (first_occurence.1, last_occurence.1)
        })
        .map(|(fo, lo)| {
            let mut res = String::with_capacity(2);
            res.push(fo);
            res.push(lo);
            res.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_test() -> std::io::Result<()> {
        let input = String::from("8czzpmvgmlchnkf");
        let output = process(input);
        assert_eq!(output, 281);
        Ok(())
    }

    #[test]
    fn test_process2() -> std::io::Result<()> {
        let input = std::fs::read_to_string("data/part2.txt")?;
        let output = process(input);
        assert_eq!(output, 281);
        Ok(())
    }

    #[test]
    fn test_real_process2() -> std::io::Result<()> {
        let input = std::fs::read_to_string("data/part1-real.txt")?;
        let output = process(input);
        assert_eq!(output, 54561);
        Ok(())
    }
}
