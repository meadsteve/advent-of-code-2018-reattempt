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
    let mut letters = input.iter().enumerate().peekable();
    while let Some((pos, first)) = letters.next() {
        let next = letters.peek();
        if let Some((_, second)) = next {
            if first.to_uppercase().next() == second.to_uppercase().next() && &first != second {
                input.remove(pos);
                input.remove(pos);
                return ReplacementResult::ReplacementsMade(input);
            }
        }
    }
    ReplacementResult::Unchanged(input)
}

fn case_replace_repeat(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    loop {
        let result = case_replace(chars);
        match result {
            ReplacementResult::Unchanged(answer) => return answer.into_iter().collect(),
            ReplacementResult::ReplacementsMade(next) => chars = next,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;

    #[test]
    fn test_replaces_differently_cased_pairs() {
        let input = "bAaBcd".chars().collect();
        let expected: Vec<char> = "bBcd".chars().collect();
        if let ReplacementResult::ReplacementsMade(output) = case_replace(input) {
            assert_eq!(expected, output)
        } else {
            panic!("Expected a change to be made")
        }
    }

    #[test]
    fn test_replaces_differently_cased_pairs_repeatedly() {
        assert_eq!(
            "dabCBAcaDA".to_string(),
            case_replace_repeat("dabAcCaCBAcCcaDA".to_string())
        )
    }
}
