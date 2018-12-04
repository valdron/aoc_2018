use aoc_runner_derive::*;
use core::str::FromStr;
use std::collections::HashMap;
use std::error::Error;

enum Event {
    NewGuard(usize),
    Sleeping(usize),
    Waking(usize),
}

impl FromStr for Event {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ref l if l.ends_with("shift") => Event::NewGuard(
                l.split(|c| c == '#' || c == ' ')
                    .nth(4)
                    .ok_or(l.clone())?
                    .parse()?,
            ),
            ref l if l.ends_with("asleep") => Event::Sleeping(
                l.split(|c| c == ']' || c == ':')
                    .nth(1)
                    .ok_or(l.clone())?
                    .parse()?,
            ),
            ref l if l.ends_with("up") => Event::Waking(
                l.split(|c| c == ']' || c == ':')
                    .nth(1)
                    .ok_or(l.clone())?
                    .parse()?,
            ),
            _ => panic!("parse_error"),
        })
    }
}

#[aoc_generator(day4)]
fn gen_day4(input: &str) -> Vec<Event> {
    let mut v: Vec<_> = input.lines().collect();
    v.sort();
    v.iter()
        .map(|s| Event::from_str(s))
        .map(Result::unwrap)
        .collect()
}

#[aoc(day4, part1)]
fn solve_day4_part1(input: &[Event]) -> i32 {
    let iter = input.into_iter().skip_while(|s| {
        if let Event::NewGuard(_) = s {
            true
        } else {
            false
        }
    });
    let mut guardid = 0;
    let mut map = HashMap::new();
    let mut asleep_since = 0;

    for line in iter {
        match line {
            Event::NewGuard(id) => {
                guardid = *id;
            }
            Event::Sleeping(min) => {
                asleep_since = *min;
            }
            Event::Waking(min) => {
                map.entry(guardid).or_insert(vec![0; 60])[asleep_since..*min]
                    .iter_mut()
                    .for_each(|min| *min += 1);
            }
        }
    }

    let (id, v) = map
        .iter()
        .max_by(|a, b| {
            let a = a.1.iter().sum::<i32>();
            let b = b.1.iter().sum::<i32>();
            a.cmp(&b)
        })
        .unwrap();
    let max = *v.iter().max().unwrap();
    (id * v.iter().position(|i| *i == max).unwrap()) as i32
}

#[aoc(day4, part2)]
fn solve_day4_part2(input: &[Event]) -> i32 {
    let iter = input.into_iter().skip_while(|s| {
        if let Event::NewGuard(_) = s {
            true
        } else {
            false
        }
    });
    let mut guardid = 0;
    let mut map = HashMap::new();
    let mut asleep_since = 0;

    for line in iter {
        match line {
            Event::NewGuard(id) => {
                guardid = *id;
            }
            Event::Sleeping(min) => {
                asleep_since = *min;
            }
            Event::Waking(min) => {
                map.entry(guardid).or_insert(vec![0; 60])[asleep_since..*min]
                    .iter_mut()
                    .for_each(|min| *min += 1);
            }
        }
    }

    let (id, v) = map
        .iter()
        .max_by(|a, b| {
            let a = a.1.iter().max();
            let b = b.1.iter().max();
            a.cmp(&b)
        })
        .unwrap();
    let max = *v.iter().max().unwrap();
    (id * v.iter().position(|i| *i == max).unwrap()) as i32
}

#[test]
fn test() {
    const TEST: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let input = gen_day4(TEST);
    assert_eq!(240, solve_day4_part1(&input));
    assert_eq!(4455, solve_day4_part2(&input));
}
