const SYMBOLS: [char; 5] = ['*', '#', '+', '-', '$'];

fn is_symbols(value: char) -> bool {
    SYMBOLS.contains(&value)
}

fn is_beggining_of_numbers(value: char, state: Vec<Vec<char>>) {}

pub fn process(input: String) -> u32 {
    let number_lines = input.lines().count();

    let mut state = vec![vec!['🤯'; number_lines]; number_lines];

    // Fill in the 2D vector
    for (index1, line) in input.lines().enumerate() {
        for (index2, charactere) in line.chars().enumerate() {
            state[index1][index2] = charactere;
        }
    }

    // Found
    for (row, line) in input.lines().enumerate() {
        for (column, charactere) in line.chars().enumerate() {
            if charactere.is_digit(10) {
                //dbg!(charactere);
                // check for left
                if column != 0 {
                    let val = state[row][column - 1];
                    if is_symbols(val) {
                        dbg!(charactere);
                    }
                }
            }
        }
    }

    34
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> std::io::Result<()> {
        let input = std::fs::read_to_string("data/part1.txt")?;
        let output = process(input);
        assert_eq!(output, 8);
        Ok(())
    }

    #[test]
    fn test_real_process_1() -> std::io::Result<()> {
        let input = std::fs::read_to_string("data/part1-real.txt")?;
        let output = process(input);
        assert_eq!(output, 2632);
        Ok(())
    }
}
