use crate::helpers::DayData;
use crate::AdventDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct DayThree();

impl AdventDay for DayThree {
    fn run_part_one(&self) -> String {
        let lines = DayData::from_file_path("./data/day03.txt");
        let cloth = lines
            .iter()
            .map(parse_line)
            .fold(Cloth::new(), |mut cloth, (pos, size)| {
                cloth.claim_area(pos, size);
                cloth
            });
        format!("Double claimed: {}", cloth.double_claims().len())
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

fn parse_line(line: &str) -> (Position, Size) {
    lazy_static! {
        static ref LINE: Regex =
            Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    }
    if let Some(parsed) = LINE.captures(line) {
        (
            Position {
                x: parsed[2].parse::<usize>().unwrap(),
                y: parsed[3].parse::<usize>().unwrap(),
            },
            Size {
                height: parsed[5].parse::<usize>().unwrap(),
                width: parsed[4].parse::<usize>().unwrap(),
            },
        )
    } else {
        panic!("Invalid line");
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Size {
    width: usize,
    height: usize,
}

struct Cloth {
    claims: HashMap<Position, usize>,
}

impl Cloth {
    fn new() -> Cloth {
        Cloth {
            claims: HashMap::new(),
        }
    }

    fn claim(&mut self, position: Position) {
        let entry = self.claims.entry(position).or_insert(0);
        *entry += 1;
    }

    pub fn claim_area(&mut self, position: Position, size: Size) {
        for x in position.x..position.x + size.width {
            for y in position.y..position.y + size.height {
                self.claim(Position { x, y })
            }
        }
    }

    pub fn double_claims(&self) -> Vec<&Position> {
        self.claims
            .iter()
            .filter(|(_, &claims)| claims > 1)
            .map(|(pos, _)| pos)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_claim_has_no_doubles() {
        let mut cloth = Cloth::new();
        cloth.claim(Position { x: 1, y: 1 });
        assert_eq!(cloth.double_claims().len(), 0);
    }

    #[test]
    fn test_overlapping_claims_are_countable() {
        let mut cloth = Cloth::new();
        cloth.claim(Position { x: 1, y: 1 });
        cloth.claim(Position { x: 1, y: 2 });
        cloth.claim(Position { x: 1, y: 1 });
        assert_eq!(cloth.double_claims().len(), 1);
    }

    #[test]
    fn test_overlapping_area_claims_are_countable() {
        let mut cloth = Cloth::new();
        cloth.claim_area(
            Position { x: 1, y: 3 },
            Size {
                width: 4,
                height: 4,
            },
        );
        cloth.claim_area(
            Position { x: 3, y: 1 },
            Size {
                width: 4,
                height: 4,
            },
        );
        cloth.claim_area(
            Position { x: 5, y: 5 },
            Size {
                width: 2,
                height: 2,
            },
        );
        assert_eq!(cloth.double_claims().len(), 4);
    }

    #[test]
    fn line_parsing_works() {
        assert_eq!(
            (
                Position { x: 3, y: 2 },
                Size {
                    height: 4,
                    width: 5
                }
            ),
            parse_line("#123 @ 3,2: 5x4")
        )
    }
}
