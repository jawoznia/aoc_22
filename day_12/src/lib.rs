use anyhow::Result;
use std::cell::RefCell;
use std::fs::read_to_string;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("No starting point found")]
    NoStartingPoint,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Height map is empty")]
    HeightMapEmpty(),
}

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
    steps: RefCell<u32>,
    symbol: char,
}

#[derive(Debug)]
pub struct Grid(Vec<Vec<Point>>);

impl Grid {
    pub fn new(file: &str) -> Result<Self, GridError> {
        let grid = read_to_string(file)?
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let steps = if c == 'S' { 0 } else { u32::MAX };
                        Point {
                            x,
                            y,
                            steps: RefCell::new(steps),
                            symbol: c,
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();

        Ok(Self(grid))
    }

    pub fn find_optimal_steps(&self) -> Result<u32, GridError> {
        let starting_point = self
            .0
            .iter()
            .flatten()
            .find(|p| p.symbol == 'S')
            .ok_or(GridError::NoStartingPoint)?;
        let mut neighbours = vec![starting_point];

        while let Some(current) = neighbours.pop() {
            if current.x > 0 && self.check_neighbour(current, &self.0[current.y][current.x - 1]) {
                self.0[current.y][current.x - 1]
                    .steps
                    .replace(*current.steps.borrow() + 1);
                neighbours.push(&self.0[current.y][current.x - 1]);
            }

            if current.x < self.0[current.y].len() - 1
                && self.check_neighbour(current, &self.0[current.y][current.x + 1])
            {
                self.0[current.y][current.x + 1]
                    .steps
                    .replace(*current.steps.borrow() + 1);
                neighbours.push(&self.0[current.y][current.x + 1]);
            }

            if current.y > 0 && self.check_neighbour(current, &self.0[current.y - 1][current.x]) {
                self.0[current.y - 1][current.x]
                    .steps
                    .replace(*current.steps.borrow() + 1);
                neighbours.push(&self.0[current.y - 1][current.x]);
            }

            if current.y < self.0.len() - 1
                && self.check_neighbour(current, &self.0[current.y + 1][current.x])
            {
                self.0[current.y + 1][current.x]
                    .steps
                    .replace(*current.steps.borrow() + 1);
                neighbours.push(&self.0[current.y + 1][current.x]);
            }
        }
        let max_steps = self
            .0
            .iter()
            .flatten()
            .filter_map(|point| {
                if point.symbol == 'E' {
                    Some(*point.steps.borrow())
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>();
        assert_eq!(max_steps.len(), 1);
        Ok(max_steps[0])
    }

    fn check_neighbour(&self, current: &Point, neighbour: &Point) -> bool {
        let current_symbol = match current.symbol {
            'S' => 'a',
            _ => current.symbol,
        };
        *current.steps.borrow() + 1 < *neighbour.steps.borrow()
            && (neighbour.symbol as i32 - current_symbol as i32) <= 1
            || (current.symbol == 'z' && neighbour.symbol == 'E')
    }

    pub fn print(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{:#?}; ", *c.steps.borrow());
            });
            println!();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = Grid::new("example.txt").unwrap();
        let steps = grid.find_optimal_steps().unwrap();
        grid.print();
        assert_eq!(steps, 31);
    }

    #[test]
    fn input() {
        let grid = Grid::new("input.txt").unwrap();
        let steps = grid.find_optimal_steps().unwrap();
        grid.print();
        assert_eq!(steps, 352);
    }
}
