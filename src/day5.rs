use aoc_runner_derive::*;
use core::iter::FromIterator;

#[aoc(day5, part1)]
fn solve_day5_part1(input: &[u8]) -> usize {
    let stack: PolymerStack = input.iter().cloned().collect();
    stack.0.len()
}

#[aoc(day5, part2)]
fn solve_day5_part2(input: &[u8]) -> usize {
    (b'a'..=b'z')
        .map(|i| {
            input
                .iter()
                .cloned()
                .filter(|c| *c | 0b_0010_0000 != i)
                .collect::<PolymerStack>()
                .0
                .len()
        })
        .min()
        .unwrap()
}

impl FromIterator<u8> for PolymerStack {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut stack = PolymerStack(Vec::with_capacity(iter.size_hint().1.unwrap_or(0)));
        for i in iter {
            stack.push(i);
        }
        stack
    }
}

struct PolymerStack(pub Vec<u8>);

impl PolymerStack {
    fn push(&mut self, c: u8) {
        match self.0.last() {
            Some(&top_char) if top_char == c ^ 32 => {
                self.0.pop();
            }
            _ => self.0.push(c),
        }
    }
}

#[test]
fn test_stack() {
    let mut stack = PolymerStack(vec![]);

    for c in "dabAcCaCBAcCcaDA".bytes() {
        stack.push(c)
    }

    assert_eq!(stack.0.len(), 10);
}
