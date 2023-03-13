use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct SectionPair {
    pub left: (u32, u32),
    pub right: (u32, u32),
}

impl SectionPair {
    pub fn overlaps(&self) -> bool {
        let SectionPair { left, right } = self;
        left.1 <= right.0 && right.1 <= left.0
    }
}

pub fn load_data(file: &str) -> Result<Vec<SectionPair>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let pair: Vec<_> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| -> Result<_> {
            let borders: Vec<_> = l
                .split([',', '-'])
                .map(|s| s.trim().parse::<u32>())
                .filter_map(|s| s.ok())
                .collect();
            assert_eq!(borders.len(), 4);

            Ok(SectionPair {
                left: (borders[0], borders[1]),
                right: (borders[2], borders[3]),
            })
        })
        .filter_map(|l| l.ok())
        .collect();

    Ok(pair)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let _sections = load_data("example.txt").unwrap();
    }
}
