extern crate utils;

use std::env;
use std::str::FromStr;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

const PW_CHARS: [char; 23] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

#[derive(PartialEq, Debug)]
struct Password {
    pw: [u8; 8]
}

impl Password {
    fn incr(&mut self) {
        let l_idx = self.pw.len() - 1;
        for i in 0..=l_idx {
            if self.pw[l_idx - i] == (PW_CHARS.len() - 1) as u8 {
                self.pw[l_idx - i] = 0;
            } else {
                self.pw[l_idx - i] += 1;
                break;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut last_cidx = None;
        let mut num_straight = 0;
        let mut has_straight = false;
        let mut num_pair = 0;
        let mut last_was_pair = false;

        for &cidx in self.pw.iter() {
            if let Some(last_cidx) = last_cidx {
                let diff = cidx as i32 - last_cidx as i32;
                if diff == 1 && last_cidx != 6 && last_cidx != 8 && last_cidx != 10 {
                    num_straight += 1;
                    if num_straight >= 2 {
                        has_straight = true;
                    }
                } else {
                    num_straight = 0;
                }

                if diff == 0 && !last_was_pair {
                    num_pair += 1;
                    last_was_pair = true;
                } else {
                    last_was_pair = false;
                }
            }
            last_cidx = Some(cidx)
        }

        has_straight && num_pair >= 2
    }

    fn char2idx(chr: char) -> u8 {
        match PW_CHARS.iter().position(|&c| c == chr) {
            Some(idx) => idx as u8,
            _ => unreachable!()
        }
    }

    fn idx2char(idx: &u8) -> char {
        PW_CHARS[*idx as usize]
    }
}

impl FromStr for Password {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pw = [0; 8];
        for (i, c) in s.chars().enumerate() {
            pw[i] = Password::char2idx(c);
        }
        Ok(Password { pw })
    }
}

impl Into<String> for Password {
    fn into(self) -> String {
        self.pw.iter().map(Password::idx2char).collect()
    }
}

fn find_next_password(current: &String) -> String {
    let mut password: Password = current.parse().unwrap();
    loop {
        password.incr();
        if password.is_valid() {
            break;
        }
    }
    password.into()
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let part1 = find_next_password(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", find_next_password(&part1));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn as_password(s: &str) -> Password {
        s.parse().unwrap()
    }

    #[test]
    fn test_increment_password() {
        let mut pw = as_password("aaaaaaxx");
        pw.incr();
        assert_eq!(pw, as_password("aaaaaaxy"));
        pw.incr();
        assert_eq!(pw, as_password("aaaaaaxz"));
        pw.incr();
        assert_eq!(pw, as_password("aaaaaaya"));
        pw.incr();
        assert_eq!(pw, as_password("aaaaaayb"));
    }

    #[test]
    fn test_password_is_valid() {
        assert_eq!(as_password("abcdffaa").is_valid(), true, "abcdffaa should be valid");
        assert_eq!(as_password("ghjaabcc").is_valid(), true, "ghjaabcc should be valid");
        assert_eq!(as_password("hepxxyzz").is_valid(), true, "hepxxyzz should be valid");
        assert_eq!(as_password("heqaabcc").is_valid(), true, "heqaabcc should be valid");
    }

    #[test]
    fn test_find_next_password() {
        assert_eq!(find_next_password(&"abcdefgh".to_string()), "abcdffaa".to_string());
        assert_eq!(find_next_password(&"hepxcrrq".to_string()), "hepxxyzz".to_string());
        assert_eq!(find_next_password(&"hepxxyzz".to_string()), "heqaabcc".to_string());
    }
}