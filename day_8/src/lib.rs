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
        println!("Border trees: {}", count);
        for c in 1..matrix.len() - 1 {
            for r in 1..matrix[c].len() - 1 {
                print!("{}", matrix[c][r]);
                if matrix[c][r] > matrix[c - 1][r]
                    || matrix[c][r] > matrix[c + 1][r]
                    || matrix[c][r] > matrix[c][r - 1]
                    || matrix[c][r] > matrix[c][r + 1]
                {
                    print!("+");
                    count += 1;
                }
            }
            println!("");
        }
        count
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
        matrix.print();
        assert_eq!(matrix.count_visible_trees(), 21);
    }
}
