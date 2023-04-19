use anyhow::Result;
use itertools::Itertools;
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Debug)]
pub enum Signal {
    Integer(u32),
    List(RefCell<Vec<Rc<Signal>>>),
}

#[derive(Debug)]
pub struct PacketPair {
    pub left: Rc<Signal>,
    pub right: Rc<Signal>,
}

#[derive(Debug)]
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
        line.chars().for_each(|c| match c {
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
            .map(|(left, right)| PacketPair::new(left, right))
            .collect();
        Ok(packet_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let packet_pairs = PacketPairs::new("example.txt").unwrap();
        println!("{:#?}", packet_pairs);
    }
}
