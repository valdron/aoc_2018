use aoc_runner_derive::*;

#[aoc(day5, part1)]
fn solve_day5_part1(input: &[u8]) -> usize {
    let mut stack = PolymerStack(vec![]);
    for c in input.into_iter().filter(|c| c.is_ascii_alphabetic()) {
        stack.push(*c);
    }
    stack.0.len()
}

#[aoc(day5, part2)]
fn solve_day5_part2(input: &[u8]) -> usize {
    let mut min = usize::max_value();
    for i in b'a'..=b'z' {
        let mut stack = PolymerStack(vec![]);
        for c in input
            .into_iter()
            .filter(|c| c.is_ascii_alphabetic() && c != &&i && c != &&i.to_ascii_uppercase())
        {
            stack.push(*c);
        }
        if stack.0.len() < min {
            min = stack.0.len()
        }
    }
    min
}

struct PolymerStack(pub Vec<u8>);

impl PolymerStack {
    fn push(&mut self, c: u8) {
        if let Some(&top_char) = self.0.last() {
            if top_char != c
                && (top_char == c.to_ascii_lowercase() || top_char == c.to_ascii_uppercase())
            {
                self.0.pop();
                return;
            }
        }
        self.0.push(c);
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
