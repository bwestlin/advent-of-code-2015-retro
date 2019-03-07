extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::error;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Instruction>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operation {
    TurnOn,
    Toggle,
    TurnOff
}

type OperationFn = Fn(&mut [u8]) -> ();

#[derive(Debug, Clone)]
pub struct OperationParseError;
impl fmt::Display for OperationParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse operation")
    }
}
impl error::Error for OperationParseError {
    fn description(&self) -> &str {
        "unable to parse operation"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for Operation {
    type Err = OperationParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        match s {
            "turn on" => Ok(TurnOn),
            "toggle" => Ok(Toggle),
            "turn off" => Ok(TurnOff),
            _ => Err(OperationParseError {})
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Instruction {
    op: Operation,
    s_x: usize,
    s_y: usize,
    e_x: usize,
    e_y: usize
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(turn on|toggle|turn off).(\d+),(\d+).+ (\d+),(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<usize>().unwrap();
        let op = caps.get(1).unwrap().as_str().parse::<Operation>().unwrap();
        Ok(Instruction { op: op, s_x: get(2), s_y: get(3), e_x: get(4), e_y: get(5) })
    }
}

struct Grid {
    lights: [[u8; 1000]; 1000]
}

impl Grid {
    fn new() -> Grid {
        Grid { lights: [[0; 1000]; 1000] }
    }

    fn apply(&mut self, ins: &Instruction, op_fn: &OperationFn) {
        for y in ins.s_y..=ins.e_y {
            let l = &mut self.lights[y][ins.s_x..=ins.e_x];
            op_fn(l);
        }
    }

    fn n_lit(&self) -> usize {
        self.lights.iter().map(|lr| lr.iter().filter(|&&l| l > 0).count()).sum()
    }

    fn brightness(&self) -> usize {
        self.lights.iter().map(|lr| lr.iter().map(|l| *l as usize).sum::<usize>()).sum()
    }
}

mod part1 {
    use super::*;

    fn turn_on(l: &mut [u8]) {
        for i in 0..l.len() {
            l[i] = 1;
        }
    }

    fn toggle(l: &mut [u8]) {
        for i in 0..l.len() {
            l[i] = if l[i] > 0 { 0 } else { 1 };
        }
    }

    fn turn_off(l: &mut [u8]) {
        for i in 0..l.len() {
            l[i] = 0;
        }
    }

    pub fn op_fn(op: &Operation) -> &OperationFn {
        use Operation::*;
        match op {
            TurnOn => &turn_on,
            Toggle => &toggle,
            TurnOff => &turn_off
        }
    }
}

mod part2 {
    use super::*;

    fn turn_on(l: &mut [u8]) {
        for i in 0..l.len() {
            l[i] += 1;
        }
    }

    fn toggle(l: &mut [u8]) {
        for i in 0..l.len() {
            l[i] += 2;
        }
    }

    fn turn_off(l: &mut [u8]) {
        for i in 0..l.len() {
            if l[i] > 0 {
                l[i] -= 1;
            }
        }
    }

    pub fn op_fn(op: &Operation) -> &OperationFn {
        use Operation::*;
        match op {
            TurnOn => &turn_on,
            Toggle => &toggle,
            TurnOff => &turn_off
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut grid = Grid::new();
    input.iter().for_each(|ins| grid.apply(ins, part1::op_fn(&ins.op)));
    grid.n_lit()
}

fn part2(input: &Input) -> usize {
    let mut grid = Grid::new();
    input.iter().for_each(|ins| grid.apply(ins, part2::op_fn(&ins.op)));
    grid.brightness()
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
    Ok(f.lines().map(|l| l.unwrap().parse::<Instruction>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "turn on 0,0 through 999,999
        toggle 0,0 through 999,0
        turn off 499,499 through 500,500";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<Instruction>().unwrap()).collect()
    }

    #[test]
    fn test_parse_instruction() {
        use Operation::*;
        assert_eq!(
            "turn on 0,0 through 999,999".parse::<Instruction>().unwrap(),
            Instruction { op: TurnOn, s_x: 0, s_y: 0, e_x: 999, e_y: 999 }
        );
        assert_eq!(
            "toggle 0,0 through 999,0".parse::<Instruction>().unwrap(),
            Instruction { op: Toggle, s_x: 0, s_y: 0, e_x: 999, e_y: 0 }
        );
        assert_eq!(
            "turn off 499,499 through 500,500".parse::<Instruction>().unwrap(),
            Instruction { op: TurnOff, s_x: 499, s_y: 499, e_x: 500, e_y: 500 }
        );
        assert_eq!(
            "turn off 1,2 through 3,4".parse::<Instruction>().unwrap(),
            Instruction { op: TurnOff, s_x: 1, s_y: 2, e_x: 3, e_y: 4 }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1000 * 1000 - 1000 - 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 1000 * 1000 + 1000 * 2 - 4);
    }
}