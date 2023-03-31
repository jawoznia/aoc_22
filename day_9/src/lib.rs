use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

pub struct Solution {
    pub head: Point,
    pub tail: Point,
    pub visited_fields: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_in_direction(&mut self, direction: &str) {
        match direction {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("Unknown direction"),
        }
    }

    pub fn is_adjacent_to(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn move_towards(mut self, other: &Point) -> Self {
        if self.is_adjacent_to(other) {
            return self;
        }

        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => self.x += 1,
            std::cmp::Ordering::Greater => self.x -= 1,
            _ => (),
        }
        match self.y.cmp(&other.y) {
            std::cmp::Ordering::Less => self.y += 1,
            std::cmp::Ordering::Greater => self.y -= 1,
            _ => (),
        }
        self
    }
}

pub fn get_positions(file: &str) -> Result<Solution> {
    let mut tail = Point::new(0, 0);
    let mut head = Point::new(0, 0);
    let mut visited = HashSet::new();

    std::fs::read_to_string(file)?.lines().for_each(|line| {
        let (direction, moves_count) = line.split_whitespace().collect_tuple().unwrap();
        let moves_count = moves_count.parse::<i32>().unwrap();
        for _ in 0..moves_count {
            head.move_in_direction(direction);
            tail = tail.move_towards(&head);

            if !visited.contains(&tail) {
                visited.insert(tail);
            }
        }
    });

    Ok(Solution {
        head,
        tail,
        visited_fields: visited.len(),
    })
}

pub fn whole_rope(file: &str) -> Result<Solution> {
    let mut rope = vec![Point::new(0, 0); 10];
    let mut visited = HashSet::new();

    std::fs::read_to_string(file)?.lines().for_each(|line| {
        let (direction, moves_count) = line.split_whitespace().collect_tuple().unwrap();
        let moves_count = moves_count.parse::<i32>().unwrap();
        for _ in 0..moves_count {
            rope[0].move_in_direction(direction);
            (1..10).for_each(|i| {
                rope[i] = rope[i].move_towards(&rope[i - 1]);
            });

            // let s = &rope[9].borrow().to_owned;
            // if !visited.contains(&rope[9].borrow()) {
            visited.insert(rope[9]);
            // }
        }
    });

    Ok(Solution {
        head: rope[0],
        tail: rope[9],
        visited_fields: visited.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let solution = get_positions("example.txt").unwrap();
        assert_eq!(solution.visited_fields, 13);
        assert_eq!(solution.head, Point { x: 2, y: 2 });
        assert_eq!(solution.tail, Point { x: 1, y: 2 });
    }

    #[test]
    fn input() {
        let solution = get_positions("input.txt").unwrap();
        assert_eq!(solution.visited_fields, 6212);
        assert_eq!(solution.head, Point { x: 363, y: -100 });
        assert_eq!(solution.tail, Point { x: 364, y: -100 });
    }

    #[test]
    fn example_2() {
        let solution = whole_rope("example.txt").unwrap();
        assert_eq!(solution.visited_fields, 1);
        assert_eq!(solution.head, Point { x: 2, y: 2 });
        assert_eq!(solution.tail, Point { x: 0, y: 0 });
    }

    #[test]
    fn second_example_2() {
        let solution = whole_rope("second_example.txt").unwrap();
        assert_eq!(solution.visited_fields, 36);
        assert_eq!(solution.head, Point { x: -11, y: 15 });
        assert_eq!(solution.tail, Point { x: -11, y: 6 });
    }

    #[test]
    fn input_2() {
        let solution = whole_rope("input.txt").unwrap();
        assert_eq!(solution.visited_fields, 2522);
        assert_eq!(solution.head, Point { x: 363, y: -100 });
        assert_eq!(solution.tail, Point { x: 365, y: -93 });
    }
}
