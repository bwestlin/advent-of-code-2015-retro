extern crate utils;

use std::env;
use std::iter::FromIterator;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<i32>;

fn solve(input: &Input, expect_liters: i32) -> (usize, usize) {
    let mut found_containers = HashSet::new();
    let mut queue = VecDeque::from_iter((0..input.len()).map(|i| (Vec::from_iter(i..=i), input[i])));

    // Find how many different combinations of containers can exactly the expected liters
    while let Some((containers, sum)) = queue.pop_front() {
        for i in (containers[containers.len() - 1] + 1)..input.len() {
            let n_sum = sum + input[i];
            if n_sum <= expect_liters {
                let mut next_containers = containers.clone();
                next_containers.push(i);

                if n_sum == expect_liters {
                    found_containers.insert(next_containers);
                } else {
                    queue.push_front((next_containers, n_sum));
                }
            }
        }
    }

    // Find how many ways the least nr of containers can be filled
    let (_, n_least) = found_containers.iter()
        .fold((std::usize::MAX, 0), |(min, n), containers| {
            let c_len = containers.len();
            if c_len == min {
                (min, n + 1)
            } else if c_len < min {
                (c_len, 1)
            } else {
                (min, n)
            }
        });

    (found_containers.len(), n_least)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input, 150);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "20
        15
        10
        5
        5";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<i32>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT), 25).0, 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT), 25).1, 3);
    }
}