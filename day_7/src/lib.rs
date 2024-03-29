use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, Default)]
pub struct Directories(HashMap<PathBuf, u64>);

impl From<HashMap<PathBuf, u64>> for Directories {
    fn from(c: HashMap<PathBuf, u64>) -> Self {
        Self(c)
    }
}

impl Directories {
    pub fn new(file: &str) -> Result<Directories> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let mut directories = HashMap::new();
        let mut current_dir = PathBuf::new();
        reader
            .lines()
            .flatten()
            .filter(|line| !line.starts_with("$ ls") && !line.starts_with("dir"))
            .for_each(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();

                if words[0] == "$" {
                    if words[2] == ".." {
                        current_dir.pop();
                    } else if words[2] == "/" && !directories.is_empty() {
                        current_dir = PathBuf::new();
                        current_dir.push("/");
                    } else {
                        let new_path = current_dir.join(words[2]);
                        directories.insert(new_path.clone(), 0);
                        current_dir = new_path;
                    }
                } else {
                    match directories.get_mut(&current_dir) {
                        Some(size) => {
                            *size += words[0].parse::<u64>().unwrap();
                        }
                        None => panic!("No such directory: {:#?}", current_dir),
                    }
                }
            });

        Ok(directories.into())
    }

    fn total_dir_sizes(&self) -> Vec<u64> {
        self.0
            .keys()
            .map(|l_path| {
                self.0
                    .iter()
                    .filter(|(r_path, _)| r_path.starts_with(l_path))
                    .map(|(_, size)| size)
                    .sum::<u64>()
            })
            .collect()
    }

    pub fn sum(&self) -> u64 {
        self.total_dir_sizes()
            .iter()
            .filter(|size| **size <= 100000_u64)
            .sum()
    }

    pub fn size_to_delete(&self) -> u64 {
        let mut sizes = self.total_dir_sizes();
        assert!(!sizes.is_empty());
        sizes.sort();
        let size_to_free = 30000000 - (70000000 - sizes.last().unwrap());
        sizes
            .iter()
            .filter(|size| **size > size_to_free)
            .take(1)
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sum() {
        let fs = Directories::new("example.txt").unwrap();
        assert_eq!(fs.sum(), 95437);
    }

    #[test]
    fn input_sum() {
        let fs = Directories::new("input.txt").unwrap();
        assert_eq!(fs.sum(), 1648397);
    }
    #[test]
    fn example_delete() {
        let fs = Directories::new("example.txt").unwrap();
        assert_eq!(fs.size_to_delete(), 24933642);
    }

    #[test]
    fn input_delete() {
        let fs = Directories::new("input.txt").unwrap();
        assert_eq!(fs.size_to_delete(), 1815525);
    }
}
