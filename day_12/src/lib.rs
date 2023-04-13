use anyhow::Result;
use std::fs::read_to_string;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("No starting point found")]
    NoStartingPoint,
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
    steps: u32,
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
                            steps,
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
        let mut queue = vec![starting_point];

        while let Some(current_point) = queue.pop() {
            let mut neighbours = vec![];
            if current_point.x > 0
                && self.check_left_neighbour(
                    current_point,
                    &self.0[current_point.y][current_point.x - 1],
                )
            {
                neighbours.push(&self.0[current_point.y][current_point.x - 1]);
            }
        }
        Ok(0)
    }

    fn check_left_neighbour(&self, current: &Point, neighbour: &Point) -> bool {
        current.steps + 1 < neighbour.steps
            && (current.symbol == neighbour.symbol
                || current.symbol as u8 == neighbour.symbol as u8 + 1)
            || neighbour.symbol == 'Z'
    }

    pub fn print(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{:#?}", c);
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
