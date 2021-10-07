use std::collections::HashSet;
use std::fs;
use crate::AdventDay;

pub struct DayOne();

impl AdventDay for DayOne {
    fn run_part_one(&self) {
        let contents = fs::read_to_string("./data/day01.txt")
            .expect("The input file was missing");
        let frequencies = contents.split("\n")
            .filter(|&line| line != "");
        println!("Answer: {}", sum_frequencies(frequencies));
    }

    fn run_part_two(&self) {
        let contents = fs::read_to_string("./data/day01.txt")
            .expect("The input file was missing");
        let frequencies = contents.split("\n")
            .filter(|&line| line != "");
        println!("Answer: {}", repeated_total(frequencies).unwrap());
    }
}

fn sum_frequencies<'a, T>(frequencies: T) -> i64
    where
        T: Iterator<Item = &'a str>
{
    frequencies.map(convert_to_int).sum()
}

fn repeated_total<'a, T>(frequencies: T) -> Option<i64>
    where
        T: Iterator<Item = &'a str> + Clone
{
    let running_totals = frequencies
        .map(convert_to_int)
        .cycle()
        .scan(0, |total, freq| {
            *total = *total + freq;
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

fn convert_to_int(freq: &str) -> i64 {
    if freq.starts_with("+") {
        freq[1..].parse::<i64>().unwrap()
    } else {
        freq[1..].parse::<i64>().unwrap() * -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_sum_a_list_of_frequency_strings() {
        let input = vec!["+1", "+3", "-2"];
        let sum = sum_frequencies(input.into_iter());
        assert_eq!(sum, 2);
    }

    #[test]
    fn it_returns_the_running_total_that_happens_twice() {
        let input = vec!["+3", "+3", "+4", "-2", "-4"];
        let repeated = repeated_total(input.into_iter()).unwrap();
        assert_eq!(repeated, 10);
    }
}