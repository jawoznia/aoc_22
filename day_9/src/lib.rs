use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
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

    pub fn move_towards(&mut self, other: &Point) {
        if self.is_adjacent_to(other) {
            return;
        }

        if self.x - other.x > 0 {
            self.x -= 1;
        } else if self.x - other.x < 0 {
            self.x += 1;
        }
        if self.y - other.y > 0 {
            self.y -= 1;
        } else if self.y - other.y < 0 {
            self.y += 1;
        }
    }
}

pub fn get_positions(file: &str) -> Result<(Point, Point)> {
    let mut tail = Point::new(0, 0);
    let mut head = Point::new(0, 0);

    std::fs::read_to_string(file)?.lines().for_each(|line| {
        let (direction, moves_count) = line.split_whitespace().collect_tuple().unwrap();
        let moves_count = moves_count.parse::<i32>().unwrap();
        for _ in 0..moves_count {
            head.move_in_direction(direction);
            tail.move_towards(&head);
        }
    });

    Ok((tail, head))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (tail, head) = get_positions("example.txt").unwrap();
        assert_eq!(head, Point { x: 2, y: 2 });
        assert_eq!(tail, Point { x: 1, y: 2 });
    }

    #[test]
    fn input() {
        let (tail, head) = get_positions("input.txt").unwrap();
        assert_eq!(head, Point { x: 363, y: -100 });
        assert_eq!(tail, Point { x: 364, y: -100 });
    }
}
