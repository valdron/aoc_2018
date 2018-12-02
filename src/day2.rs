use aoc_runner_derive::*;
use std::collections::HashMap;

#[aoc_generator(day2)]
fn gen_day2(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[aoc(day2, part1)]
fn solve_day2_part1(input: &[Vec<u8>]) -> i32 {
    let mut dups = 0;
    let mut trips = 0;
    for v in input {
        let (has_dup, has_trip) = has_dup_trip(v);
        if has_dup {
            dups += 1;
        }
        if has_trip {
            trips += 1;
        }
    }
    dups * trips
}

#[aoc(day2, part2)]
fn solve_day2_part2(input: &[Vec<u8>]) -> String {
    let len = input[0].len();
    for v1 in input {
        for v2 in input {
            let common = common(v1, v2);
            if len - common.len() == 1 {
                println!(
                    "{}|{}",
                    String::from_utf8(v1.to_vec()).unwrap(),
                    String::from_utf8(v2.to_vec()).unwrap()
                );
                return String::from_utf8(common).unwrap();
            }
        }
    }
    unreachable!();
}

fn has_dup_trip(values: &[u8]) -> (bool, bool) {
    let mut map = HashMap::new();
    for v in values {
        *map.entry(v).or_insert(0) += 1;
    }
    let mut dup = false;
    let mut trip = false;
    for (_, v) in map {
        match v {
            2 => dup = true,
            3 => trip = true,
            _ => {}
        }
    }
    (dup, trip)
}

fn common(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter()
        .zip(right)
        .filter(|(l, r)| l == r)
        .map(|(l, _)| *l)
        .collect()
}

#[test]
fn test_has_dup_trip() {
    assert_eq!((false, false), has_dup_trip(&[1, 2, 3, 4, 5]));
    assert_eq!((true, false), has_dup_trip(&[1, 2, 2, 4, 5]));
    assert_eq!((false, true), has_dup_trip(&[1, 2, 2, 2, 5]));
    assert_eq!((true, true), has_dup_trip(&[1, 2, 2, 2, 1]));
}

#[test]
fn test_common() {
    assert_eq!(vec![1, 2, 3], common(&[1, 2, 3], &[1, 2, 3]));
    assert_eq!(vec![1, 3], common(&[1, 2, 3], &[1, 1, 3]));
    assert!(common(&[1, 2, 3], &[4, 5, 6]).is_empty());
}
