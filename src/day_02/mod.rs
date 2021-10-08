use crate::helpers::DayData;
use crate::AdventDay;
use std::collections::HashMap;

use rayon::prelude::*;

pub struct DayTwo();

impl AdventDay for DayTwo {
    fn run_part_one(&self) {
        let data = DayData::from_file_path("./data/day02.txt");
        let lines: Vec<&str> = data.iter().collect();
        let with_two = lines
            .par_iter()
            .filter(|word| has_repeated_letters(word, 2))
            .count();
        let with_three = lines
            .par_iter()
            .filter(|word| has_repeated_letters(word, 3))
            .count();
        println!(
            "Solution: {} x {} = {}",
            with_two,
            with_three,
            with_two * with_three
        );
    }

    fn run_part_two(&self) {
        let data = DayData::from_file_path("./data/day02.txt");
        let lines: Vec<&str> = data.iter().collect();

        for x in lines.iter() {
            for y in lines.iter() {
                match differ_by_one(x, y) {
                    DiffResult::DiffByOne(common) => {
                        println!("Pair different by one: {} and {} = {}", x, y, common);
                        return;
                    }
                    DiffResult::Nope => continue,
                }
            }
        }
    }
}

fn has_repeated_letters(input: &str, count_required: u32) -> bool {
    let char_counts = input.chars().fold(HashMap::new(), |mut totals, c| {
        let total = totals.entry(c).or_insert(0);
        *total += 1;
        totals
    });
    char_counts
        .iter()
        .map(|(_, count)| count)
        .any(|&count| count == count_required)
}

#[derive(Debug, PartialEq)]
enum DiffResult {
    DiffByOne(String),
    Nope,
}

fn differ_by_one(left: &str, right: &str) -> DiffResult {
    let diffs: Vec<usize> = left
        .chars()
        .zip(right.chars())
        .enumerate()
        .filter(|(_pos, (a, b))| a != b)
        .map(|(pos, _)| pos)
        .collect();
    if diffs.len() == 1 {
        let split_position = diffs[0];
        let common = left[..split_position].to_string() + &left[(split_position + 1)..];
        DiffResult::DiffByOne(common)
    } else {
        DiffResult::Nope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_exactly_of_a_letter_works_returns_true() {
        assert!(has_repeated_letters("bababc", 2));
        assert!(has_repeated_letters("bababc", 3));
        assert!(has_repeated_letters("abcccd", 3));
    }

    #[test]
    fn has_exactly_of_a_letter_works_returns_false() {
        assert!(!has_repeated_letters("abcdef", 2));
        assert!(!has_repeated_letters("abcdef", 3));
    }

    #[test]
    fn differ_by_one_works() {
        assert_eq!(
            differ_by_one("fghij", "fguij"),
            DiffResult::DiffByOne("fgij".to_string())
        );
    }
}
