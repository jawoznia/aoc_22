use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

enum Outcome {
    Lose,
    Draw,
    Win,
}

struct Round {
    shape_bonus: u32,
    outcome: Outcome,
}

impl Round {
    fn calc_score(&self) -> u32 {
        self.shape_bonus + self.outcome.score()
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Outcome {
    fn new(s: &str) -> Self {
        if s.ends_with('X') {
            Outcome::Lose
        } else if s.ends_with('Y') {
            Outcome::Draw
        } else {
            Outcome::Win
        }
    }
}

impl Round {
    fn new(s: &str) -> Self {
        let outcome = Outcome::new(s);
        let shape_bonus = Round::calc_shape_bonus(s, &outcome);
        Self {
            shape_bonus,
            outcome,
        }
    }

    fn calc_shape_bonus(s: &str, outcome: &Outcome) -> u32 {
        match outcome {
            Outcome::Lose => {
                if s.starts_with('A') {
                    3
                } else if s.starts_with('B') {
                    1
                } else {
                    2
                }
            }
            Outcome::Draw => {
                if s.starts_with('A') {
                    1
                } else if s.starts_with('B') {
                    2
                } else {
                    3
                }
            }
            Outcome::Win => {
                if s.starts_with('A') {
                    2
                } else if s.starts_with('B') {
                    3
                } else {
                    1
                }
            }
        }
    }
}

pub fn calc_score(file: &str) -> Result<u32> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut score = 0;
    reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Round::new(l.as_str()))
        .for_each(|r| score += r.calc_score());

    Ok(score)
}

// ================================
// First part
pub fn load_data(file: &str) -> Result<u32> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut score = 0;
    reader
        .lines()
        .filter_map(|l| l.ok())
        .for_each(|l| score += round_score(&l));
    Ok(score)
}

fn round_score(line: &str) -> u32 {
    if line.starts_with('A') {
        if line.ends_with('X') {
            return 4;
        } else if line.ends_with('Y') {
            return 8;
        } else {
            return 3;
        }
    } else if line.starts_with('B') {
        if line.ends_with('X') {
            return 1;
        } else if line.ends_with('Y') {
            return 5;
        } else {
            return 9;
        }
    } else {
        if line.ends_with('X') {
            return 7;
        } else if line.ends_with('Y') {
            return 2;
        } else {
            return 6;
        }
    }
}
// ===================================

#[cfg(test)]
mod tests {
    use crate::{calc_score, load_data};

    #[test]
    fn test_data() {
        let score = load_data("test_data.txt").unwrap();
        assert_eq!(score, 15);
    }

    #[test]
    fn test_input() {
        let score = load_data("input.txt").unwrap();
        assert_eq!(score, 10718)
    }

    #[test]
    fn test_data_second_part() {
        let score = calc_score("test_data.txt").unwrap();
        assert_eq!(score, 12);
    }

    #[test]
    fn test_input_second_part() {
        let score = calc_score("input.txt").unwrap();
        assert_eq!(score, 14652)
    }
}
