use std::collections::LinkedList;
use std::fs;

pub fn get_highest_calories(path: &str) -> u32 {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut calories = LinkedList::<u32>::new();
    calories.push_front(0);
    data.split('\n').for_each(|s| {
        if s.is_empty() {
            calories.push_back(0);
        } else {
            *calories.back_mut().unwrap() += s.trim().parse().unwrap_or(0);
        }
    });

    calories.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test_data() {
        let calories = get_highest_calories("test_data.txt");
        assert_eq!(calories, 24000);
    }

    #[test]
    fn assigned_values() {
        let calories = get_highest_calories("input.txt");
        println!("{}", calories);
    }
}
