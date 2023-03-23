use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, Default)]
pub struct Directories(HashMap<PathBuf, u64>);

#[derive(Debug)]
pub struct FileSystem {
    pub current_dir: PathBuf,
    pub directories: Directories,
}

impl FileSystem {
    pub fn new(file: &str) -> Result<FileSystem> {
        let file = std::fs::File::open(file)?;
        let reader = BufReader::new(file);

        let mut file_system = FileSystem {
            current_dir: PathBuf::new(),
            directories: Directories::default(),
        };
        reader
            .lines()
            .flatten()
            .filter(|line| !line.starts_with("$ ls") && !line.starts_with("dir"))
            .for_each(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();

                if words[0] == "$" {
                    if words[2] == ".." {
                        file_system.current_dir.pop();
                    } else if words[2] == "/" && !file_system.directories.0.is_empty() {
                        file_system.current_dir = PathBuf::new();
                        file_system.current_dir.push("/");
                    } else {
                        let new_path = file_system.current_dir.join(words[2]);
                        file_system.directories.0.insert(new_path.clone(), 0);
                        file_system.current_dir = new_path;
                    }
                } else {
                    match file_system.directories.0.get_mut(&file_system.current_dir) {
                        Some(size) => {
                            *size += words[0].parse::<u64>().unwrap();
                        }
                        None => panic!("No such directory: {:#?}", file_system.current_dir),
                    }
                }
            });

        Ok(file_system)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let fs = FileSystem::new("example.txt").unwrap();
        println!("{:#?}", fs);
    }
}
