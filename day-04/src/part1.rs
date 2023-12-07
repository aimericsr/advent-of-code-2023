use tracing::info;

// impl Value for Vec<u32> {
//     fn record(&self, key: &tracing::field::Field, visitor: &mut dyn tracing::field::Visit) {
//         // Convert the Vec<u32> to a string for recording purposes
//         let values_as_string: Vec<String> = self.iter().map(|&num| num.to_string()).collect();

//         // Record the value in the visitor associated with the key
//         visitor.record_u32_array(key, &values_as_string);
//     }
// }

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let card_id: Vec<&str> = line.split(':').collect();
            let numbers = card_id.get(1).expect("No numbers found");
            let card_parts: Vec<&str> = numbers.split('|').collect();

            let winning_numbers: Vec<u32> = card_parts
                .get(0)
                .expect("There should be a list of numbers")
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let my_numbers: Vec<u32> = card_parts
                .get(1)
                .expect("There should be a list of numbers")
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            //let test = winning_numbers.to
            //info!(winning_numbers.to_string());
            winning_numbers
                .iter()
                .flat_map(|w_num| my_numbers.iter().map(move |my_num| (w_num, my_num)))
                .fold(0, |acc, (&w_num, &my_num)| {
                    //info!("Acc / Pair: ({} / {}, {})", acc, w_num, my_num);
                    if my_num == w_num {
                        if acc == 0 {
                            1
                        } else {
                            acc * 2
                        }
                    } else {
                        acc
                    }
                })
        })
        .sum()

    // let mut total_scores = 0;
    // for line in input.lines() {
    //     let card_id: Vec<&str> = line.split(':').collect();
    //     let numbers = card_id.get(1).expect("No numbers found");
    //     let card_parts: Vec<&str> = numbers.split('|').collect();
    //     let winning_numbers = card_parts
    //         .get(0)
    //         .expect("There should be a list of numbers")
    //         .split(' ')
    //         .filter(|x| !x.is_empty())
    //         .map(|x| x.parse::<u32>().unwrap())
    //         .collect::<Vec<u32>>();
    //     let my_numbers = card_parts
    //         .get(1)
    //         .expect("There should be a list of numbers")
    //         .split(' ')
    //         .filter(|x| !x.is_empty())
    //         .map(|x| x.parse::<u32>().unwrap())
    //         .collect::<Vec<u32>>();

    //     let mut current_score = 0;
    //     for w_num in winning_numbers.into_iter() {
    //         for my_num in my_numbers.clone().into_iter() {
    //             if my_num == w_num {
    //                 if current_score == 0 {
    //                     current_score = 1;
    //                 } else {
    //                     current_score *= 2;
    //                 }
    //             }
    //         }
    //     }
    //     total_scores += current_score;
    // }
    // total_scores
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
