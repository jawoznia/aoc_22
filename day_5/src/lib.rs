use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct Move {
    pub count: u32,
    pub from: u32,
    pub to: u32,
}

#[derive(Debug)]
pub struct Storage {
    pub stacks: Vec<VecDeque<char>>,
    pub moves: Vec<Move>,
}

impl Storage {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>();
        let no_stacks = lines[0].len() / 4 + 1;
        let mut storage = Self {
            stacks: vec![VecDeque::new(); no_stacks],
            moves: vec![],
        };

        lines.iter().filter(|l| l.contains('[')).for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .zip(storage.stacks.iter_mut())
                .filter(|(letter, _)| letter != &' ')
                .for_each(|(letter, stack)| {
                    stack.push_front(letter);
                })
        });

        lines.iter().filter(|l| l.contains('m')).for_each(|l| {
            let Some((count, from, to)) = l.split(' ').skip(1).step_by(2).map(|s| s.parse::<u32>()).filter_map(|s| s.ok()).collect_tuple() else {
                panic!("Unexpected format of move commands: {}", l);
            };
            storage.moves.push(Move { count, from, to })
        });

        Ok(storage)
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
