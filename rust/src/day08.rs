extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Vec<char>>;

fn in_memory_len(s: &Vec<char>) -> usize {
    let s = &s[1..(s.len() - 1)];

    let mut cnt = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == '\\' {
            if i < s.len() - 1 {
                if s[i + 1] == 'x' {
                    i += 2;
                }
                i += 1;
            }
        }
        cnt += 1;
        i += 1;
    }
    cnt
}

fn part1(input: &Input) -> usize {
    let (code_len, memory_len) = input.iter()
        .fold((0, 0), |(cl, ml), s| {
            (cl + s.len(), ml + in_memory_len(s))
        });

    code_len - memory_len
}

fn encoded_len(s: &Vec<char>) -> usize {
    s.iter().fold(0, |l, c| l + if *c == '\\' || *c == '"' { 2 } else { 1 }) + 2
}

fn part2(input: &Input) -> usize {
    let (code_len, encoded_len) = input.iter()
        .fold((0, 0), |(cl, el), s| {
            (cl + s.len(), el + encoded_len(s))
        });

    encoded_len - code_len
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
    Ok(f.lines().map(|l| l.unwrap().chars().collect()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       r#"
          ""
          "abc"
          "aaa\"aaa"
          "\x27"
        "#;

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().into()).filter(|s: &String| s.len() > 0).map(|s| s.chars().collect()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 19);
    }
}