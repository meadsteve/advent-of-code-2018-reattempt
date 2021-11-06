use crate::helpers::DayData;
use crate::AdventDay;
use std::sync::{Arc, Mutex};

pub struct DayFive();

impl AdventDay for DayFive {
    fn run_part_one(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        format!("the result: {}", replacer::remove_case_pairs(input).len())
    }

    fn run_part_two(&self) -> String {
        let input = DayData::from_file_path("./data/day05.txt").first_line();
        let results: Arc<Mutex<Vec<(char, usize)>>> = Arc::new(Mutex::new(Vec::new()));
        let pool = threadpool::Builder::new().build();
        for c in 'a'..='z' {
            let mut input_without_c = input.clone();
            let local_results = Arc::clone(&results);
            input_without_c = input_without_c.replace(c, "");
            input_without_c = input_without_c.replace(c.to_uppercase().next().unwrap(), "");
            pool.execute(move || {
                let result = replacer::remove_case_pairs(input_without_c).len();
                local_results.lock().unwrap().push((c, result));
            });
        }
        pool.join();
        let mut results = results
            .lock()
            .expect("Problem occured getting the results from the threads");
        results.sort_by(|(_, a), (_, b)| a.cmp(b));
        let answer = *results
            .first()
            .expect("The results list was strangely empty");
        format!("Letter {} with {}", answer.0, answer.1)
    }
}

mod replacer {
    enum ReplacementResult {
        ReplacementsMade(Vec<char>),
        Unchanged(Vec<char>),
    }

    pub fn remove_case_pairs(input: String) -> String {
        let mut chars: Vec<char> = input.chars().collect();
        loop {
            let result = single_pass_replace(chars);
            match result {
                ReplacementResult::Unchanged(answer) => {
                    return answer.into_iter().filter(|&c| c != ' ').collect()
                }
                ReplacementResult::ReplacementsMade(next) => chars = next,
            }
        }
    }

    fn single_pass_replace(mut input: Vec<char>) -> ReplacementResult {
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
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;

    #[test]
    fn test_replaces_differently_cased_pairs_repeatedly() {
        assert_eq!(
            "dabCBAcaDA".to_string(),
            replacer::remove_case_pairs("dabAcCaCBAcCcaDA".to_string())
        )
    }
}
