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

    pub fn is_in_order(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Signal::Integer(a), Signal::Integer(b)) => match a.cmp(b) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(false),
            },
            (Signal::List(a), Signal::List(b)) => {
                let a = a.borrow();
                let b = b.borrow();

                // println!("a: {:#?}, b: {:#?}", a, b);
                let comparison = a
                    .iter()
                    .zip(b.iter())
                    .map(|(a, b)| a.is_in_order(b.as_ref()))
                    // .for_each(|x| println!("comparison: {:#?}", x));
                    .find(|x| x.is_some())
                    // .skip_while(|x| x.is_none())
                    // .next()
                    .unwrap_or(None);
                // let comparison = None;

                // println!("comparison: {:#?}", comparison);

                match comparison {
                    Some(_) => comparison,
                    None => match a.len().cmp(&b.len()) {
                        std::cmp::Ordering::Less => Some(true),
                        std::cmp::Ordering::Equal => None,
                        std::cmp::Ordering::Greater => Some(false),
                    },
                }
            }
            (Signal::List(_), Signal::Integer(b)) => {
                let b = Signal::List(RefCell::new(vec![Rc::new(Signal::Integer(*b))]));
                self.is_in_order(&b)
            }
            (Signal::Integer(a), Signal::List(_)) => {
                // println!("self: {:#?}, other: {:#?}", self, other);
                let a = Signal::List(RefCell::new(vec![Rc::new(Signal::Integer(*a))]));
                a.is_in_order(other)
            }
        }
    }
}

impl PacketPair {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: Signal::new_list(left),
            right: Signal::new_list(right),
        }
    }

    pub fn is_in_order(&self) -> bool {
        self.left.is_in_order(self.right.as_ref()).unwrap_or(false)
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

    pub fn count_pairs_in_order(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.is_in_order())
            .map(|(index, _)| index + 1)
            // .for_each(|index| println!("Pair {} is in order", index));
            .sum()
        // 0
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
    fn signal_list_in_order() {
        let left = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(3)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
        ])));
        let right = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(5)),
            Rc::new(Signal::Integer(1)),
            Rc::new(Signal::Integer(1)),
        ])));
        let pairs = PacketPairs(vec![PacketPair { left, right }]);

        assert_eq!(pairs.count_pairs_in_order(), 1);
    }

    #[test]
    fn third() {
        let packet_pairs = PacketPairs::new("example.txt").unwrap();
        let third = packet_pairs.0.into_iter().nth(2).unwrap();
        let pairs = PacketPairs(vec![third]);
        let pairs_in_order = pairs.count_pairs_in_order();
        assert_eq!(pairs_in_order, 0);
    }

    #[test]
    fn example_1() {
        let packet_pairs = PacketPairs::new("example.txt").unwrap();
        let pairs_in_order = packet_pairs.count_pairs_in_order();
        assert_eq!(pairs_in_order, 13);
    }

    #[test]
    fn input_1() {
        let packet_pairs = PacketPairs::new("input.txt").unwrap();
        let pairs_in_order = packet_pairs.count_pairs_in_order();
        assert_eq!(pairs_in_order, 5715);
    }

    #[test]
    fn custom_input() {
        let left = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::List(RefCell::new(vec![
                Rc::new(Signal::Integer(4)),
                Rc::new(Signal::Integer(4)),
            ]))),
            Rc::new(Signal::List(RefCell::new(vec![
                Rc::new(Signal::Integer(4)),
                Rc::new(Signal::Integer(4)),
            ]))),
            Rc::new(Signal::Integer(9)),
        ])));
        let right = Rc::new(Signal::List(RefCell::new(vec![
            Rc::new(Signal::List(RefCell::new(vec![
                Rc::new(Signal::Integer(4)),
                Rc::new(Signal::Integer(4)),
            ]))),
            Rc::new(Signal::List(RefCell::new(vec![
                Rc::new(Signal::Integer(5)),
                Rc::new(Signal::Integer(3)),
            ]))),
            Rc::new(Signal::Integer(2)),
        ])));

        let pair = PacketPairs(vec![PacketPair { left, right }]);
        assert_eq!(pair.count_pairs_in_order(), 1);
    }
}
