use anyhow::Result;
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Debug)]
pub enum Signal {
    Integer(u32),
    List(RefCell<Vec<Rc<Signal>>>),
}

#[derive(Debug)]
pub struct PacketPair {
    pub left: Vec<Signal>,
    pub right: Vec<Signal>,
}

#[derive(Debug)]
pub struct PacketPairs(Vec<PacketPair>);

pub fn load(file: &str) -> Result<Rc<Signal>> {
    // chars: '[' ']' ',' 'integer'
    // '[' -> push new vec to stack
    // ']' -> pop vec from stack somewhere
    // integer -> add to vec on top of stack
    // ',' -> ignore
    // let mut stack = LinkedList::new();
    let output = Rc::new(Signal::List(RefCell::new(vec![])));
    let mut current_depth = 0;
    let mut current_signal = output.clone();

    read_to_string(file)?.lines().for_each(|line| {
        // let mut signal = Signal::List(vec![]);
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
        })
    });
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let signal = load("example.txt").unwrap();
        println!("{:#?}", signal);
    }
}
