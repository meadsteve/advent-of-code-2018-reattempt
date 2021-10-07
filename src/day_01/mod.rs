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
        todo!()
    }
}

fn sum_frequencies<'a, T>(frequencies: T) -> i64
    where
        T: Iterator<Item = &'a str>
{
    frequencies.map(convert_to_int).sum()
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
    use crate::day_01::sum_frequencies;

    #[test]
    fn it_works_for_a_list_of_frequency_strings() {
        let input = vec!["+1", "+3", "-2"];
        let sum = sum_frequencies(input.into_iter());
        assert_eq!(sum, 2);
    }
}