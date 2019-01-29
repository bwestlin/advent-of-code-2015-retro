extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<char>;

fn part1(input: &Input) -> i32 {
    input.iter().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => unreachable!(),
        }
    })
}

fn part2(input: &Input) -> usize {
    input.iter().enumerate().scan((0i32, false), |(floor, done), (i, c)| {
        if *done {
            return None;
        }
        match c {
            '(' => *floor += 1,
            ')' => *floor -= 1,
            _ => unreachable!(),
        }

        if *floor == -1 {
            *done = true;
        }
        Some(i + 1)
    })
    .last()
    .unwrap()
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
    Ok(f.lines().map(|l| l.unwrap()).next().unwrap().chars().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        s.chars().collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input("(())")), 0);
        assert_eq!(part1(&as_input("()()")), 0);
        assert_eq!(part1(&as_input("(((")), 3);
        assert_eq!(part1(&as_input("(()(()(")), 3);
        assert_eq!(part1(&as_input("))(((((")), 3);
        assert_eq!(part1(&as_input("())")), -1);
        assert_eq!(part1(&as_input("))(")), -1);
        assert_eq!(part1(&as_input(")))")), -3);
        assert_eq!(part1(&as_input(")())())")), -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(")")), 1);
        assert_eq!(part2(&as_input("()())")), 5);
    }
}