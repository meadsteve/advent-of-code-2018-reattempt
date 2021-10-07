use std::fs;

pub mod day_01;

pub trait AdventDay {
    fn run_part_one(&self);
    fn run_part_two(&self);
}

pub struct FileLines(String);

impl FileLines {
    fn iter(&self) -> impl Iterator<Item = &str> + '_ {
        self.0.split('\n').filter(|&line| !line.is_empty())
    }
}

pub fn lines_from_file(path: &str) -> FileLines {
    FileLines(fs::read_to_string(path).expect("The input file was missing"))
}
