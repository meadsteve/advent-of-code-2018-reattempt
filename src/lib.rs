pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

mod helpers;

use day_01::DayOne;
use day_02::DayTwo;
use day_03::DayThree;
use day_04::DayFour;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::day_05::DayFive;
use std::str::FromStr;
use structopt::StructOpt;

pub trait AdventDay {
    fn run_part_one(&self) -> String;
    fn run_part_two(&self) -> String;
}

enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = &'static str;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "01" => Ok(Part::One),
            "02" => Ok(Part::Two),
            _ => Err("Only 01 or 02 is valid"),
        }
    }
}

#[derive(StructOpt)]
pub struct CliArgs {
    #[structopt(short, long)]
    day: i32,
    #[structopt(short, long)]
    part: Part,
}

#[derive(Debug)]
pub struct UnimplementedSolutionError(String);

impl Display for UnimplementedSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for UnimplementedSolutionError {}

pub fn run(args: &CliArgs, mut writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "Hello, advent of code")?;

    let solution: &(dyn AdventDay) = match args.day {
        1 => &DayOne(),
        2 => &DayTwo(),
        3 => &DayThree(),
        4 => &DayFour(),
        5 => &DayFive(),
        _ => {
            return Err(Box::new(UnimplementedSolutionError(
                "That day has not been done".to_string(),
            )))
        }
    };
    let result = run_day(solution, &args.part);
    writeln!(writer, "{}", result)?;
    Ok(())
}

fn run_day(solution: &dyn AdventDay, part: &Part) -> String {
    match part {
        Part::One => solution.run_part_one(),
        Part::Two => solution.run_part_two(),
    }
}
