use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

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

#[cfg(test)]
mod tests {
    use crate::load_data;

    #[test]
    fn test_data() {
        let score = load_data("test_data.txt").unwrap();
        assert_eq!(score, 15);
    }

    #[test]
    fn test_first_input() {
        let score = load_data("input.txt").unwrap();
        assert_eq!(score, 10718)
    }
}
