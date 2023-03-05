pub mod second;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}
pub struct Rucksacks(Vec<Rucksack>);

impl From<(Vec<char>, Vec<char>)> for Rucksack {
    fn from(c: (Vec<char>, Vec<char>)) -> Self {
        Rucksack {
            left: c.0,
            right: c.1,
        }
    }
}

impl Rucksack {
    pub fn find_duplication(&self) -> Option<char> {
        assert_eq!(self.left.len(), self.right.len());
        let mut i_l = 0;
        let mut i_r = 0;
        let len = self.left.len();

        while i_l < len && i_r < len {
            match self.left[i_l].cmp(&self.right[i_r]) {
                std::cmp::Ordering::Less => i_l += 1,
                std::cmp::Ordering::Equal => return Some(self.left[i_l]),
                std::cmp::Ordering::Greater => i_r += 1,
            }
        }
        None
    }
}

impl Rucksacks {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let rucksacks: Vec<Rucksack> = reader
            .lines()
            .filter_map(|l| l.ok())
            .map(|mut l| {
                let r = l.split_off(l.len() / 2);

                let mut l: Vec<_> = l.chars().collect();
                let mut r: Vec<_> = r.chars().collect();
                r.sort();
                l.sort();

                (l, r).into()
            })
            .collect();
        Ok(Self(rucksacks))
    }

    // First solution
    pub fn calc_prio(&self) -> u32 {
        let uppercase_a = 'A' as u32 - 1;
        let lowercase_a = 'a' as u32 - 1;

        self.0
            .iter()
            .filter_map(|rs| rs.find_duplication())
            .map(|c| {
                if c >= 'a' {
                    c as u32 - lowercase_a
                } else {
                    c as u32 - uppercase_a + 26
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_data() {
        let rs = Rucksacks::new("example.txt").unwrap();
        assert_eq!(rs.calc_prio(), 157);
    }

    #[test]
    fn first() {
        let rs = Rucksacks::new("first.txt").unwrap();
        assert_eq!(rs.calc_prio(), 7903);
    }
}
