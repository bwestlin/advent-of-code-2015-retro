extern crate utils;
extern crate rayon;

use std::env;
use std::cmp;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use rayon::prelude::*;
use utils::*;

type Input = usize;

fn part1(input: Input) -> usize {
    let mut presents_delivered = vec![0_u32; input / 10 + 1];
    let input = input as u32;

    for elf in 1..presents_delivered.len() {
        let n_presents = elf as u32 * 10;

        for house_nr in (elf..presents_delivered.len()).step_by(elf) {
            presents_delivered[house_nr] += n_presents;
        }
    }

    presents_delivered.iter().enumerate()
        .find(|(_, &p)| p >= input).map(|(house_nr, _)| house_nr).unwrap_or(0)
}

fn part2(input: Input) -> usize {
    let mut presents_delivered = vec![0_u32; input / 10 + 1];
    let input = input as u32;

    for elf in 1..presents_delivered.len() {
        let n_presents = elf as u32 * 11;

        let last_house_nr = cmp::min(elf + elf * 50, presents_delivered.len() - 1);
        for house_nr in (elf..=last_house_nr).step_by(elf) {
            presents_delivered[house_nr] += n_presents;
        }
    }

    presents_delivered.iter().enumerate()
        .find(|(_, &p)| p >= input).map(|(house_nr, _)| house_nr).unwrap_or(0)
}

fn main() {
    measure_times(100, || {
        let input = input().expect("Input failed");

        // Run part 1&2 in paralell
        let parts: Vec<_> = [part1, part2].par_iter()
            .map(|f| f(input))
            .collect();

        println!("Part1: {}", parts[0]);
        println!("Part2: {}", parts[1]);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(30), 2);
        assert_eq!(part1(35), 3);
        assert_eq!(part1(100), 6);
        assert_eq!(part1(150), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(30), 2);
        assert_eq!(part2(35), 3);
        assert_eq!(part2(100), 6);
        assert_eq!(part2(150), 8);
    }
}