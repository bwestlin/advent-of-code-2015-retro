#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<String>;

fn is_nice(s: &str) -> bool {
    lazy_static! {
        static ref vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    }

    let mut n_vovels = 0;
    let mut twice = false;
    let mut lchr = None;

    for c in s.chars() {
        if vowels.contains(&c) {
            n_vovels += 1;
        }

        if let Some(lchr) = lchr {
            if lchr == c {
                twice = true;
            }
            if (lchr == 'a' && c == 'b') || (lchr == 'c' && c == 'd') || (lchr == 'p' && c == 'q') || (lchr == 'x' && c == 'y') {
                return false;
            }
        }

        lchr = Some(c);
    }
    n_vovels >= 3 && twice
}

fn is_nice_better(s: &str) -> bool {
    let mut pairs = vec![];
    for i in 0..(s.len() - 1) {
        pairs.push(&s[i..=(i + 1)]);
    }

    let any_pair_appears_twice = pairs.iter().any(|pair| s.matches(pair).count() > 1);

    let mut repeat_one_between = false;
    let chars: Vec<_> = s.chars().collect();
    for i in 0..(chars.len() - 2) {
        if chars[i] == chars[i + 2] {
            repeat_one_between = true;
            break;
        }
    }

    any_pair_appears_twice && repeat_one_between
}

fn part1(input: &Input) -> usize {
    input.iter().filter(|s| is_nice(&s[..])).count()
}

fn part2(input: &Input) -> usize {
    input.iter().filter(|s| is_nice_better(&s[..])).count()
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

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().into()).collect()
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part1() {
        const INPUT: &'static str =
            "ugknbfddgicrmopn
             aaa
             jchzalrnumimnmhp
             haegwjzuvuyypxyu
             dvszwmarrgswjxmb";
        assert_eq!(part1(&as_input(INPUT)), 2);
    }

    #[test]
    fn test_is_nice_better() {
        assert_eq!(is_nice_better("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_better("xxyxx"), true);
        assert_eq!(is_nice_better("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_better("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn test_part2() {
        const INPUT: &'static str =
            "qjhvhtzxzqqjkmpb
             xxyxx
             uurcxstgmygtbstg
             ieodomkazucvgmuy";
        assert_eq!(part2(&as_input(INPUT)), 2);
    }
}