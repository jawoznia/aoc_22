use anyhow::Result;
use std::{cell::RefCell, fs::read_to_string};

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

        while let Some(current_point) = neighbours.pop() {
            if current_point.x > 0
                && self
                    .check_neighbour(current_point, &self.0[current_point.y][current_point.x - 1])
            {
                self.0[current_point.y][current_point.x - 1]
                    .steps
                    .replace(*current_point.steps.borrow() + 1);
                neighbours.push(&self.0[current_point.y][current_point.x - 1]);
            }

            if current_point.x < self.0[current_point.x].len()
                && self
                    .check_neighbour(current_point, &self.0[current_point.y][current_point.x + 1])
            {
                self.0[current_point.y][current_point.x + 1]
                    .steps
                    .replace(*current_point.steps.borrow() + 1);
                neighbours.push(&self.0[current_point.y][current_point.x + 1]);
            }

            if current_point.y > 0
                && self
                    .check_neighbour(current_point, &self.0[current_point.y - 1][current_point.x])
            {
                self.0[current_point.y - 1][current_point.x]
                    .steps
                    .replace(*current_point.steps.borrow() + 1);
                neighbours.push(&self.0[current_point.y - 1][current_point.x]);
            }

            if current_point.y < self.0.len()
                && self
                    .check_neighbour(current_point, &self.0[current_point.y + 1][current_point.x])
            {
                self.0[current_point.y + 1][current_point.x]
                    .steps
                    .replace(*current_point.steps.borrow() + 1);
                neighbours.push(&self.0[current_point.y + 1][current_point.x]);
            }
        }
        let max_steps = self
            .0
            .iter()
            .flatten()
            .map(|point| *point.steps.borrow())
            .max()
            .unwrap_or(0);
        Ok(max_steps)
    }

    fn check_neighbour(&self, current: &Point, neighbour: &Point) -> bool {
        *current.steps.borrow() + 1 < *neighbour.steps.borrow()
            && (current.symbol == neighbour.symbol
                || current.symbol as u8 == neighbour.symbol as u8 + 1)
            || neighbour.symbol == 'Z'
    }

    pub fn print(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{:#?}; ", *c.steps.borrow());
            });
            print!("\n")
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = Grid::new("example.txt").unwrap();
        grid.print();
    }

    #[test]
    fn input() {
        let grid = Grid::new("input.txt").unwrap();
        grid.print();
    }
}
