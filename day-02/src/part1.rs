use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            sets: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Set) -> Option<Ordering> {
        if self.blue == other.blue && self.red == other.red && self.green == other.green {
            return Some(Ordering::Equal);
        }
        if self.blue >= other.blue && self.red >= other.red && self.green >= other.green {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Less)
    }
}

impl Game {
    fn get_set_with_most_cubes(self) -> Set {
        let mut bigest_set = Set::default();

        for set in self.sets.into_iter() {
            if set.blue > bigest_set.blue {
                bigest_set.blue = set.blue;
            }
            if set.green > bigest_set.green {
                bigest_set.green = set.green;
            }
            if set.red > bigest_set.red {
                bigest_set.red = set.red;
            }
        }
        bigest_set
    }
}

pub fn process(input: &File) -> u32 {
    let reader = BufReader::new(input);
    let mut games: Vec<Game> = Vec::new();

    let mut current_game = Game::default();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if !l.is_empty() {
                    let line_split: Vec<&str> = l.split(':').collect();

                    current_game.id = line_split[0]
                        .matches(char::is_numeric)
                        .collect::<Vec<&str>>()
                        .join("")
                        .parse()
                        .unwrap();

                    let sets: Vec<&str> = line_split[1].split(';').collect();

                    for set in sets.into_iter() {
                        let mut current_set = Set::default();
                        let sets: Vec<&str> = set.split(',').collect();
                        for i in sets.into_iter() {
                            for color in vec!["blue", "red", "green"] {
                                if i.contains(color) {
                                    let current_cubes: u32 = i
                                        .matches(char::is_numeric)
                                        .collect::<Vec<&str>>()
                                        .join("")
                                        .parse()
                                        .unwrap();

                                    match color {
                                        "blue" => {
                                            if current_cubes > current_set.blue {
                                                current_set.blue = current_cubes;
                                            }
                                        }
                                        "red" => {
                                            if current_cubes > current_set.red {
                                                current_set.red = current_cubes;
                                            }
                                        }
                                        "green" => {
                                            if current_cubes > current_set.green {
                                                current_set.green = current_cubes;
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                        current_game.sets.push(current_set);
                    }

                    games.push(current_game);
                    current_game = Game::default();
                }
            }
            Err(_) => println!("Error reading the line"),
        }
    }

    let mut score: u32 = 0;

    let maximum_set = Set {
        blue: 14,
        green: 13,
        red: 12,
    };

    for game in games.into_iter() {
        let current_set = game.clone().get_set_with_most_cubes();

        if maximum_set >= current_set {
            score += game.id;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> std::io::Result<()> {
        let input = File::open("data/part1.txt")?;
        let output = process(&input);
        assert_eq!(output, 8);
        Ok(())
    }

    #[test]
    fn test_real_process_1() -> std::io::Result<()> {
        let input = File::open("data/part1-real.txt")?;
        let output = process(&input);
        assert_eq!(output, 2632);
        Ok(())
    }
}
