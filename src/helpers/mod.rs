use std::fs;

pub struct DayData(String);

impl DayData {
    pub fn from_file_path(path: &str) -> DayData {
        DayData(fs::read_to_string(path).expect("The input file was missing"))
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> + Clone + '_ {
        self.0.split('\n').filter(|&line| !line.is_empty())
    }
}
