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

pub enum Instruction {
    Addx(i32),
    Noop,
}

pub fn print_message(file: &str) -> Result<()> {
    let mut signal_strength = 0;
    let mut ongoing_operation = None;

    let instructions: Vec<Instruction> = std::fs::read_to_string(file)?
        .lines()
        .map(|line| {
            let instruction = line.split_whitespace().collect::<Vec<&str>>();
            match instruction.len() {
                1 => Instruction::Noop,
                _ => Instruction::Addx(instruction[1].parse::<i32>().unwrap()),
            }
        })
        .collect();

    let mut instruction_iter = instructions.iter();
    for cycle in 0..240 {
        if ongoing_operation.is_none() {
            ongoing_operation = match instruction_iter.next() {
                Some(Instruction::Addx(x)) => Some(x),
                _ => None,
            };
        } else {
            signal_strength += ongoing_operation.unwrap();
            ongoing_operation = None;
        }

        if (signal_strength - 1..=signal_strength + 1).contains(&(cycle % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        // if cycle % 40 == 0 && cycle != 0 {
        if [39, 79, 119, 159, 199, 239].contains(&cycle) {
            print!("\n");
        }
    }

    Ok(())
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

    #[test]
    fn example_2() {
        print_message("example.txt").unwrap();
    }

    #[test]
    fn input_2() {
        print_message("input.txt").unwrap();
    }
}
