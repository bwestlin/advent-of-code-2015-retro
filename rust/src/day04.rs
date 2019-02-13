extern crate utils;
extern crate md5;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

fn find_advent_coin(input: &Input, leading_zeroes: usize) -> i32 {
    for i in 1.. {
        let digest = md5::compute(format!("{}{}", input, i));
        for j in 0..leading_zeroes {
            if ((0x0F << ((j + 1) % 2) * 4) & digest[j >> 1]) > 0 {
                break;
            }
            if j == leading_zeroes - 1 {
                return i;
            }
        }
    }
    0
}

fn part1(input: &Input) -> i32 {
    find_advent_coin(input, 5)
}

fn part2(input: &Input) -> i32 {
    find_advent_coin(input, 6)

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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&"abcdef".into()), 609043);
        assert_eq!(part1(&"pqrstuv".into()), 1048970);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&"abcdef".into()), 6742839);
        assert_eq!(part2(&"pqrstuv".into()), 5714438);
    }
}