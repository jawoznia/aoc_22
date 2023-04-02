use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
pub enum Operation {
    Add(u32),
    Multiply(u32),
    Power,
}

impl Operation {
    pub fn new((op, right): (&str, &str)) -> Self {
        let right = if right == "old" {
            None
        } else {
            Some(right.parse::<u32>().unwrap())
        };
        if op == "+" {
            if let Some(right) = right {
                Self::Add(right)
            } else {
                Self::Multiply(2)
            }
        } else if let Some(right) = right {
            Self::Multiply(right)
        } else {
            Self::Power
        }
    }
}

#[derive(Debug)]
pub struct Test {
    pub divisor: u32,
    pub matched_receiver: u32,
    pub unmatched_receiver: u32,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u32>,
    pub operation: Operation,
    pub test: Test,
}

#[derive(Debug)]
pub struct Monkeys(Vec<Monkey>);

impl FromIterator<Monkey> for Monkeys {
    fn from_iter<I: IntoIterator<Item = Monkey>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Monkeys {
    pub fn new(file: &str) -> Result<Self> {
        let monkeys: Monkeys = read_to_string(file)?
            .lines()
            .tuples()
            .map(|(_, items, op, test, matched, unmatched, _)| {
                let items = items
                    .split_whitespace()
                    .skip(2)
                    .map(|item| item.replace(',', "").parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let mut operation = op
                    .split_whitespace()
                    .skip(4)
                    .tuples()
                    .map(|(op, right)| Operation::new((op, right)))
                    .collect::<Vec<_>>();
                assert_eq!(operation.len(), 1);
                let operation = operation.pop().unwrap();

                let divisor = test
                    .split_whitespace()
                    .nth(3)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let matched_receiver = matched
                    .split_whitespace()
                    .nth(5)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let unmatched_receiver = unmatched
                    .split_whitespace()
                    .nth(5)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let test = Test {
                    divisor,
                    matched_receiver,
                    unmatched_receiver,
                };

                Monkey {
                    items,
                    operation,
                    test,
                }
            })
            .collect();

        Ok(monkeys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let monkey = Monkeys::new("example.txt").unwrap();
        println!("{:#?}", monkey);
    }
    #[test]
    fn input() {
        let _monkey = Monkeys::new("input.txt").unwrap();
    }
}
