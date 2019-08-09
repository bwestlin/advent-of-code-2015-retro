extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::cmp;
use std::iter::FromIterator;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Raindeer>;

#[derive(PartialEq, Eq, Debug)]
struct Raindeer {
    name: String,
    fly_speed: i32,
    fly_time: i32,
    rest_time: i32
}

impl FromStr for Raindeer {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\S+) .* (\d+) .* (\d+) .* (\d+) .*$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let get = |idx| caps.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        Ok(Raindeer {
            name: caps.get(1).unwrap().as_str().into(),
            fly_speed: get(2),
            fly_time: get(3),
            rest_time: get(4)
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
enum RaindeerStatus {
    Flying, Resting
}

#[derive(PartialEq, Eq, Debug)]
struct RaindeerState<'a> {
    raindeer: &'a Raindeer,
    status: RaindeerStatus,
    distance: i32,
    next_status_countdown: i32,
    points: i32
}

impl<'a> RaindeerState<'a> {

    fn new(raindeer: &Raindeer) -> RaindeerState {
        RaindeerState {
            raindeer: raindeer,
            status: RaindeerStatus::Flying,
            distance: 0,
            next_status_countdown: raindeer.fly_time,
            points: 0
        }
    }

    fn step(&mut self) {
        use RaindeerStatus::*;
        let Raindeer { fly_time, fly_speed, rest_time, .. } = self.raindeer;

        self.next_status_countdown -= 1;
        if self.status == Flying {
            self.distance += fly_speed;
        }

        if self.next_status_countdown == 0 {
            self.status = match self.status {
                Flying => Resting,
                _ => Flying
            };

            self.next_status_countdown = match self.status {
                Flying => *fly_time,
                _ => *rest_time
            };
        }
    }

    fn award(&mut self) {
        self.points += 1;
    }
}

fn solve(input: &Input, seconds: i32) -> (i32, i32) {
    let mut raindeer_states = Vec::from_iter(input.iter().map(RaindeerState::new));

    for _ in 0..seconds {
        let mut lead_distance = 0;

        for i in 0..raindeer_states.len() {
            raindeer_states[i].step();
            if raindeer_states[i].distance > lead_distance {
                lead_distance = raindeer_states[i].distance;
            }
        }

        for i in 0..raindeer_states.len() {
            if raindeer_states[i].distance == lead_distance {
                raindeer_states[i].award();
            }
        }
    }

    raindeer_states.iter().fold((0, 0), |(distance, points), rs| {
        (cmp::max(distance, rs.distance), cmp::max(points, rs.points))
    })
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input, 2503);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<Raindeer>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<Raindeer>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT), 1000).0, 1120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT), 1000).1, 689);
    }
}