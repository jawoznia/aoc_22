use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Rucksack(Vec<char>);
#[derive(Debug)]
pub struct Rucksacks(Vec<(Rucksack, Rucksack, Rucksack)>);

impl From<Vec<char>> for Rucksack {
    fn from(c: Vec<char>) -> Self {
        Rucksack(c)
    }
}

impl FromIterator<char> for Rucksack {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

pub fn find_duplication(v: &(&Vec<char>, &Vec<char>, &Vec<char>)) -> Option<char> {
    // while i_l < len && i_r < len {
    //     match self.left[i_l].cmp(&self.right[i_r]) {
    //         std::cmp::Ordering::Less => i_l += 1,
    //         std::cmp::Ordering::Equal => return Some(self.left[i_l]),
    //         std::cmp::Ordering::Greater => i_r += 1,
    //     }
    // }

    None
}

impl Rucksacks {
    pub fn new(file: &str) -> Result<Self> {
        let file = File::open(file)?;
        let mut reader = BufReader::new(file);
        let lines: Vec<String> = reader.by_ref().lines().filter_map(|l| l.ok()).collect();
        let rucksacks: Vec<(Rucksack, Rucksack, Rucksack)> = lines
            .iter()
            .step_by(3)
            .zip(
                lines
                    .iter()
                    .skip(1)
                    .step_by(3)
                    .zip(lines.iter().skip(2).step_by(3)),
            )
            .map(|(a, (b, c))| {
                let mut a: Vec<char> = a.chars().collect();
                a.sort();
                let mut b: Vec<char> = b.chars().collect();
                b.sort();
                let mut c: Vec<char> = c.chars().collect();
                c.sort();
                (a.into(), b.into(), c.into())
            })
            .collect();

        Ok(Self(rucksacks))
    }

    // First solution
    pub fn calc_prio(&self) -> u32 {
        let uppercase_a = 'A' as u32 - 1;
        let lowercase_a = 'a' as u32 - 1;

        0
        // self.0
        //     .iter()
        //     .filter_map(|rs| rs.find_duplication())
        //     .map(|c| {
        //         if c >= 'a' {
        //             c as u32 - lowercase_a
        //         } else {
        //             c as u32 - uppercase_a + 26
        //         }
        //     })
        //     .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rs = Rucksacks::new("example.txt").unwrap();
        rs.0.iter().for_each(|r| println!("Rucksack: {:#?}", r));
        assert_eq!(rs.calc_prio(), 70);
    }
    #[test]
    fn first() {
        let rs = Rucksacks::new("first.txt").unwrap();
        rs.0.iter().for_each(|r| println!("Rucksack: {:#?}", r));
        assert_eq!(rs.calc_prio(), 7903);
    }
}
