use adventofcode2018_reattempt::day_01::DayOne;
use adventofcode2018_reattempt::AdventDay;
use std::env;

fn main() {
    println!("Hello, advent of code");
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];

    match &day[..] {
        "01" => run_day(&DayOne(), part),
        _ => panic!("That day has not been done"),
    };
}

fn run_day(solution: &dyn AdventDay, part: &str) {
    match &part[..] {
        "01" => solution.run_part_one(),
        "02" => solution.run_part_two(),
        _ => panic!("Day must be 01 or 02"),
    };
}
