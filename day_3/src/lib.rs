use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rucksack {
    left: String,
    right: String,
}
type Rucksacks = Vec<Rucksack>;

impl From<(String, String)> for Rucksack {
    fn from(c: (String, String)) -> Self {
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
            (l, r).into()
        })
        .collect();
    Ok(rucksacks)
}
