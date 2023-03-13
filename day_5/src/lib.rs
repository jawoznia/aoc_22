use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

pub struct Move {
    pub count: u32,
    pub from: u32,
    pub to: u32,
}

#[derive(Debug)]
pub struct Storage(Vec<VecDeque<char>>);

impl Storage {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>();
        let no_stacks = lines[0].len() / 4 + 1;
        let mut storage = Self(vec![VecDeque::new(); no_stacks]);

        lines.iter().filter(|l| l.contains('[')).for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .zip(storage.0.iter_mut())
                .for_each(|(letter, stack)| {
                    if letter != ' ' {
                        stack.push_front(letter);
                    }
                })
        });

        println!("storage: {:#?}", storage);
        println!("no_stacks = {}", no_stacks);
        // Ok(Self(rucksacks))
        Ok(Self(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Storage::new("example.txt").unwrap();
    }
}
