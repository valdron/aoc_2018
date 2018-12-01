use std::collections::HashSet;
use std::hash::Hash;
use std::ops::AddAssign;

use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn gen_day1(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_day1_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn solve_day1_part2(input: &[i32]) -> i32 {
    let mut sum = 0;
    let mut seen = HashSet::new();
    seen.insert(sum);
    for i in input.iter().cycle() {
        sum += i;
        if seen.contains(&sum) {
            break;
        } else {
            seen.insert(sum);
        }
    }
    sum
}

#[aoc(day1, part2, CustomIter)]
fn solve_day1_part2_customiter(input: &[i32]) -> i32 {
    get_first_duplicate(get_summed_iter(input.iter().cycle().cloned())).unwrap()
}

fn get_summed_iter<I, N>(iter: I) -> impl Iterator<Item = N>
where
    I: Iterator<Item = N>,
    N: AddAssign + Default + Copy,
{
    let mut sum = Default::default();
    iter.map(move |item| {
        sum += item;
        sum
    })
}

fn get_first_duplicate<I, D>(iter: I) -> Option<D>
where
    I: Iterator<Item = D>,
    D: Eq + Hash,
{
    let mut set = HashSet::new();
    for i in iter {
        if set.contains(&i) {
            return Some(i);
        } else {
            set.insert(i);
        }
    }
    None
}

#[test]
fn test_day1_part2() {
    const TEST: &[i32] = &[7, 7, -2, -7, -4];
    assert_eq!(14, solve_day1_part2(TEST));
}

#[test]
fn test_day1_part2_customiter() {
    const TEST: &[i32] = &[7, 7, -2, -7, -4];
    assert_eq!(14, solve_day1_part2_customiter(TEST));
}
