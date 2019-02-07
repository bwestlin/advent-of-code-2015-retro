extern crate utils;

use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

fn deliver_presents(input: &Input, num_deliverers: usize) -> usize {
    let start = (0, 0);
    let mut deliverer_pos = VecDeque::new();
    for _ in 0..num_deliverers {
        deliverer_pos.push_back(start);
    }
    let mut visited = HashSet::new();
    visited.insert(start);

    for c in input.chars() {
        let (x, y) = deliverer_pos.pop_front().unwrap();
        let next_pos = match c {
            '^' => (x, y - 1),
            '>' => (x + 1, y),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            _ => unreachable!()
        };
        visited.insert(next_pos);
        deliverer_pos.push_back(next_pos);
    }

    visited.len()
}

fn part1(input: &Input) -> usize {
    deliver_presents(input, 1)
}

fn part2(input: &Input) -> usize {
    deliver_presents(input, 2)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        s.into()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(">")), 2);
        assert_eq!(part1(&as_input("^>v<")), 4);
        assert_eq!(part1(&as_input("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input("^v")), 3);
        assert_eq!(part2(&as_input("^>v<")), 3);
        assert_eq!(part2(&as_input("^v^v^v^v^v")), 11);
    }
}