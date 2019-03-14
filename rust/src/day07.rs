extern crate utils;

use std::env;
use std::error;
use std::fmt;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Part>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Signal {
    Wire(String),
    Value(u16)
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Gate {
    And(Signal, Signal),
    Or(Signal, Signal),
    Lshift(Signal, Signal),
    Rshift(Signal, Signal),
    Not(Signal),
    Forward(Signal)
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Part {
    gate: Gate,
    out: Signal
}

#[derive(Debug, Clone)]
pub struct PartParseError { s: String }
impl fmt::Display for PartParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse part {}", self.s)
    }
}
impl error::Error for PartParseError {
    fn description(&self) -> &str {
        "unable to parse part"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for Part {
    type Err = PartParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("->").map(|v| v.trim()).collect();

        let signal = |i: &str| match i.parse::<u16>() {
            Ok(i) => Signal::Value(i),
            Err(_) => Signal::Wire(i.into())
        };

        let g = match &(v[0].split(' ').collect::<Vec<_>>())[..] {
            [a, "AND", b]    => Some(Gate::And(signal(a), signal(b))),
            [a, "OR", b]     => Some(Gate::Or(signal(a), signal(b))),
            [a, "LSHIFT", b] => Some(Gate::Lshift(signal(a), signal(b))),
            [a, "RSHIFT", b] => Some(Gate::Rshift(signal(a), signal(b))),
            ["NOT", a]       => Some(Gate::Not(signal(a))),
            [a]              => Some(Gate::Forward(signal(a))),
            _ => None
        };

        let out  = v[1];

        match g {
            Some(gate) => Ok(Part { gate: gate, out: Signal::Wire(out.into()) }),
            _ => Err(PartParseError { s: s.into() })
        }
    }
}

fn resolve_wire(input: &Input, wire: &str, preset_wires: &BTreeMap<String, u16>) -> u16 {
    let mut wires: BTreeMap<String, u16> = preset_wires.clone();
    let wires = &mut wires;

    let mut parts: VecDeque<_> = input.iter().collect();

    while let Some(part) = parts.pop_front() {

        let mut value;
        {
            let resolved = |s: &[&Signal]| s.iter().all(|s| match s {
                Signal::Wire(w) => wires.contains_key(w),
                _ => true
            });
            let resolve = |s: &Signal| match s {
                Signal::Wire(w) => *wires.get(w).unwrap(),
                Signal::Value(v) => *v,
            };

            value = match &part.gate {
                Gate::And(ref a, ref b)     if resolved(&[a, b]) => Some(resolve(a) & resolve(b)),
                Gate::Or(ref a, ref b)      if resolved(&[a, b]) => Some(resolve(a) | resolve(b)),
                Gate::Lshift(ref a, ref b)  if resolved(&[a, b]) => Some(resolve(a) << resolve(b)),
                Gate::Rshift(ref a, ref b)  if resolved(&[a, b]) => Some(resolve(a) >> resolve(b)),
                Gate::Not(ref a)            if resolved(&[a]) => Some(!resolve(a)),
                Gate::Forward(ref a)        if resolved(&[a]) => {
                    Some(match &part.out {
                        Signal::Wire(w) => preset_wires.get(&w.to_owned()).map(|v| *v).unwrap_or_else(|| resolve(a)),
                        _ => resolve(a)
                    })
                },
                _ => None
            };
        }

        if let Some(value) = value {
            if let Signal::Wire(ref w) = part.out {
                wires.insert(w.to_owned(), value);
            };
        } else {
            parts.push_back(part);
        }
    }

    *wires.get(wire).unwrap_or(&0)
}


fn part1(input: &Input) -> u16 {
    resolve_wire(input, "a", &BTreeMap::new())
}

fn part2(input: &Input, part1: u16) -> u16 {
    let mut preset: BTreeMap<String, u16> = BTreeMap::new();
    preset.insert("b".into(), part1);
    resolve_wire(input, "a", &preset)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let part1 = part1(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2(&input, part1));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<Part>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<Part>().unwrap()).collect()
    }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&as_input(INPUT)), 1337);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&as_input(INPUT)), 1337);
    // }
}