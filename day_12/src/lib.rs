use anyhow::Result;
use std::fs::read_to_string;

pub struct Grid(Vec<Vec<char>>);

impl FromIterator<Vec<char>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<char>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Grid {
    pub fn new(file: &str) -> Result<Self> {
        Ok(read_to_string(file)?
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect())
    }

    pub fn print(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{}", c);
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
}
