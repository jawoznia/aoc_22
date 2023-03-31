use anyhow::Result;

pub fn calculate_one(file: &str) -> Result<i32> {
    let mut cycle = 0;
    let mut curr_signal_strength = 1;
    let mut signal_strength = 0;
    let cycle_checks = [20, 60, 100, 140, 180, 220];

    std::fs::read_to_string(file)?.lines().for_each(|line| {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();

        match instruction.len() {
            1 => {
                cycle += 1;
                if cycle_checks.contains(&cycle) {
                    signal_strength += curr_signal_strength * cycle;
                }
            }
            _ => {
                cycle += 1;
                if cycle_checks.contains(&cycle) {
                    signal_strength += curr_signal_strength * cycle;
                }
                cycle += 1;
                if cycle_checks.contains(&cycle) {
                    signal_strength += curr_signal_strength * cycle;
                }
                curr_signal_strength += instruction[1].parse::<i32>().unwrap();
            }
        }
    });

    Ok(signal_strength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let signal_strength = calculate_one("example.txt").unwrap();
        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn input_1() {
        let signal_strength = calculate_one("input.txt").unwrap();
        assert_eq!(signal_strength, 15260);
    }
}
