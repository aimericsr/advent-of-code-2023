use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input: &File) -> u32 {
    let reader = BufReader::new(input);
    let mut calibration: Vec<u32> = Vec::new();

    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if !l.is_empty() {
                    for c in l.chars() {
                        if c.is_digit(10) {
                            if first_digit.is_none() {
                                first_digit = Some(c);
                            } else {
                                last_digit = Some(c);
                            }
                        }
                    }
                }
            }
            Err(_) => println!("Error reading the line"),
        }
        // Calculate the sum
        let mut res: String = "".to_string();
        if last_digit.is_none() {
            res.insert(0, first_digit.unwrap());
            res.insert(1, first_digit.unwrap());
        } else {
            res.insert(0, first_digit.unwrap());
            res.insert(1, last_digit.unwrap());
        }
        first_digit = None;
        last_digit = None;
        calibration.push(res.parse::<u32>().unwrap());
    }

    let mut total: u32 = 0;
    for calibration in calibration.into_iter() {
        total += calibration;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> std::io::Result<()> {
        let input = File::open("data/part1.txt")?;
        let output = process(&input);
        assert_eq!(output, 142);
        Ok(())
    }

    #[test]
    fn test_real_process() -> std::io::Result<()> {
        let input = File::open("data/part1-real.txt")?;
        let output = process(&input);
        assert_eq!(output, 54561);
        Ok(())
    }
}
