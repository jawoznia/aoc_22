use anyhow::Result;

#[derive(Debug)]
pub struct Matrix(Vec<Vec<u8>>);

impl Matrix {
    pub fn new(file: &str) -> Result<Self> {
        let data = std::fs::read_to_string(file)?
            .lines()
            .map(|line| {
                line.chars()
                    .map(|s| s.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Ok(Self(data))
    }

    // Border trees are always visible
    pub fn count_visible_trees(&self) -> usize {
        let matrix = &self.0;
        let mut count = matrix.len() * 2 + matrix[0].len() * 2 - 4;
        for r in 1..matrix.len() - 1 {
            for c in 1..matrix[r].len() - 1 {
                if self.is_top_visible(r, c)
                    || self.is_bottom_visible(r, c)
                    || self.is_left_visible(r, c)
                    || self.is_right_visible(r, c)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_left_visible(&self, r: usize, c: usize) -> bool {
        let matrix = &self.0;
        let val = matrix[r][c];
        for pos in 0..c {
            if matrix[r][pos] >= val {
                return false;
            }
        }
        true
    }

    fn is_right_visible(&self, r: usize, c: usize) -> bool {
        let matrix = &self.0;
        let val = matrix[r][c];
        for pos in c + 1..matrix[r].len() {
            if matrix[r][pos] >= val {
                return false;
            }
        }
        true
    }

    fn is_top_visible(&self, r: usize, c: usize) -> bool {
        let matrix = &self.0;
        let val = matrix[r][c];
        for pos in 0..r {
            if matrix[pos][c] >= val {
                return false;
            }
        }
        true
    }
    fn is_bottom_visible(&self, r: usize, c: usize) -> bool {
        let matrix = &self.0;
        let val = matrix[r][c];
        for pos in r + 1..matrix.len() {
            if matrix[pos][c] >= val {
                return false;
            }
        }
        true
    }

    pub fn print(&self) {
        for row in &self.0 {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let matrix = Matrix::new("example.txt").unwrap();
        assert_eq!(matrix.count_visible_trees(), 21);
    }

    #[test]
    fn input() {
        let matrix = Matrix::new("input.txt").unwrap();
        assert_eq!(matrix.count_visible_trees(), 1776);
    }
}
