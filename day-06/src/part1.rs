use std::{collections::HashMap, iter::zip};

pub fn process(input: &str) -> u32 {
    let mut time_for_distance: HashMap<u32, u32> = HashMap::new();

    let lines = input.lines().collect::<Vec<&str>>();

    let first_line = lines
        .get(0)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let second_line = lines
        .get(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let zipped = zip(
        first_line.into_iter().enumerate(),
        second_line.into_iter().enumerate(),
    );

    for (first_line, second_line) in zipped.into_iter() {
        if first_line.0 != 0 && second_line.0 != 0 {
            time_for_distance.insert(
                first_line.1.parse::<u32>().unwrap(),
                second_line.1.parse::<u32>().unwrap(),
            );
        }
    }

    let mut numbers_of_possible_wins: Vec<u32> = Vec::new();
    for (index, key_values) in time_for_distance.into_iter().enumerate() {
        let mut current_wins = 0_u32;
        for hold_time in 0..key_values.0 + 1 {
            let speed = hold_time;
            let remaining_time = key_values.0 - hold_time;
            let distance = speed * remaining_time;
            if distance > key_values.1 {
                current_wins += 1;
            }
            dbg!(distance);
        }
        numbers_of_possible_wins.insert(index, current_wins);
    }
    numbers_of_possible_wins.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1.txt");
        let output = process(input);
        assert_eq!(output, 289);
        Ok(())
    }

    #[test]
    fn test_real_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1-real.txt");
        let output = process(input);
        assert_eq!(output, 5133600);
        Ok(())
    }
}
