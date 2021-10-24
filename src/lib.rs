pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

mod helpers;

use day_01::DayOne;
use day_02::DayTwo;
use day_03::DayThree;
use day_04::DayFour;

use structopt::StructOpt;

pub trait AdventDay {
    fn run_part_one(&self) -> String;
    fn run_part_two(&self) -> String;
}

#[derive(StructOpt)]
pub struct CliArgs {
    #[structopt(short, long)]
    day: String,
    #[structopt(short, long)]
    part: String,
}

pub fn run(
    args: &CliArgs,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "Hello, advent of code")?;

    let solution: &(dyn AdventDay) = match &args.day[..] {
        "01" => &DayOne(),
        "02" => &DayTwo(),
        "03" => &DayThree(),
        "04" => &DayFour(),
        _ => panic!("That day has not been done"),
    };
    let result = run_day(solution, &args.part[..]);
    writeln!(writer, "{}", result)?;
    Ok(())
}

fn run_day(solution: &dyn AdventDay, part: &str) -> String {
    match part {
        "01" => solution.run_part_one(),
        "02" => solution.run_part_two(),
        _ => panic!("Day must be 01 or 02"),
    }
}
