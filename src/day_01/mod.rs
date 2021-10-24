use std::collections::HashSet;

use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayOne();

impl AdventDay for DayOne {
    fn run_part_one(&self) -> String {
        let lines = DayData::from_file_path("./data/day01.txt");
        let frequencies = lines.iter();
        format!("Answer: {}", sum_frequencies(frequencies))
    }

    fn run_part_two(&self) -> String {
        let lines = DayData::from_file_path("./data/day01.txt");
        let frequencies = lines.iter();
        format!("Answer: {}", repeated_total(frequencies).unwrap())
    }
}

fn sum_frequencies<T>(frequencies: T) -> i64
where
    T: Iterator<Item = String>,
{
    frequencies.map(convert_to_int).sum()
}

fn repeated_total<T>(frequencies: T) -> Option<i64>
where
    T: Iterator<Item = String> + Clone,
{
    let running_totals = frequencies
        .map(convert_to_int)
        .cycle()
        .scan(0, |total, freq| {
            *total += freq;
            Some(*total)
        });
    let mut previous_totals = HashSet::new();
    for total in running_totals {
        if previous_totals.contains(&total) {
            return Some(total);
        }
        previous_totals.insert(total);
    }
    None
}

fn convert_to_int(freq: String) -> i64 {
    if let Some(value) = freq.strip_prefix('+') {
        value.parse::<i64>().unwrap()
    } else if let Some(value) = freq.strip_prefix('-') {
        -value.parse::<i64>().unwrap()
    } else {
        panic!("The value didn't start with + or -")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_sum_a_list_of_frequency_strings() {
        let input = vec!["+1".to_string(), "+3".to_string(), "-2".to_string()];
        let sum = sum_frequencies(input.into_iter());
        assert_eq!(sum, 2);
    }

    #[test]
    fn it_returns_the_running_total_that_happens_twice() {
        let input: Vec<String> = vec![
            "+3".to_string(),
            "+3".to_string(),
            "+4".to_string(),
            "-2".to_string(),
            "-4".to_string(),
        ];
        let repeated = repeated_total(input.into_iter()).unwrap();
        assert_eq!(repeated, 10);
    }
}
