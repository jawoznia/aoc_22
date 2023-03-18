pub File {
    pub name: String,
    pub size: u64,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub sub_directories: Vec<Directory>,
}

pub struct FileSystem {
    pub root: Directory,
    pub current_dir:Directory 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
