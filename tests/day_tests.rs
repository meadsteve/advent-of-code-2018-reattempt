use adventofcode2018_reattempt::day_01::DayOne;
use adventofcode2018_reattempt::day_02::DayTwo;
use adventofcode2018_reattempt::day_03::DayThree;
use adventofcode2018_reattempt::day_04::DayFour;
use adventofcode2018_reattempt::AdventDay;

use test_case::test_case;

#[test_case(Box::new(DayOne()), Some("Answer: 587"), Some("Answer: 83130"); "day one")]
#[test_case(Box::new(DayTwo()), Some("Solution: 246 x 35 = 8610"), Some("Pair different by one: iosnxmfkpabcjpdywvrtaqhluy and iosnxmfkpabcjpdywvrtawhluy = iosnxmfkpabcjpdywvrtahluy"); "day two")]
#[test_case(Box::new(DayThree()), Some("Double claimed: 116489"), Some("Single claims: {ClaimId(1260)}"); "day three")]
#[test_case(Box::new(DayFour()), None, None; "day four")]
fn test_the_days(
    solution: Box<dyn AdventDay>,
    day_one_sol: Option<&str>,
    day_two_sol: Option<&str>,
) {
    if let Some(expectation) = day_one_sol {
        assert_eq!(expectation, solution.run_part_one());
    }
    if let Some(expectation) = day_two_sol {
        assert_eq!(expectation, solution.run_part_two());
    }
}
