use adventofcode2018_reattempt::day_01::DayOne;
use adventofcode2018_reattempt::day_02::DayTwo;
use adventofcode2018_reattempt::day_03::DayThree;
use adventofcode2018_reattempt::day_04::DayFour;
use adventofcode2018_reattempt::AdventDay;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    day: String,
    part: String,
}

fn main() {
    println!("Hello, advent of code");
    let args: CliArgs = CliArgs::from_args();

    let solution: &(dyn AdventDay) = match &args.day[..] {
        "01" => &DayOne(),
        "02" => &DayTwo(),
        "03" => &DayThree(),
        "04" => &DayFour(),
        _ => panic!("That day has not been done"),
    };
    let result = run_day(solution, &args.part[..]);
    println!("{}", result);
}

fn run_day(solution: &dyn AdventDay, part: &str) -> String {
    match part {
        "01" => solution.run_part_one(),
        "02" => solution.run_part_two(),
        _ => panic!("Day must be 01 or 02"),
    }
}
