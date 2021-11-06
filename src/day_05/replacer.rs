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

enum ReplacementResult {
    ReplacementsMade(Vec<char>),
    Unchanged(Vec<char>),
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

#[cfg(test)]
mod tests {
    use crate::day_05::replacer::*;

    #[test]
    fn test_replaces_differently_cased_pairs_repeatedly() {
        assert_eq!(
            "dabCBAcaDA".to_string(),
            remove_case_pairs("dabAcCaCBAcCcaDA".to_string())
        )
    }
}
