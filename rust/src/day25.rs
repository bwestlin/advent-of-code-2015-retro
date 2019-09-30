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

struct Input {
    row: u32,
    col: u32
}

impl FromStr for Input {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^.* (\d+),.* (\d+).*$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<u32>();
        Ok(Input { row: get(1)?, col: get(2)? })
    }
}

fn nth_code(col: u32, row: u32) -> u32 {
    let n = col + row - 1;
    ((n * (n + 1)) / 2) - (row - 1)
}

fn code(col: u32, row: u32) -> u64 {
    (1..nth_code(col, row))
        .fold(20151125_u64, |code, _| {
            (code * 252533) % 33554393
        })
}

fn part1(input: &Input) -> u64 {
    code(input.col, input.row)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap().map(|l| l.parse::<Input>().unwrap()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "To continue, please consult the code grid in the manual.  Enter the code at row 6, column 6.";

    fn as_input(s: &str) -> Input {
        s.split('\n').next().map(|s| s.parse::<Input>().unwrap()).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 27995004);
    }
}