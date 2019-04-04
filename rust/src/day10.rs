extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Digits = Vec<u8>;

fn look_and_say(digits: &Digits) -> Digits {
    let mut out = vec![];
    out.reserve(digits.len());

    let mut last_digit = digits[0];
    let mut last_cnt = 1;

    for &i in &digits[1..] {
        if i != last_digit {
            out.push(last_cnt);
            out.push(last_digit);
            last_digit = i;
            last_cnt = 0;
        }
        last_cnt += 1;
    }
    out.push(last_cnt);
    out.push(last_digit);

    out
}

fn part_1_2(digits: &Digits) -> (usize, usize) {
    let mut last = digits.to_owned();
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 1..=50 {
        let next = look_and_say(&last);

        if i == 40 {
            p1 = next.len();
        } else if i == 50 {
            p2 = next.len();
        }
        last = next;
    }

    (p1, p2)
}

fn main() {
    measure(|| {
        let input = input().expect("Digits failed");
        let (part1, part2) = part_1_2(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Digits> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap().map(|line| line.as_str().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Digits>()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Digits {
        s.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(&as_input("1")), as_input("11"));
        assert_eq!(look_and_say(&as_input("11")), as_input("21"));
        assert_eq!(look_and_say(&as_input("21")), as_input("1211"));
        assert_eq!(look_and_say(&as_input("1211")), as_input("111221"));
        assert_eq!(look_and_say(&as_input("111221")), as_input("312211"));
    }
}