mod linked_letters;

use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayFive();

impl AdventDay for DayFive {
    fn run_part_one(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        format!("the result: {}", case_replace_repeat(input).len())
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

enum ReplacementResult {
    ReplacementsMade(Vec<char>),
    Unchanged(Vec<char>),
}

fn case_replace(mut input: Vec<char>) -> ReplacementResult {
    let pairs_to_remove = find_pair_to_remove(&input);
    match pairs_to_remove.len() {
        0 => ReplacementResult::Unchanged(input),
        _ => {
            for (x, y) in pairs_to_remove {
                input[x] = ' ';
                input[y] = ' ';
            }
            ReplacementResult::ReplacementsMade(input)
        }
    }
}

fn find_pair_to_remove(input: &[char]) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    let mut letters = input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != ' ')
        .peekable();
    while let Some((pos_first, first)) = letters.next() {
        let next = letters.peek();
        if let Some((pos_second, second)) = next {
            if first.to_uppercase().next() == second.to_uppercase().next() && &first != second {
                output.push((pos_first, *pos_second));
                letters.next();
            }
        }
    }
    output
}

fn case_replace_repeat(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    loop {
        let result = case_replace(chars);
        match result {
            ReplacementResult::Unchanged(answer) => {
                return answer.into_iter().filter(|&c| c != ' ').collect()
            }
            ReplacementResult::ReplacementsMade(next) => chars = next,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;

    #[test]
    fn test_replaces_differently_cased_pairs_repeatedly() {
        assert_eq!(
            "dabCBAcaDA".to_string(),
            case_replace_repeat("dabAcCaCBAcCcaDA".to_string())
        )
    }
}
