extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Present>;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Present {
    l: u32,
    w: u32,
    h: u32
}

impl FromStr for Present {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<u32>().unwrap();
        Ok(Present { l: get(1), w: get(2), h: get(3) })
    }
}

impl Present {
    fn wrapping_needed(&self) -> u32 {
        let Present { l, w, h } = self;
        let sides = [l * w, w * h, h * l];
        sides.iter().map(|s| s * 2).sum::<u32>() + sides.iter().min().unwrap()
    }
    fn ribbon_needed(&self) -> u32 {
        let Present { l, w, h } = self;
        let mut sides = [l, w, h];
        sides.sort();
        &sides[..2].iter().map(|&s| s + s).sum::<u32>() + (l * w * h)
    }
}

fn part1(input: &Input) -> u32 {
    input.iter().map(Present::wrapping_needed).sum()
}

fn part2(input: &Input) -> u32 {
    input.iter().map(Present::ribbon_needed).sum()
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
    Ok(f.lines().map(|l| l.unwrap().parse::<Present>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "2x3x4
        1x1x10";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<Present>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 58 + 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 34 + 14);
    }
}