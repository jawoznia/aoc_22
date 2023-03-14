use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

#[derive(Debug)]
pub struct MarkerFinder(String);

#[derive(Debug)]
pub struct MarkerFinders(Vec<MarkerFinder>);

impl FromIterator<MarkerFinder> for MarkerFinders {
    fn from_iter<T: IntoIterator<Item = MarkerFinder>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl MarkerFinders {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let finders: Self = reader
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| MarkerFinder(l))
            .collect();

        Ok(finders)
    }

    pub fn find(&self) -> Vec<usize> {
        self.0
            .iter()
            .map(|mf| mf.find())
            .filter_map(|marker| marker)
            .collect()
    }
}

impl MarkerFinder {
    pub fn find(&self) -> Option<usize> {
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
    fn example() {
        let marker_finders = MarkerFinders::new("example.txt").unwrap();
        let markers = marker_finders.find();
        assert_eq!(markers, vec![5, 6, 10, 11]);
    }

    #[test]
    fn input() {
        let marker_finders = MarkerFinders::new("input.txt").unwrap();
        let markers = marker_finders.find();
        assert_eq!(markers, vec![1655]);
    }
}
