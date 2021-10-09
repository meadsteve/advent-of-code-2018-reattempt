use adventofcode2018_reattempt::day_01::DayOne;
use adventofcode2018_reattempt::day_02::DayTwo;
use adventofcode2018_reattempt::day_03::DayThree;
use adventofcode2018_reattempt::AdventDay;
use std::env;

fn main() {
    println!("Hello, advent of code");
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];

    let result = match &day[..] {
        "01" => run_day(&DayOne(), part),
        "02" => run_day(&DayTwo(), part),
        "03" => run_day(&DayThree(), part),
        _ => panic!("That day has not been done"),
    };
    println!("{}", result);
}

fn run_day(solution: &dyn AdventDay, part: &str) -> String {
    match part {
        "01" => solution.run_part_one(),
        "02" => solution.run_part_two(),
        _ => panic!("Day must be 01 or 02"),
    }
}
