extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::cmp;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Ingredient>;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl FromStr for Ingredient {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+): .+ (.+), .+ (.+), .+ (.+), .+ (.+), .+ (.+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Ingredient {
            name: caps.get(1).unwrap().as_str().into(),
            capacity: get(2),
            durability: get(3),
            flavor: get(4),
            texture: get(5),
            calories: get(6)
        })
    }
}

fn score_mix(input: &Input, distribution: &Vec<usize>) -> i32 {
    let capacity = distribution.iter().enumerate().map(|(i, &d)| input[i].capacity * d as i32).sum::<i32>();
    let durability = distribution.iter().enumerate().map(|(i, &d)| input[i].durability * d as i32).sum::<i32>();
    let flavor = distribution.iter().enumerate().map(|(i, &d)| input[i].flavor * d as i32).sum::<i32>();
    let texture = distribution.iter().enumerate().map(|(i, &d)| input[i].texture * d as i32).sum::<i32>();

    cmp::max(0, capacity) * cmp::max(0, durability) * cmp::max(0, flavor) * cmp::max(0, texture)
}

fn calories_mix(input: &Input, distribution: &Vec<usize>) -> i32 {
    distribution.iter().enumerate().map(|(i, &d)| input[i].calories * d as i32 ).sum::<i32>()
}

fn find_best_mix(input: &Input, distribution: &Vec<usize>, to_distribute: usize, calories: i32) -> (i32, i32) {

    if distribution.len() == input.len() - 1 {
        let mut distribution = distribution.clone();
        distribution.push(to_distribute);
        let score = score_mix(input, &distribution);
        (score, if calories_mix(input, &distribution) == calories { score } else { 0 })
    } else {
        let mut distribution = distribution.clone();
        let didx = distribution.len();
        distribution.push(0);
        let mut best_s1 = 0;
        let mut best_s2 = 0;

        for i in 0..=to_distribute {
            distribution[didx] = i;
            let (s1, s2) = find_best_mix(input, &distribution, to_distribute - i, calories);
            if s1 > best_s1 {
                best_s1 = s1;
            }
            if s2 > best_s2 {
                best_s2 = s2;
            }
        }
        (best_s1, best_s2)
    }
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = find_best_mix(&input, &vec![], 100, 500);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<Ingredient>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<Ingredient>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        let (part1, _) = find_best_mix(&as_input(INPUT), &vec![], 100, 500);
        assert_eq!(part1, 62842880);
    }

    #[test]
    fn test_part2() {
        let (_, part2) = find_best_mix(&as_input(INPUT), &vec![], 100, 500);
        assert_eq!(part2, 57600000);
    }
}