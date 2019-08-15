extern crate utils;

use std::env;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Sequence = Vec<char>;
type Replacement = (Sequence, Sequence);

#[derive(Debug)]
struct Input {
    replacements: Vec<Replacement>,
    molecule: Sequence
}

impl Input {
    fn parse<R: Read>(reader: BufReader<R>) -> Input {
        reader.lines().map(|l| l.unwrap().trim().to_string())
            .fold((Input { replacements: vec![], molecule: vec![] }, false), |(mut i, mut molecule_next), l| {
                if l.len() == 0 {
                    molecule_next = true;
                } else {
                    if molecule_next {
                        i.molecule = l.chars().collect();
                    } else {
                        let mut parts = l.split(" => ").map(|s| s.chars().collect::<Vec<char>>());
                        i.replacements.push((parts.next().unwrap(), parts.next().unwrap()));
                    }
                }
                (i, molecule_next)
            }).0
    }
}

fn replace(seq: &Sequence, replacement: &Replacement) -> Vec<Sequence> {
    let mut found = vec![];
    let (from, to) = replacement;
    if seq.len() >= from.len() {
        for i in 0..=(seq.len() - from.len()) {
            let s = &seq[i..];
            if s.starts_with(from) {
                let mut r: Sequence = seq[0..i].into();
                for &s in to {
                    r.push(s);
                }
                if s.len() > from.len() {
                    for &s in &s[from.len()..] {
                        r.push(s);
                    }
                }
                found.push(r);
            }
        }
    }
    found
}

fn part1(input: &Input) -> usize {
    let mut found = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(input.molecule.clone());

    for r in input.replacements.iter() {
        let repl = replace(&input.molecule, r);
        for r in repl {
            let rc = r.clone();
            if found.insert(r) {
                queue.push_back(rc);
            }
        }
    }

    found.len()
}

#[derive(Eq, Debug)]
struct QueueStep {
    molecule: Sequence,
    n_steps: usize
}

impl Ord for QueueStep {
    fn cmp(&self, other: &QueueStep) -> Ordering {
        other.molecule.len().cmp(&self.molecule.len())
            .then(other.n_steps.cmp(&self.n_steps))
    }
}

impl PartialOrd for QueueStep {
    fn partial_cmp(&self, other: &QueueStep) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueueStep {
    fn eq(&self, other: &QueueStep) -> bool {
        self.molecule == other.molecule && self.n_steps == other.n_steps
    }
}

fn part2(input: &Input) -> usize {
    let reverse_replacements: Vec<_> = input.replacements.iter().map(|(to, from)| (from.clone(), to.clone())).collect();
    let mut heap = BinaryHeap::new();
    heap.push(QueueStep { molecule: input.molecule.clone(), n_steps: 0 });
    let target = vec!['e'];

    while let Some(step) = heap.pop() {
        for r in reverse_replacements.iter() {
            let repl = replace(&step.molecule, r);
            for r in repl {
                if r == target {
                    return step.n_steps + 1;
                }
                heap.push(QueueStep { molecule: r, n_steps: step.n_steps + 1 });
            }
        }
    }
    0
}

fn main() {
    measure_times(100, || {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(Input::parse(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
       "H => HO
        H => OH
        O => HH
        
        ";

    const INPUT2: &'static str =
       "e => H
        e => O
        H => HO
        H => OH
        O => HH
        
        ";

    fn as_input(s: &str, m: &str) -> Input {
        let mut s2 = s.to_string();
        s2.push_str(m);
        let f = BufReader::new(s2.as_bytes());
        Input::parse(f)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT1, "HOH")), 4);
        assert_eq!(part1(&as_input(INPUT1, "HOHOHO")), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT2, "HOH")), 3);
        assert_eq!(part2(&as_input(INPUT2, "HOHOHO")), 6);
    }
}