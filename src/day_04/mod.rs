use std::num::ParseIntError;
use std::str::FromStr;

mod logs;
use logs::LogEntry;

use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayFour();

impl AdventDay for DayFour {
    fn run_part_one(&self) -> String {
        let mut entries: Vec<LogEntry> = DayData::from_file_path("./data/day04.txt")
            .lines()
            .map(LogEntry::from_line)
            .collect();
        entries.sort_by_key(|e| e.datetime);
        let entries = entries;
        println!("{:?}", entries[0]);
        println!("{:?}", entries[1]);
        println!("{:?}", entries[2]);
        "errm".to_string()
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GuardNumber(usize);

impl FromStr for GuardNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_number = s.parse::<usize>()?;
        Ok(GuardNumber(id_number))
    }
}
