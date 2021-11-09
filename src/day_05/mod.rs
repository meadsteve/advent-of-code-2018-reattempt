use rayon::prelude::*;

use crate::helpers::DayData;
use crate::AdventDay;

mod replacer;

pub struct DayFive();

impl AdventDay for DayFive {
    fn run_part_one(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        format!("the result: {}", replacer::remove_case_pairs(input).len())
    }

    fn run_part_two(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        let letters: Vec<char> = ('a'..='z').collect();
        let mut answers: Vec<(char, usize)> = letters
            .par_iter()
            .map(|&c| {
                let new_string = with_letter_removed(&input, c);
                (c, replacer::remove_case_pairs(new_string).len())
            })
            .collect();
        answers.sort_by(|(_, a), (_, b)| a.cmp(b));
        let answer = answers.first().expect("No answers found");
        format!("Letter {} with {}", answer.0, answer.1)
    }
}

fn with_letter_removed(input: &str, letter: char) -> String {
    let mut new_string = input.to_string();
    new_string = new_string.replace(letter, "");
    new_string = new_string.replace(letter.to_uppercase().next().unwrap(), "");
    new_string
}
