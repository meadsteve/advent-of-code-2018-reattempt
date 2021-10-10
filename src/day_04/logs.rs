use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::day_04::GuardNumber;

#[derive(Debug, Eq, PartialEq)]
pub enum LogEvent {
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
pub struct LogEntry {
    pub datetime: NaiveDateTime,
    pub event: LogEvent,
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
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::day_04::logs::LogEvent;
    use crate::day_04::*;

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
