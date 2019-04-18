extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Result {
    part1: i32,
    part2: i32
}

#[derive(PartialEq, Eq)]
pub enum HType {
    Obj,
    Arr,
}

fn solve(json: &str) -> Result {
    let mut p1_sums: Vec<Vec<i32>> = vec![vec![]];
    let mut p2_sums: Vec<Vec<i32>> = vec![vec![]];
    let mut htypes: Vec<HType> = vec![];
    let mut has_red: Vec<bool> = vec![];

    let chars = &mut json.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                p1_sums.push(vec![]);
                p2_sums.push(vec![]);
                htypes.push(HType::Obj);
                has_red.push(false);
            },
            '}' => {
                let p1_sum = p1_sums.pop().unwrap().iter().sum();
                let p2_sum = p2_sums.pop().unwrap().iter().sum();
                htypes.pop();

                (*p1_sums.last_mut().unwrap()).push(p1_sum);
                if let Some(red) = has_red.pop() {
                    if !red {
                        (*p2_sums.last_mut().unwrap()).push(p2_sum);
                    }
                }
            },
            '[' => {
                p1_sums.push(vec![]);
                p2_sums.push(vec![]);
                htypes.push(HType::Arr);
                has_red.push(false);
            },
            ']' => {
                let p1_sum = p1_sums.pop().unwrap().iter().sum();
                let p2_sum = p2_sums.pop().unwrap().iter().sum();
                htypes.pop();
                has_red.pop();
                (*p1_sums.last_mut().unwrap()).push(p1_sum);
                (*p2_sums.last_mut().unwrap()).push(p2_sum);
            },
            '"' => {
                let quoted: String = chars.take_while(|&c| c != '"').collect();

                if htypes.last() == Some(&HType::Obj) {
                    if &quoted[..] == "red" {
                        *has_red.last_mut().unwrap() = true;
                    }
                }
            },
            ',' | ':' => {},
            _ => {
                let mut nchrs = vec![c];
                while let Some(&c) = chars.peek() {
                    if c < '0' || c > '9' {
                        break;
                    }
                    nchrs.push(chars.next().unwrap());
                }
                let num: i32 = nchrs.iter().collect::<String>().parse().unwrap();
                (*p1_sums.last_mut().unwrap()).push(num);
                (*p2_sums.last_mut().unwrap()).push(num);
            }
        }
    }

    Result { part1: p1_sums[0].iter().sum(), part2: p2_sums[0].iter().sum() }
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let parts = solve(&input);
        println!("Part1: {}", parts.part1);
        println!("Part2: {}", parts.part2);
    });
}

fn input() -> io::Result<String> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve(r#"[1,2,3]"#).part1, 6);
        assert_eq!(solve(r#"{"a":2,"b":4}"#).part1, 6);
        assert_eq!(solve(r#"[[[3]]]"#).part1, 3);
        assert_eq!(solve(r#"{"a":{"b":4},"c":-1}"#).part1, 3);
        assert_eq!(solve(r#"{"a":[-1,1]}"#).part1, 0);
        assert_eq!(solve(r#"[-1,{"a":1}]"#).part1, 0);
        assert_eq!(solve(r#"[]"#).part1, 0);
        assert_eq!(solve(r#"{}"#).part1, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(r#"[1,2,3]"#).part2, 6);
        assert_eq!(solve(r#"[1,{"c":"red","b":2},3]"#).part2, 4);
        assert_eq!(solve(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).part2, 0);
        assert_eq!(solve(r#"[1,"red",5]"#).part2, 6);
    }

    fn input() -> io::Result<String> {
        let f = File::open("../input/day12")?;
        let f = BufReader::new(f);
        Ok(f.lines().map(|l| l.unwrap()).collect())
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(&input().unwrap()), Result { part1: 191164, part2: 87842 });
    }
}