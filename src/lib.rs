pub mod day_01;
pub mod day_02;
pub mod day_03;

mod helpers;

pub trait AdventDay {
    fn run_part_one(&self);
    fn run_part_two(&self);
}
