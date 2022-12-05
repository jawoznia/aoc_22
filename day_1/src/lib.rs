use std::fs;

fn load_data(path: &str) {
    let data = fs::read_to_string(path).expect("Unable to read file");

    println!("{}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test_data() {
        load_data("test_data.txt");
    }
}
