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

    // Border trees are always visible
    pub fn highest_viewing_distance(&self) -> usize {
        let matrix = &self.0;
        let mut highest = 0;
        for r in 1..matrix.len() - 1 {
            for c in 1..matrix[r].len() - 1 {
                let new_highest = self.count_to_top(r, c)
                    * self.count_to_bottom(r, c)
                    * self.count_to_left(r, c)
                    * self.count_to_right(r, c);
                if new_highest > highest {
                    highest = new_highest;
                }
            }
        }
        highest
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

    fn count_to_left(&self, r: usize, c: usize) -> usize {
        let matrix = &self.0;
        let val = matrix[r][c];
        let row_width = matrix[r].len();

        let val = matrix
            .iter()
            .skip(r)
            .take(1)
            .flatten()
            .rev()
            .skip(row_width - c)
            .take_while(|&&x| x < val)
            .count();

        // If we reached the end of the row, return the val
        // otherwise add 1 to the count as we see the last tree
        let val = if val == c { val } else { val + 1 };
        val
    }

    fn count_to_right(&self, r: usize, c: usize) -> usize {
        let matrix = &self.0;
        let val = matrix[r][c];
        let row_width = matrix[r].len();
        let max_trees = row_width - c - 1;

        let val = matrix
            .iter()
            .skip(r)
            .take(1)
            .flatten()
            .skip(c + 1)
            .take_while(|&&x| x < val)
            .count();
        let val = if val == max_trees { val } else { val + 1 };

        val
    }

    fn count_to_top(&self, r: usize, c: usize) -> usize {
        let matrix = &self.0;
        let row_width = matrix[r].len();
        let matrix_height = matrix.len();
        let val = matrix[r][c];

        let val = matrix
            .iter()
            .rev()
            .skip(matrix_height - r)
            .flatten()
            .skip(c)
            .step_by(row_width)
            .take_while(|&&x| x < val)
            .count();
        let val = if val == r { val } else { val + 1 };
        val
    }

    // All is based on this
    fn count_to_bottom(&self, r: usize, c: usize) -> usize {
        let matrix = &self.0;
        let row_width = matrix[r].len();
        let matrix_height = matrix.len();
        let val = matrix[r][c];
        let max_trees = matrix_height - r - 1;

        let val = matrix
            .iter()
            .skip(r + 1)
            .flatten()
            .skip(c)
            .step_by(row_width)
            .take_while(|&&x| x < val)
            .count();

        let val = if val == max_trees { val } else { val + 1 };
        val
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
    fn example_count_trees() {
        let matrix = Matrix::new("example.txt").unwrap();
        assert_eq!(matrix.count_visible_trees(), 21);
    }

    #[test]
    fn input_count_trees() {
        let matrix = Matrix::new("input.txt").unwrap();
        assert_eq!(matrix.count_visible_trees(), 1776);
    }

    #[test]
    fn example_highest_distance() {
        let matrix = Matrix::new("example.txt").unwrap();
        assert_eq!(matrix.highest_viewing_distance(), 8);
    }

    #[test]
    fn input_highest_distance() {
        let matrix = Matrix::new("input.txt").unwrap();
        assert_eq!(matrix.highest_viewing_distance(), 234416);
    }
}
