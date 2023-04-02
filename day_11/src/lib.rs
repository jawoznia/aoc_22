use std::fs::read_to_string;

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
            .ok_or_else(|| MonkeyError::WrongDivisorFormat)?
            .parse::<u32>()?;

        let matched_receiver = matched
            .split_whitespace()
            .nth(5)
            .ok_or_else(|| MonkeyError::WrongMatchedMonkeyIdentifier)?
            .parse::<u32>()?;

        let unmatched_receiver = unmatched
            .split_whitespace()
            .nth(5)
            .ok_or_else(|| MonkeyError::WrongUnmatchedMonkeyIdentifier)?
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

impl Monkey {
    pub fn new(
        (items, op, divisor, matched, unmatched): (&str, &str, &str, &str, &str),
    ) -> Result<Self> {
        let items = items
            .split_whitespace()
            .skip(2)
            .map(|item| -> Result<u32> { item.replace(',', "").parse::<u32>().map_err(Into::into) })
            .collect::<Result<Vec<_>>>()?;

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
            .tuples()
            .map(
                |(_, items, op, test, matched, unmatched, _)| -> Result<Monkey> {
                    Monkey::new((items, op, test, matched, unmatched))
                },
            )
            .collect::<Result<Monkeys>>()?;

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
