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
    Add(u32),
    Multiply(u32),
    Power,
}

impl Operation {
    pub fn new((op, right): (&str, &str)) -> Result<Self> {
        let right = if right == "old" {
            None
        } else {
            Some(right.parse::<u32>()?)
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
    pub divisor: u32,
    pub matched_receiver: u32,
    pub unmatched_receiver: u32,
}

impl Test {
    pub fn new(divisor: &str, matched: &str, unmatched: &str) -> Result<Self, MonkeyError> {
        let divisor = divisor
            .split_whitespace()
            .nth(3)
            .ok_or(MonkeyError::WrongDivisorFormat)?
            .parse::<u32>()?;

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
    pub items: RefCell<Vec<u32>>,
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

impl Monkey {
    pub fn new(
        (items, op, divisor, matched, unmatched): (&str, &str, &str, &str, &str),
    ) -> Result<Self> {
        let items = items
            .split_whitespace()
            .skip(2)
            .map(|item| -> Result<u32> { item.replace(',', "").parse::<u32>().map_err(Into::into) })
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
    pub fn new(file: &str) -> Result<Self> {
        let monkeys: Monkeys = read_to_string(file)?
            .lines()
            .filter(|line| !(line.is_empty() || line.starts_with("Monkey")))
            .tuples()
            .map(|(items, op, test, matched, unmatched)| -> Result<Monkey> {
                Monkey::new((items, op, test, matched, unmatched))
            })
            .collect::<Result<Monkeys>>()?;

        Ok(monkeys)
    }

    pub fn sling_stuff(self) -> Option<(u32, u32)> {
        let mut inspections_count = vec![0_u32; self.0.len()];
        for _ in 0..20 {
            self.0.iter().zip(inspections_count.iter_mut()).for_each(
                |(monkey, inspection_count)| {
                    let items_count = monkey.items.borrow().len();
                    for _ in 0..items_count {
                        if let Some(item) = monkey.items.borrow_mut().pop() {
                            *inspection_count += 1;
                            let item = match &monkey.operation {
                                Operation::Add(right) => (item + right) / 3,
                                Operation::Multiply(right) => (item * right) / 3,
                                Operation::Power => item.pow(2) / 3,
                            };
                            if item % monkey.test.divisor == 0 {
                                self.0[monkey.test.matched_receiver as usize]
                                    .items
                                    .borrow_mut()
                                    .push(item);
                            } else {
                                self.0[monkey.test.unmatched_receiver as usize]
                                    .items
                                    .borrow_mut()
                                    .push(item);
                            }
                        }
                    }
                },
            );
        }
        inspections_count.iter().enumerate().for_each(|(i, count)| {
            println!("Monkey {} inspected items {} times", i, count);
        });
        inspections_count.sort();
        inspections_count.into_iter().rev().take(2).collect_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let monkey = Monkeys::new("example.txt").unwrap();
        let (first, second) = monkey.sling_stuff().unwrap();
        assert_eq!(first, 105);
        assert_eq!(second, 101);
    }
    #[test]
    fn input() {
        let monkey = Monkeys::new("input.txt").unwrap();
        let (first, second) = monkey.sling_stuff().unwrap();
        assert_eq!(first, 253);
        assert_eq!(second, 247);
    }
}
