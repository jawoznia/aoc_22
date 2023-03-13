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

pub struct Sections(Vec<SectionPair>);

impl SectionPair {
    /// Checks if one of the sections contains the other one
    pub fn contains(&self) -> bool {
        let SectionPair { left, right } = self;
        left.0 <= right.0 && right.1 <= left.1 || left.0 >= right.0 && right.1 >= left.1
    }

    /// Checks if pairs overlap
    pub fn overlaps(&self) -> bool {
        let SectionPair { left, right } = self;
        left.0 <= right.1 && right.0 <= left.1
    }
}

impl FromIterator<SectionPair> for Sections {
    fn from_iter<T: IntoIterator<Item = SectionPair>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Sections {
    pub fn count_intersections(&self) -> usize {
        self.0.iter().filter(|s| s.contains()).count()
    }

    pub fn count_overlaps(&self) -> usize {
        self.0.iter().filter(|s| s.overlaps()).count()
    }
}

pub fn load_data(file: &str) -> Result<Sections> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let pair: Sections = reader
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
    fn example_1() {
        let sections = load_data("example.txt").unwrap();
        assert_eq!(sections.count_intersections(), 2)
    }

    #[test]
    fn test_1() {
        let sections = load_data("test_1.txt").unwrap();
        assert_eq!(sections.count_intersections(), 459)
    }

    #[test]
    fn example_2() {
        let sections = load_data("example.txt").unwrap();
        assert_eq!(sections.count_overlaps(), 4)
    }

    #[test]
    fn test_2() {
        let sections = load_data("test_1.txt").unwrap();
        assert_eq!(sections.count_overlaps(), 779)
    }
}
