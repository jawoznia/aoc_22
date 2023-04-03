use std::{cell::RefCell, fs::read_to_string};

use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonkeyError {
    #[error("Wrong divisor format")]
    WrongDivisorFormat,
    #[error("Wrong matched monkey identifier")]
    WrongMatchedMonkeyIdentifier,
    #[error("Wrong unmatched monkey identifier")]
    WrongUnmatchedMonkeyIdentifier,
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[derive(Debug)]
pub enum Operation {
    Add(u128),
    Multiply(u128),
    Power,
}

impl Operation {
    pub fn new((op, right): (&str, &str)) -> Result<Self> {
        let right = if right == "old" {
            None
        } else {
            Some(right.parse::<u128>()?)
        };
        if op == "+" {
            if let Some(right) = right {
                Ok(Self::Add(right))
            } else {
                Ok(Self::Multiply(2))
            }
        } else if let Some(right) = right {
            Ok(Self::Multiply(right))
        } else {
            Ok(Self::Power)
        }
    }
}

#[derive(Debug)]
pub struct Test {
    pub divisor: u64,
    pub matched_receiver: u32,
    pub unmatched_receiver: u32,
}

impl Test {
    pub fn new(divisor: &str, matched: &str, unmatched: &str) -> Result<Self, MonkeyError> {
        let divisor = divisor
            .split_whitespace()
            .nth(3)
            .ok_or(MonkeyError::WrongDivisorFormat)?
            .parse::<u64>()?;

        let matched_receiver = matched
            .split_whitespace()
            .nth(5)
            .ok_or(MonkeyError::WrongMatchedMonkeyIdentifier)?
            .parse::<u32>()?;

        let unmatched_receiver = unmatched
            .split_whitespace()
            .nth(5)
            .ok_or(MonkeyError::WrongUnmatchedMonkeyIdentifier)?
            .parse::<u32>()?;

        Ok(Test {
            divisor,
            matched_receiver,
            unmatched_receiver,
        })
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: RefCell<Vec<u128>>,
    pub operation: Operation,
    pub test: Test,
}

#[derive(Debug)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
    relief_factor: Option<u128>,
}

impl Monkey {
    pub fn new(
        (items, op, divisor, matched, unmatched): (&str, &str, &str, &str, &str),
    ) -> Result<Self> {
        let items = items
            .split_whitespace()
            .skip(2)
            .map(|item| -> Result<_> { item.replace(',', "").parse::<u128>().map_err(Into::into) })
            .collect::<Result<Vec<_>>>()?;
        let items = RefCell::new(items);

        let mut operation = op
            .split_whitespace()
            .skip(4)
            .tuples()
            .map(|(op, right)| Operation::new((op, right)))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(operation.len(), 1);
        let operation = operation.pop().unwrap();

        let test = Test::new(divisor, matched, unmatched)?;

        Ok(Self {
            items,
            operation,
            test,
        })
    }
}

impl Monkeys {
    pub fn new(file: &str, custom_relief: bool) -> Result<Self> {
        let monkeys = read_to_string(file)?
            .lines()
            .filter(|line| !(line.is_empty() || line.starts_with("Monkey")))
            .tuples()
            .map(|(items, op, test, matched, unmatched)| -> Result<Monkey> {
                Monkey::new((items, op, test, matched, unmatched))
            })
            .collect::<Result<Vec<_>>>()?;

        let relief_factor = if custom_relief {
            Some(
                monkeys
                    .iter()
                    .map(|monkey| monkey.test.divisor as u128)
                    .product(),
            )
        } else {
            None
        };
        Ok(Self {
            monkeys,
            relief_factor,
        })
    }

    fn relief(&self, item: u128) -> u128 {
        match self.relief_factor {
            Some(relief_factor) => item % relief_factor,
            None => item / 3,
        }
    }

    pub fn sling_stuff(self, rounds: u32) -> u64 {
        let mut inspections_count = vec![0_u64; self.monkeys.len()];
        for _ in 0..rounds {
            self.monkeys
                .iter()
                .zip(inspections_count.iter_mut())
                .for_each(|(monkey, inspection_count)| {
                    let items_count = monkey.items.borrow().len();
                    for _ in 0..items_count {
                        if let Some(item) = monkey.items.borrow_mut().pop() {
                            *inspection_count += 1;
                            let item = match &monkey.operation {
                                Operation::Add(right) => item + right,
                                Operation::Multiply(right) => item * right,
                                Operation::Power => item.pow(2),
                            };
                            let item = self.relief(item);
                            if item % monkey.test.divisor as u128 == 0 {
                                self.monkeys[monkey.test.matched_receiver as usize]
                                    .items
                                    .borrow_mut()
                                    .push(item);
                            } else {
                                self.monkeys[monkey.test.unmatched_receiver as usize]
                                    .items
                                    .borrow_mut()
                                    .push(item);
                            }
                        }
                    }
                });
        }
        inspections_count.iter().enumerate().for_each(|(i, count)| {
            println!("Monkey {} inspected items {} times", i, count);
        });
        inspections_count.sort();

        inspections_count.into_iter().rev().take(2).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let monkeys = Monkeys::new("example.txt", false).unwrap();
        let inspections_product = monkeys.sling_stuff(20);
        assert_eq!(inspections_product, 10605);
    }
    #[test]
    fn input() {
        let monkeys = Monkeys::new("input.txt", false).unwrap();
        let inspections_product = monkeys.sling_stuff(20);
        assert_eq!(inspections_product, 62491);
    }

    #[test]
    fn example_without_relief() {
        let monkeys = Monkeys::new("example.txt", true).unwrap();
        let inspections_product = monkeys.sling_stuff(10000);
        assert_eq!(inspections_product, 2713310158);
    }
    #[test]
    fn input_without_relief() {
        let monkeys = Monkeys::new("input.txt", true).unwrap();
        let inspections_product = monkeys.sling_stuff(10000);
        assert_eq!(inspections_product, 17408399184);
    }
}
