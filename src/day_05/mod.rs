use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayFive();

impl AdventDay for DayFive {
    fn run_part_one(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        format!("the result: {}", case_replace_repeat(&input).len())
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

fn case_replace(input: &str) -> String {
    let mut letters = input.chars().enumerate().peekable();
    while let Some((pos, first)) = letters.next() {
        let next = letters.peek();
        if let Some((_, second)) = next {
            if first.to_uppercase().next() == second.to_uppercase().next() && &first != second {
                let mut new_string = String::new();
                new_string.push_str(&input[..pos]);
                new_string.push_str(&input[pos + 2..]);
                return new_string;
            }
        }
    }
    input.to_string()
}

fn case_replace_repeat(input: &str) -> String {
    let mut answer = input.to_string();
    let mut next_answer;
    loop {
        next_answer = case_replace(&answer);
        if next_answer == answer {
            return next_answer;
        } else {
            answer = next_answer
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;

    #[test]
    fn test_replaces_differently_cased_pairs() {
        assert_eq!("bBcd".to_string(), case_replace("bAaBcd"))
    }

    #[test]
    fn test_replaces_differently_cased_pairs_repeatedly() {
        assert_eq!(
            "dabCBAcaDA".to_string(),
            case_replace_repeat("dabAcCaCBAcCcaDA")
        )
    }
}
