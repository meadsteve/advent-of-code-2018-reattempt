use crate::helpers::DayData;
use crate::AdventDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct DayThree();

impl AdventDay for DayThree {
    fn run_part_one(&self) -> String {
        let lines = DayData::from_file_path("./data/day03.txt");
        let cloth = Cloth::from(lines);
        format!("Double claimed: {}", cloth.double_claimed_positions().len())
    }

    fn run_part_two(&self) -> String {
        let lines = DayData::from_file_path("./data/day03.txt");
        let cloth = Cloth::from(lines);
        format!("Single claims: {:?}", cloth.single_claimants())
    }
}

impl From<DayData> for Cloth {
    fn from(lines: DayData) -> Self {
        lines
            .lines()
            .map(|l| l.parse().unwrap())
            .fold(Cloth::new(), |mut cloth, claim| {
                cloth.claim_area(claim);
                cloth
            })
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct ClaimId(usize);

impl FromStr for ClaimId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClaimId(s.parse::<usize>()?))
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Claim {
    id: ClaimId,
    pos: Position,
    size: Size,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE: Regex =
                Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
        }
        if let Some(parsed) = LINE.captures(s) {
            Ok(Claim {
                id: parsed[1].parse().unwrap(),
                pos: Position {
                    x: parsed[2].parse::<usize>().unwrap(),
                    y: parsed[3].parse::<usize>().unwrap(),
                },
                size: Size {
                    height: parsed[5].parse::<usize>().unwrap(),
                    width: parsed[4].parse::<usize>().unwrap(),
                },
            })
        } else {
            Err("Invalid line".to_string())
        }
    }
}

struct Cloth {
    claims: HashMap<Position, HashSet<ClaimId>>,
    all_claimants: HashSet<ClaimId>,
}

impl Cloth {
    fn new() -> Cloth {
        Cloth {
            claims: HashMap::new(),
            all_claimants: HashSet::new(),
        }
    }

    fn claim(&mut self, position: Position, claim_id: ClaimId) {
        self.all_claimants.insert(claim_id);
        let entry = self.claims.entry(position).or_insert_with(HashSet::new);
        entry.insert(claim_id);
    }

    pub fn claim_area(&mut self, claim: Claim) {
        let pos = claim.pos;
        let size = claim.size;
        for x in pos.x..pos.x + size.width {
            for y in pos.y..pos.y + size.height {
                self.claim(Position { x, y }, claim.id)
            }
        }
    }

    pub fn double_claimed_positions(&self) -> Vec<&Position> {
        self.claims
            .iter()
            .filter(|(_, claims)| claims.len() > 1)
            .map(|(pos, _)| pos)
            .collect()
    }

    pub fn double_claimants(&self) -> HashSet<ClaimId> {
        self.claims
            .iter()
            .filter(|(_, claims)| claims.len() > 1)
            .flat_map(|(_, ids)| ids)
            .cloned()
            .collect()
    }

    pub fn single_claimants(&self) -> HashSet<ClaimId> {
        let double_claimants = self.double_claimants();
        self.all_claimants
            .difference(&double_claimants)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping_area_claims_are_countable() {
        let mut cloth = Cloth::new();
        cloth.claim_area("#1 @ 1,3: 4x4".parse().unwrap());
        cloth.claim_area("#2 @ 3,1: 4x4".parse().unwrap());
        cloth.claim_area("#3 @ 5,5: 2x2".parse().unwrap());
        assert_eq!(cloth.double_claimed_positions().len(), 4);
    }

    #[test]
    fn test_single_claimaints_can_be_found() {
        let mut cloth = Cloth::new();
        cloth.claim_area("#1 @ 1,3: 4x4".parse().unwrap());
        cloth.claim_area("#2 @ 3,1: 4x4".parse().unwrap());
        cloth.claim_area("#3 @ 5,5: 2x2".parse().unwrap());

        let mut expected = HashSet::new();
        expected.insert(ClaimId(3));

        assert_eq!(cloth.single_claimants(), expected);
    }

    #[test]
    fn line_parsing_works() {
        assert_eq!(
            Claim {
                id: ClaimId(123),
                pos: Position { x: 3, y: 2 },
                size: Size {
                    height: 4,
                    width: 5
                }
            },
            "#123 @ 3,2: 5x4".parse::<Claim>().unwrap()
        )
    }
}
