use crate::helpers::DayData;
use crate::AdventDay;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct DayFour();

impl AdventDay for DayFour {
    fn run_part_one(&self) -> String {
        let mut entries: Vec<LogEntry> = DayData::from_file_path("./data/day04.txt")
            .iter()
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
struct GuardNumber(usize);

impl FromStr for GuardNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_number = s.parse::<usize>()?;
        Ok(GuardNumber(id_number))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum LogEvent {
    Awakes,
    Sleeps,
    ShiftStarts(GuardNumber),
}

impl LogEvent {
    fn from_entry_string(entry: &str) -> LogEvent {
        lazy_static! {
            static ref SHIFT_START: Regex = Regex::new("Guard #([0-9]+) begins shift").unwrap();
        }
        match entry {
            "wakes up" => LogEvent::Awakes,
            "falls asleep" => LogEvent::Sleeps,
            shift_start => match SHIFT_START.captures(shift_start) {
                Some(captures) => {
                    LogEvent::ShiftStarts(GuardNumber::from_str(&captures[1]).unwrap())
                }
                None => panic!("What kind of line was that"),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LogEntry {
    datetime: NaiveDateTime,
    event: LogEvent,
}

impl LogEntry {
    pub fn from_line(line: &str) -> LogEntry {
        lazy_static! {
            static ref LOG_ENTRY: Regex = Regex::new("\\[(.+)\\] (.+)").unwrap();
        }
        let parts = LOG_ENTRY.captures(line).unwrap();
        LogEntry {
            datetime: NaiveDateTime::parse_from_str(&parts[1], "%Y-%m-%d %H:%M").unwrap(),
            event: LogEvent::from_entry_string(&parts[2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_line_parser_for_sleeping() {
        let expected = LogEntry {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 1),
                NaiveTime::from_hms(0, 5, 0),
            ),
            event: LogEvent::Sleeps,
        };
        assert_eq!(
            LogEntry::from_line("[1518-11-01 00:05] falls asleep"),
            expected
        );
    }

    #[test]
    fn test_line_parser_for_waking() {
        let expected = LogEntry {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 1),
                NaiveTime::from_hms(0, 10, 0),
            ),
            event: LogEvent::Awakes,
        };
        assert_eq!(LogEntry::from_line("[1518-11-01 00:10] wakes up"), expected);
    }

    #[test]
    fn test_line_parser_for_shift_starts() {
        let expected = LogEntry {
            datetime: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 1),
                NaiveTime::from_hms(0, 30, 0),
            ),
            event: LogEvent::ShiftStarts(GuardNumber(5)),
        };
        assert_eq!(
            LogEntry::from_line("[1518-11-01 00:30] Guard #5 begins shift"),
            expected
        );
    }
}
