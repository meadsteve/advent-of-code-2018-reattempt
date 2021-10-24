use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub struct DayData(String);

impl DayData {
    pub fn from_file_path(path: &str) -> DayData {
        DayData(path.to_string())
    }

    pub fn lines(&self) -> DayDataLineIterator {
        DayDataLineIterator::new_from_path(&self.0)
    }
}

pub struct DayDataLineIterator(String, Lines<BufReader<File>>);

impl DayDataLineIterator {
    fn new_from_path(path: &str) -> DayDataLineIterator {
        let f = File::open(path).expect("input not found");
        let f = BufReader::new(f);
        DayDataLineIterator(path.to_string(), f.lines())
    }
}

impl Clone for DayDataLineIterator {
    fn clone(&self) -> Self {
        DayDataLineIterator::new_from_path(&self.0)
    }
}

impl Iterator for DayDataLineIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|s| s.unwrap())
    }
}
