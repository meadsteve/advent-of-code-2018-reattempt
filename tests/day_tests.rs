use adventofcode2018_reattempt::day_01::DayOne;
use adventofcode2018_reattempt::day_02::DayTwo;
use adventofcode2018_reattempt::AdventDay;

#[test]
fn day_01_part_01() {
    assert_eq!("Answer: 587", DayOne().run_part_one());
}

#[test]
fn day_01_part_02() {
    assert_eq!("Answer: 83130", DayOne().run_part_two());
}

#[test]
fn day_02_part_01() {
    assert_eq!("Solution: 246 x 35 = 8610", DayTwo().run_part_one());
}

#[test]
fn day_02_part_02() {
    assert_eq!("Pair different by one: iosnxmfkpabcjpdywvrtaqhluy and iosnxmfkpabcjpdywvrtawhluy = iosnxmfkpabcjpdywvrtahluy", DayTwo().run_part_two());
}
