pub fn process(input: &str) -> u32 {
    let number_lines = input.matches("\n").collect::<Vec<&str>>().len() + 1;

    let mut state = vec![vec!['🤯'; number_lines]; number_lines];

    dbg!(&state);

    // Fill in the 2D array
    for (index1, line) in input.lines().enumerate() {
        for (index2, charactere) in line.chars().enumerate() {
            state[index1][index2] = charactere;
        }
    }

    let symbols = vec!['*', '#', '+', '-', '$'];
    // Found
    for (index1, line) in input.lines().enumerate() {
        for (index2, charactere) in line.chars().enumerate() {
            if charactere.is_ascii_digit() {
                // check for left
                if let Some(c) = state.get(index1 - 1) {
                    if let Some(c2) = c.get(index2) {

                    }
                }
                //if state[][]
            }
        }
    }

    dbg!(state);
    34
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1.txt");
        let output = process(input);
        assert_eq!(output, 8);
        Ok(())
    }

    #[test]
    fn test_real_process_1() -> std::io::Result<()> {
        let input = include_str!("../data/part1-real.txt");
        let output = process(input);
        assert_eq!(output, 2632);
        Ok(())
    }
}
