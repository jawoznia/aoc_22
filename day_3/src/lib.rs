use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}
type Rucksacks = Vec<Rucksack>;

impl From<(Vec<char>, Vec<char>)> for Rucksack {
    fn from(c: (Vec<char>, Vec<char>)) -> Self {
        Rucksack {
            left: c.0,
            right: c.1,
        }
    }
}

pub fn load_data(file: &str) -> Result<Rucksacks> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let rucksacks: Rucksacks = reader
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
    Ok(rucksacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_data() {
        let rucksacks = load_data("example.txt").unwrap();
        println!("RS => {:#?}", rucksacks);
    }
}
