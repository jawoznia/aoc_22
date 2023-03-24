use anyhow::Result;

#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<Vec<u8>>,
}

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
        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let matrix = Matrix::new("example.txt").unwrap();
        println!("{:#?}", matrix.data)
    }
}
