use anyhow::Result;
use std::cell::RefCell;
use std::fs::read_to_string;

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
    pub fn new(file: &str) -> Result<Self> {
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

    pub fn optimal_steps_from(&self, starting_symbols: &[char]) -> Option<u32> {
        self.0
            .iter()
            .flatten()
            .filter(|p| starting_symbols.contains(&p.symbol))
            .map(|p| {
                self.0.iter().flatten().for_each(|p| {
                    p.steps.replace(u32::MAX);
                });
                p.steps.replace(0);
                self.find_optimal_steps(p)
            })
            .min()
    }

    fn find_optimal_steps(&self, starting_point: &Point) -> u32 {
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
        max_steps[0]
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
        let steps = grid.optimal_steps_from(&['S']).unwrap();
        grid.print();
        assert_eq!(steps, 31);
    }

    #[test]
    fn input() {
        let grid = Grid::new("input.txt").unwrap();
        let steps = grid.optimal_steps_from(&['S']).unwrap();
        grid.print();
        assert_eq!(steps, 352);
    }

    #[test]
    fn example_two() {
        let grid = Grid::new("example.txt").unwrap();
        let steps = grid.optimal_steps_from(&['S', 'a']).unwrap();
        grid.print();
        assert_eq!(steps, 29);
    }

    #[test]
    fn input_two() {
        let grid = Grid::new("input.txt").unwrap();
        let steps = grid.optimal_steps_from(&['S', 'a']).unwrap();
        grid.print();
        assert_eq!(steps, 345);
    }
}
