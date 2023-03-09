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

pub fn find_duplication(l: Vec<char>, r: Vec<char>) -> Vec<char> {
    let l_len = l.len();
    let r_len = r.len();
    let mut i_l = 0;
    let mut i_r = 0;

    let mut duplications = vec![];

    while i_l < l_len && i_r < r_len {
        match l[i_l].cmp(&r[i_r]) {
            std::cmp::Ordering::Less => i_l += 1,
            std::cmp::Ordering::Equal => {
                duplications.push(l[i_l]);
                i_l += 1;
                i_r += 1;
            }
            std::cmp::Ordering::Greater => i_r += 1,
        }
    }

    duplications
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

    pub fn calc_prio(self) -> u32 {
        let uppercase_a = 'A' as u32 - 1;
        let lowercase_a = 'a' as u32 - 1;

        let rucksacks = self.0;

        rucksacks
            .into_iter()
            .map(|(a, b, c)| {
                let duplicated = find_duplication(a.0, b.0);
                let c = find_duplication(duplicated, c.0);
                assert!(!c.is_empty());
                let c = c[0];

                if c >= 'a' {
                    c as u32 - lowercase_a
                } else {
                    c as u32 - uppercase_a + 26
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rs = Rucksacks::new("example.txt").unwrap();
        assert_eq!(rs.calc_prio(), 70);
    }
    #[test]
    fn first() {
        let rs = Rucksacks::new("first.txt").unwrap();
        assert_eq!(rs.calc_prio(), 2548);
    }
}
