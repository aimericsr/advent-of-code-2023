use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, Clone, Debug, EnumIter)]
#[strum(serialize_all = "snake_case")]
enum ValidSpelledDigit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<&str> for ValidSpelledDigit {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "one" => Ok(ValidSpelledDigit::One),
            "two" => Ok(ValidSpelledDigit::Two),
            "three" => Ok(ValidSpelledDigit::Three),
            "four" => Ok(ValidSpelledDigit::Four),
            "five" => Ok(ValidSpelledDigit::Five),
            "six" => Ok(ValidSpelledDigit::Six),
            "seven" => Ok(ValidSpelledDigit::Seven),
            "eight" => Ok(ValidSpelledDigit::Eight),
            "nine" => Ok(ValidSpelledDigit::Nine),
            _ => Err("Not a valid spelled digit"),
        }
    }
}

impl From<ValidSpelledDigit> for u32 {
    fn from(value: ValidSpelledDigit) -> Self {
        match value {
            ValidSpelledDigit::One => 1,
            ValidSpelledDigit::Two => 2,
            ValidSpelledDigit::Three => 3,
            ValidSpelledDigit::Four => 4,
            ValidSpelledDigit::Five => 5,
            ValidSpelledDigit::Six => 6,
            ValidSpelledDigit::Seven => 7,
            ValidSpelledDigit::Eight => 8,
            ValidSpelledDigit::Nine => 9,
        }
    }
}

pub fn process(input: &File) -> u32 {
    let reader = BufReader::new(input);
    let mut calibration: Vec<u32> = Vec::new();

    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if !l.is_empty() {
                    // check for digit number
                    for c in l.chars() {
                        if c.is_digit(10) {
                            if first_digit.is_none() {
                                first_digit = Some(c.into());
                            } else {
                                last_digit = Some(c.into());
                            }
                        }
                    }

                    // check for digit written in string
                    // Loop through the custom enum
                    // for valid_digit in ValidSpelledDigit::iter() {
                    //     if let Some(i) = l.find(&valid_digit.to_string()) {
                    //         // if the first digit is already set
                    //         if let Some(digit) = first_digit {
                    //             dbg!(digit.clone());
                    //             dbg!(l.clone());
                    //             let digit_index = l.find(char::from_u32(digit).unwrap()).unwrap();

                    //             if i < digit_index {
                    //                 first_digit = Some(valid_digit.clone().into());
                    //             }
                    //         } else {
                    //             first_digit = Some(valid_digit.clone().into());
                    //         }

                    //         // if the last digit is already set
                    //         if let Some(digit) = last_digit {
                    //             let digit_index = l.find(char::from_u32(digit).unwrap()).unwrap();

                    //             if i > digit_index {
                    //                 last_digit = Some(valid_digit.clone().into());
                    //             }
                    //         } else {
                    //             last_digit = Some(valid_digit.into());
                    //         }
                    //     }
                    // }
                }
            }
            Err(_) => println!("Error reading the line"),
        }
        // Calculate the sum
        let mut res: String = "".to_string();
        if last_digit.is_none() {
            res.insert(0, char::from_u32(first_digit.unwrap()).unwrap());
            res.insert(1, char::from_u32(first_digit.unwrap()).unwrap());
        } else {
            res.insert(0, char::from_u32(first_digit.unwrap()).unwrap());
            res.insert(1, char::from_u32(last_digit.unwrap()).unwrap());
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
    fn test_display_enum() -> std::io::Result<()> {
        let l = "dsf".to_string();
        // let tets = ValidSpelledDigit::try_from(l.clone()).unwrap();
        // let res: ValidSpelledDigit = l.try_into().unwrap();

        let enum_test = ValidSpelledDigit::One;
        assert_eq!(enum_test.to_string(), "one");
        Ok(())
    }

    #[test]
    fn test_process2() -> std::io::Result<()> {
        let input = File::open("data/part2.txt")?;
        let output = process(&input);
        assert_eq!(output, 281);
        Ok(())
    }

    // #[test]
    // fn test_real_process() -> std::io::Result<()> {
    //     let input = File::open("data/part1-real.txt")?;
    //     let output = process(&input);
    //     assert_eq!(output, 54561);
    //     Ok(())
    // }
}
