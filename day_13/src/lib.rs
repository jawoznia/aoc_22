use anyhow::Result;
use itertools::Itertools;
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Debug, Eq, PartialEq)]
pub enum Signal {
    Integer(u32),
    List(RefCell<Vec<Rc<Signal>>>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct PacketPair {
    pub left: Rc<Signal>,
    pub right: Rc<Signal>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PacketPairs(Vec<PacketPair>);

impl FromIterator<PacketPair> for PacketPairs {
    fn from_iter<T: IntoIterator<Item = PacketPair>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Signal {
    pub fn new_list(line: &str) -> Rc<Self> {
        let output = Rc::new(Signal::List(RefCell::new(vec![])));
        let mut current_depth = 0;
        let mut current_signal = output.clone();
        line.chars().skip(1).for_each(|c| match c {
            '[' => match current_signal.clone().as_ref() {
                Signal::List(vec) => {
                    vec.borrow_mut()
                        .push(Rc::new(Signal::List(RefCell::new(vec![]))));
                    current_signal = vec.borrow().last().unwrap().clone();
                    current_depth += 1;
                }
                Signal::Integer(_) => panic!("Current signal is an integer"),
            },
            ']' => match current_signal.as_ref() {
                Signal::List(_) => {
                    current_signal = output.clone();
                    current_depth -= 1;
                    for _ in 0..current_depth {
                        if let Signal::List(vec) = current_signal.clone().as_ref() {
                            current_signal = vec.borrow().last().unwrap().clone();
                        }
                    }
                }
                Signal::Integer(_) => panic!("Current signal is an integer"),
            },
            ',' => (),
            _ => match current_signal.as_ref() {
                Signal::List(vec) => vec
                    .borrow_mut()
                    .push(Rc::new(Signal::Integer(c.to_digit(10).unwrap()))),
                Signal::Integer(_) => panic!("Unexpected integer"),
            },
        });
        output
    }
}

impl PacketPair {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: Signal::new_list(left),
            right: Signal::new_list(right),
        }
    }
}

impl PacketPairs {
    pub fn new(file: &str) -> Result<Self> {
        let packet_pairs: Self = read_to_string(file)?
            .lines()
            .filter(|line| !line.is_empty())
            .tuple_windows()
            .step_by(2)
            .map(|(left, right)| PacketPair::new(left, right))
            .collect();
        Ok(packet_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_pairs_new() {
        let packet_pairs = PacketPairs::new("example.txt").unwrap();
        let first = packet_pairs.0[0].left.clone();
        let expected_first = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(3)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
        ])));
        assert_eq!(first, expected_first);

        let fourth = packet_pairs.0[3].right.clone();
        let expected_fourth = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::List(RefCell::new(vec![
                Rc::new(Signal::Integer(4)),
                Rc::new(Signal::Integer(4)),
            ]))),
            Rc::new(Signal::Integer(4)),
            Rc::new(Signal::Integer(4)),
            Rc::new(Signal::Integer(4)),
        ])));
        assert_eq!(fourth, expected_fourth);

        let seventh = packet_pairs.0[6].left.clone();
        let expected_seventh = Rc::new(Signal::List(RefCell::new(vec![Rc::new(Signal::List(
            RefCell::new(vec![Rc::new(Signal::List(RefCell::new(vec![])))]),
        ))])));
        assert_eq!(seventh, expected_seventh);
    }

    #[test]
    fn example_1() {
        let _packet_pairs = PacketPairs::new("example.txt").unwrap();
    }

    #[test]
    fn input_1() {
        let _packet_pairs = PacketPairs::new("input.txt").unwrap();
    }
}
