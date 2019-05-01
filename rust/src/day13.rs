extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;


#[derive(Debug)]
struct Input {
    persons: Vec<String>,
    happiness_change_next_to: Vec<Vec<i32>>
}

impl Input {
    fn parse(lines: impl IntoIterator<Item = String>) -> Input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) .* (gain|lose) (\d+) .* (.+)\.$").unwrap();
        }

        let mut persons: Vec<String> = vec![];
        let mut happiness_change_next_to: Vec<Vec<(usize, i32)>> = vec![];
        {
            let mut person_idx = |person: String| {
                match persons.iter().position(|p| *p == person) {
                    Some(idx) => idx,
                    None => {
                        persons.push(person);
                        persons.len() - 1
                    }
                }
            };

            for l in lines {
                let caps = RE.captures(&l[..]).unwrap();
                let pidx = person_idx(caps.get(1).unwrap().as_str().into());
                let direction = match caps.get(2).unwrap().as_str() {
                    "gain" => 1,
                    "lose" => -1,
                    _ => unreachable!()
                };
                let amount = direction * caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let pnidx = person_idx(caps.get(4).unwrap().as_str().into());

                if happiness_change_next_to.len() <= pidx {
                    happiness_change_next_to.resize_with(pidx + 1, Default::default)
                }
                happiness_change_next_to[pidx].push((pnidx, amount));
            }
        }

        let n_persons = persons.len();
        Input {
            persons,
            happiness_change_next_to: happiness_change_next_to.iter()
                .map(|changes| {
                    let mut change_by_idx = vec![0i32; n_persons];
                    for &(pnidx, change) in changes.iter() {
                        change_by_idx[pnidx] = change;
                    }
                    change_by_idx
                }).collect()
        }
    }

    fn add_yourself(&mut self) {
        self.persons.push("you".into());
        for i in 0..self.happiness_change_next_to.len() {
            self.happiness_change_next_to[i].push(0);
        }
        self.happiness_change_next_to.push(vec![0; self.persons.len()]);
    }
}

#[derive(PartialEq, Debug)]
struct Seating {
    pidx: Vec<usize>,
    dirs: Vec<i8>,
    first: bool
}

impl Seating {
    fn new(n_seats: usize) -> Seating {
        Seating { pidx: (0..n_seats).collect(), dirs: vec![-1; n_seats], first: true }
    }
}

// Iterator of pssible seating permutations based on the Steinhaus–Johnson–Trotter algorithm
impl Iterator for Seating {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.pidx.clone());
        }
        let len = self.pidx.len();

        // Find the largest movable value, note that first value is non movable in circular seating
        let mut largest_movable = None;
        for i in 1..len {
            let n = self.pidx[i];
            let dir = self.dirs[n];
            // Check that we're not at the boundary
            if (i == 1 && dir == -1) || (i == len - 1 && dir == 1) {
                continue;
            }
            // Check that value in direction is smaller
            if self.pidx[(i as i8 + dir) as usize] > n {
                continue;
            }
            if let Some(lm) = largest_movable {
                if n > self.pidx[lm] {
                    largest_movable = Some(i)
                }
            } else {
                largest_movable = Some(i)
            }
        }

        if let Some(lm) = largest_movable {
            // Swap direction of all values larger the largest movable
            for i in 1..len {
                if self.pidx[i] > self.pidx[lm] {
                    self.dirs[self.pidx[i]] *= -1;
                }
            }

            // Swap with element in the direction
            let dir = self.dirs[self.pidx[lm]];
            let tmp = self.pidx[(lm as i8 + dir) as usize];
            self.pidx[(lm as i8 + dir) as usize] = self.pidx[lm];
            self.pidx[lm] = tmp;

            Some(self.pidx.clone())
        } else {
            // If no largest movable is found we're done
            None
        }
    }
}

fn max_happiness_change(input: &Input) -> i32 {
    let seating = Seating::new(input.persons.len());
    let mut max_happiness_change = 0;

    for s in seating {
        let mut happiness_change = 0;

        for i in 0..s.len() {
            let pidx = s[i];
            let adjacent = [
                if i == 0 { s[s.len() - 1] } else { s[i - 1] },
                if i == s.len() - 1 { s[0] } else { s[i + 1] }
            ];
            happiness_change += adjacent.iter().map(|a| input.happiness_change_next_to[pidx][*a]).sum::<i32>();
        }

        if happiness_change > max_happiness_change {
            max_happiness_change = happiness_change;
        }
    }

    max_happiness_change
}

fn part1(input: &Input) -> i32 {
    max_happiness_change(input)
}

fn part2(input: &mut Input) -> i32 {
    input.add_yourself();
    max_happiness_change(input)
}

fn main() {
    measure(|| {
        let mut input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&mut input));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(Input::parse(f.lines().map(|l| l.unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const INPUT: &'static str =
       "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.";

    fn as_input(s: &str) -> Input {
        Input::parse(s.split('\n').map(|s| s.trim().into()))
    }

    #[test]
    fn test_seating() {
        let seatings: Vec<_> = Seating::new(4).collect::<BTreeSet<_>>().into_iter().collect();
        assert_eq!(seatings, vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 3, 2],
            vec![0, 2, 1, 3],
            vec![0, 2, 3, 1],
            vec![0, 3, 1, 2],
            vec![0, 3, 2, 1]
        ]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 330);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut as_input(INPUT)), 286);
    }
}