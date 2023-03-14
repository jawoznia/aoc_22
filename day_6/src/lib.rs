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
            .map(|l| Detector(l))
            .collect();

        Ok(detectors)
    }

    pub fn find_markers(&self) -> Vec<usize> {
        self.0
            .iter()
            .map(|mf| mf.find_marker())
            .filter_map(|marker| marker)
            .collect()
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
}
