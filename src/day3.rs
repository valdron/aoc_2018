use aoc_runner_derive::*;
use core::str::FromStr;
use std::collections::HashMap;
use std::error::Error;

#[derive(PartialEq, Debug)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[aoc_generator(day3)]
fn gen_day2(input: &str) -> Vec<Claim> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
fn solve_day3_part1(input: &[Claim]) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    for pos in input.iter().flat_map(Claim::positions) {
        *map.entry(pos).or_insert(0) += 1;
        if map[&pos] == 2 {
            count += 1;
        }
    }
    count
}

#[aoc(day3, part2)]
fn solve_day3_part2(input: &[Claim]) -> i32 {
    let mut map = HashMap::new();
    for pos in input.iter().flat_map(Claim::positions) {
        *map.entry(pos).or_insert(0) += 1;
    }

    for claim in input {
        if claim.positions().all(|pos| map[&pos] == 1) {
            return claim.id;
        }
    }
    unreachable!();
}

impl Claim {
    fn positions(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (self.x..self.x + self.width)
            .flat_map(move |x| (self.y..self.y + self.height).map(move |y| (x, y)))
    }
}

#[test]
fn test_positions() {
    let claim = Claim {
        id: 0,
        x: 0,
        y: 0,
        width: 2,
        height: 1,
    };
    let poss: Vec<_> = claim.positions().collect();
    assert_eq!(vec![(0, 0), (1, 0)], poss);
}

impl FromStr for Claim {
    type Err = Box<Error>;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut iter = string.chars();
        let at = iter.position(|c| c == '@').unwrap();
        let comma = iter.position(|c| c == ',').unwrap() + at + 1;
        let colon = iter.position(|c| c == ':').unwrap() + comma + 1;
        let mul = iter.position(|c| c == 'x').unwrap() + colon + 1;

        let id = string[1..at - 1].parse()?;
        let x = string[at + 2..comma].parse()?;
        let y = string[comma + 1..colon].parse()?;
        let width = string[colon + 2..mul].parse()?;
        let height = string[mul + 1..].parse()?;

        Ok(Self {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        (Claim {
            id: 1,
            x: 1,
            y: 3,
            width: 4,
            height: 4
        }),
        "#1 @ 1,3: 4x4".parse().unwrap()
    );
}
