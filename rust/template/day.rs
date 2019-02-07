extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<String>;

fn part1(input: &Input) -> i32 {
    0
}

fn part2(input: &Input) -> i32 {
    0
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
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1337);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 1337);
    }
}