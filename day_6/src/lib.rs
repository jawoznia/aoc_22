use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

#[derive(Debug)]
pub struct Detector(String);

#[derive(Debug)]
pub struct Detectors(Vec<Detector>);

impl FromIterator<Detector> for Detectors {
    fn from_iter<T: IntoIterator<Item = Detector>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Detectors {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let detectors: Self = reader
            .lines()
            .filter_map(|l| l.ok())
            .map(Detector)
            .collect();

        Ok(detectors)
    }

    pub fn find_markers(&self) -> Vec<usize> {
        self.0.iter().filter_map(|mf| mf.find_marker()).collect()
    }

    pub fn find_messages(&self) -> Vec<usize> {
        self.0.iter().filter_map(|mf| mf.find_message()).collect()
    }
}

impl Detector {
    pub fn find_marker(&self) -> Option<usize> {
        for top in 4..self.0.len() {
            let bottom = top - 4;

            if self.0[bottom..top].chars().all_unique() {
                return Some(top);
            }
        }
        None
    }

    pub fn find_message(&self) -> Option<usize> {
        assert!(self.0.len() >= 14);
        for top in 14..self.0.len() {
            let bottom = top - 14;

            if self.0[bottom..top].chars().all_unique() {
                return Some(top);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_marker() {
        let detectors = Detectors::new("example.txt").unwrap();
        let markers = detectors.find_markers();
        assert_eq!(markers, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn input_marker() {
        let detectors = Detectors::new("input.txt").unwrap();
        let markers = detectors.find_markers();
        assert_eq!(markers, vec![1655]);
    }

    #[test]
    fn example_message() {
        let detectors = Detectors::new("example.txt").unwrap();
        let markers = detectors.find_messages();
        assert_eq!(markers, vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn input_message() {
        let detectors = Detectors::new("input.txt").unwrap();
        let markers = detectors.find_messages();
        assert_eq!(markers, vec![2665]);
    }
}
