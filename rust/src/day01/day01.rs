extern crate utils;

use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;


fn part1(input: &Vec<char>) -> i32 {
    input.iter().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => unreachable!(),
        }
    })
}

fn part2(input: &Vec<char>) -> usize {
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

fn main() -> Result<(), Box<Error>> {
    let input = input()?;
    measure_exec(|| {
        let result = part1(&input);
        println!("Part1 result: {}", result);
        Ok(())
    })?;
    measure_exec(|| {
        let result = part2(&input);
        println!("Part2 result: {}", result);
        Ok(())
    })?;
    Ok(())
}

fn input() -> io::Result<Vec<char>> {
    let f = File::open("src/day01/input")?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).next().unwrap().chars().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Vec<char> {
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