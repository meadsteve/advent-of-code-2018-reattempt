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
            .fold(Cloth::new(), |mut cloth, claim| {
                cloth.claim_area(claim);
                cloth
            });
        format!("Double claimed: {}", cloth.double_claims().len())
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

fn parse_line(line: &str) -> Claim {
    lazy_static! {
        static ref LINE: Regex =
            Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    }
    if let Some(parsed) = LINE.captures(line) {
        Claim {
            id: parsed[1].parse::<usize>().unwrap(),
            pos: Position {
                x: parsed[2].parse::<usize>().unwrap(),
                y: parsed[3].parse::<usize>().unwrap(),
            },
            size: Size {
                height: parsed[5].parse::<usize>().unwrap(),
                width: parsed[4].parse::<usize>().unwrap(),
            },
        }
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

#[derive(Hash, PartialEq, Eq, Debug)]
struct Claim {
    id: usize,
    pos: Position,
    size: Size,
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

    pub fn claim_area(&mut self, claim: Claim) {
        let pos = claim.pos;
        let size = claim.size;
        for x in pos.x..pos.x + size.width {
            for y in pos.y..pos.y + size.height {
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
    fn test_overlapping_area_claims_are_countable() {
        let mut cloth = Cloth::new();
        cloth.claim_area(Claim {
            id: 1,
            pos: Position { x: 1, y: 3 },
            size: Size {
                height: 4,
                width: 4,
            },
        });
        cloth.claim_area(Claim {
            id: 2,
            pos: Position { x: 3, y: 1 },
            size: Size {
                height: 4,
                width: 4,
            },
        });
        cloth.claim_area(Claim {
            id: 3,
            pos: Position { x: 5, y: 5 },
            size: Size {
                height: 2,
                width: 2,
            },
        });
        assert_eq!(cloth.double_claims().len(), 4);
    }

    #[test]
    fn line_parsing_works() {
        assert_eq!(
            Claim {
                id: 123,
                pos: Position { x: 3, y: 2 },
                size: Size {
                    height: 4,
                    width: 5
                }
            },
            parse_line("#123 @ 3,2: 5x4")
        )
    }
}
