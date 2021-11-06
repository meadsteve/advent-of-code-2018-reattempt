use std::sync::{Arc, Mutex};

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
