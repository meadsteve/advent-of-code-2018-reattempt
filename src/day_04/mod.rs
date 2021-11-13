use chrono::Timelike;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

mod logs;
use logs::LogEntry;

use crate::day_04::logs::LogEvent;
use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayFour();

impl AdventDay for DayFour {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day04.txt");
        let final_state = Self::process_data(data.lines());
        let sleepiest_guard = final_state.find_sleepiest_guard().guard_number;
        let sleepiest_minute = final_state.sleepiest_minute(&sleepiest_guard);
        format!(
            "Guard number {:?} at minute {}",
            &sleepiest_guard, sleepiest_minute
        )
    }

    fn run_part_two(&self) -> String {
        let data = DayData::from_file_path("./data/day04.txt");
        let final_state = Self::process_data(data.lines());
        let result = final_state.find_the_guard_with_a_very_sleepy_time_of_night();
        format!(
            "Guard number {:?} at minute {}",
            &result.guard_number, &result.sleepiest_minute
        )
    }
}

impl DayFour {
    pub fn process_data<T: Iterator<Item = String>>(lines: T) -> GuardsState {
        let mut entries: Vec<LogEntry> = lines
            .filter(|l| !l.is_empty())
            .map(LogEntry::from_line)
            .collect();
        entries.sort_by_key(|e| e.datetime);

        entries
            .iter()
            .fold(GuardsState::new(), GuardsState::apply_entry)
    }
}

/// Minute to count of asleep that minute
type SleepMap = HashMap<GuardNumber, HashMap<u32, u32>>;

#[derive(Debug, PartialEq)]
pub struct GuardsState {
    on_duty: Option<GuardNumber>,
    fell_asleep_at: Option<u32>,
    sleep_map: SleepMap,
}

pub struct TotalSleepStats {
    total_minutes_slept: u32,
    guard_number: GuardNumber,
}

impl From<(&GuardNumber, &HashMap<u32, u32>)> for TotalSleepStats {
    fn from((guard, sleep_map): (&GuardNumber, &HashMap<u32, u32>)) -> Self {
        let total_minutes_slept = sleep_map.values().sum();
        TotalSleepStats {
            guard_number: guard.clone(),
            total_minutes_slept,
        }
    }
}

pub struct SleepiestMinute {
    times_slept: u32,
    sleepiest_minute: u32,
    guard_number: GuardNumber,
}

impl From<(&GuardNumber, &HashMap<u32, u32>)> for SleepiestMinute {
    fn from((guard, sleep_map): (&GuardNumber, &HashMap<u32, u32>)) -> Self {
        let (sleepiest_minute, times_slept) =
            sleep_map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        SleepiestMinute {
            guard_number: guard.clone(),
            sleepiest_minute: *sleepiest_minute,
            times_slept: *times_slept,
        }
    }
}

impl GuardsState {
    pub fn new() -> GuardsState {
        GuardsState::default()
    }

    fn apply_entry(mut self, entry: &LogEntry) -> GuardsState {
        match &entry.event {
            LogEvent::Awakes => {
                let guard = self.on_duty.clone().unwrap();
                let sleep_start = self.fell_asleep_at.unwrap();

                let guard_sleeps = self.sleep_map.entry(guard).or_insert_with(HashMap::new);
                for minute in sleep_start..(entry.datetime.minute()) {
                    let current_count = guard_sleeps.entry(minute).or_insert(0);
                    *current_count += 1;
                }
                self.fell_asleep_at = None;
            }
            LogEvent::Sleeps => self.fell_asleep_at = Some(entry.datetime.minute()),
            LogEvent::ShiftStarts(guard) => self.on_duty = Some(guard.clone()),
        };
        self
    }

    fn find_sleepiest_guard(&self) -> TotalSleepStats {
        self.sleep_map
            .iter()
            .map(TotalSleepStats::from)
            .max_by(|a, b| a.total_minutes_slept.cmp(&b.total_minutes_slept))
            .unwrap()
    }

    fn sleepiest_minute(&self, guard: &GuardNumber) -> u32 {
        let (&sleepiest_minute, _) = self.sleep_map[guard]
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        sleepiest_minute
    }

    fn find_the_guard_with_a_very_sleepy_time_of_night(&self) -> SleepiestMinute {
        self.sleep_map
            .iter()
            .map(SleepiestMinute::from)
            .max_by(|a, b| a.times_slept.cmp(&b.times_slept))
            .unwrap()
    }
}

impl Default for GuardsState {
    fn default() -> Self {
        GuardsState {
            on_duty: None,
            fell_asleep_at: None,
            sleep_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct GuardNumber(usize);

impl FromStr for GuardNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_number = s.parse::<usize>()?;
        Ok(GuardNumber(id_number))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_04::*;

    fn sample_lines() -> Box<dyn Iterator<Item = String>> {
        let input = r#"
            [1518-11-01 00:00] Guard #10 begins shift
            [1518-11-01 00:05] falls asleep
            [1518-11-01 00:25] wakes up
            [1518-11-01 00:30] falls asleep
            [1518-11-01 00:55] wakes up
            [1518-11-01 23:58] Guard #99 begins shift
            [1518-11-02 00:40] falls asleep
            [1518-11-02 00:50] wakes up
            [1518-11-03 00:05] Guard #10 begins shift
            [1518-11-03 00:24] falls asleep
            [1518-11-03 00:29] wakes up
            [1518-11-04 00:02] Guard #99 begins shift
            [1518-11-04 00:36] falls asleep
            [1518-11-04 00:46] wakes up
            [1518-11-05 00:03] Guard #99 begins shift
            [1518-11-05 00:45] falls asleep
            [1518-11-05 00:55] wakes up
        "#;
        Box::new(input.lines().map(|x| x.trim()).map(|x| x.to_string()))
    }

    #[test]
    fn test_finds_the_sleepiest_guard() {
        assert_eq!(
            GuardNumber(10),
            DayFour::process_data(sample_lines())
                .find_sleepiest_guard()
                .guard_number
        );
    }

    #[test]
    fn test_find_the_guard_with_the_sleepiest_minute() {
        assert_eq!(
            GuardNumber(99),
            DayFour::process_data(sample_lines())
                .find_the_guard_with_a_very_sleepy_time_of_night()
                .guard_number
        );
    }

    #[test]
    fn test_finds_the_sleepiest_minute_for_a_gaurd() {
        assert_eq!(
            24,
            DayFour::process_data(sample_lines()).sleepiest_minute(&GuardNumber(10))
        );
    }
}
