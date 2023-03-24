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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let fs = Directories::new("example.txt").unwrap();
        println!("{:#?}", fs);
    }
}
