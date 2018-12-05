use aoc_runner_derive::*;
use core::iter::FromIterator;

#[aoc(day5, part1)]
fn solve_day5_part1(input: &[u8]) -> usize {
    let stack: PolymerStack = input.iter().collect();
    stack.len()
}

#[aoc(day5, part2)]
fn solve_day5_part2(input: &[u8]) -> usize {
    (b'a'..=b'z')
        .map(|i| {
            input
                .iter()
                .filter(|c| !c.eq_ignore_ascii_case(&i))
                .collect::<PolymerStack>()
                .len()
        })
        .min()
        .unwrap()
}

impl<'a> FromIterator<&'a u8> for PolymerStack {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut stack = PolymerStack::with_capacity(iter.size_hint().1.unwrap_or(0));
        for i in iter {
            stack.push(*i);
        }
        stack
    }
}

struct PolymerStack(Vec<u8>);

impl PolymerStack {
    fn with_capacity(cap: usize) -> Self {
        Self {
            0: Vec::with_capacity(cap),
        }
    }

    fn push(&mut self, c: u8) {
        match self.0.last() {
            Some(&top_char) if top_char == c ^ 32 => {
                self.0.pop();
            }
            _ => self.0.push(c),
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn test_stack() {
    let stack = "dabAcCaCBAcCcaDA".bytes().collect::<PolymerStack>();

    assert_eq!(stack.len(), 10);
}
