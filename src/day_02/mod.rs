use crate::helpers::DayData;
use crate::AdventDay;
use std::collections::HashMap;

pub struct DayTwo();

impl AdventDay for DayTwo {
    fn run_part_one(&self) {
        let lines = DayData::from_file_path("./data/day02.txt");
        let with_two = lines
            .iter()
            .filter(|word| has_repeated_letters(word, 2))
            .count();
        let with_three = lines
            .iter()
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
        todo!()
    }
}

fn has_repeated_letters(input: &str, count_required: u32) -> bool {
    let char_counts = input.chars().fold(HashMap::new(), |mut totals, c| {
        let total = totals.entry(c).or_insert(0);
        *total += 1;
        totals
    });
    let correct_counts: Vec<&u32> = char_counts
        .iter()
        .map(|(_, count)| count)
        .filter(|&&count| count == count_required)
        .take(1)
        .collect();
    !correct_counts.is_empty()
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
}
