extern crate utils;
//extern crate rayon;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<u32>;

struct Combinations {
    values: Vec<u32>,
    indexes: Vec<usize>,
    n: usize,
    first: bool
}

impl Combinations {
    fn new(values: &Vec<u32>, n: usize) -> Combinations {
        Combinations { values: values.clone(), indexes: (0..n).collect(), n: n, first: true }
    }
}

impl Iterator for Combinations {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.indexes.iter().map(|i| self.values[*i]).collect::<Self::Item>());
        }

        let n_values = self.values.len();
        let n = self.n;
        {
            let v = &mut self.indexes;
            let l = n - 1;
            for i in 0..n {
                if v[l - i] == n_values - 1 - i {
                    if i == n - 1 {
                        return None;
                    }
                    v[l - i] = v[l - i - 1] + 2;
                    if i > 0 {
                        for j in (0..=(i - 1)).rev() {
                            v[l - j] = v[l - (j + 1)] + 1;
                        }
                    }
                } else {
                    v[l - i] += 1;
                    break;
                }
            }
        }

        // This is faster than: Some(self.indexes.iter().map(|i| self.values[*i]).collect::<Self::Item>())
        let mut next = Vec::with_capacity(n_values);
        for i in 0..n {
            next.push(self.values[self.indexes[i]]);
        }
        Some(next)
    }
}

fn quantum_entanglement(packages: Input) -> u64 {
    packages.iter().map(|p| *p as u64).product::<u64>()
}

fn ideal_quantum_entanglement(packages: &Input, n_groups: u32) -> u64 {
    let n_packages = packages.len();
    let per_group = packages.iter().sum::<u32>() / n_groups;
    let max_packages = n_packages - (n_groups as usize - 1);

    (1..=max_packages)
        .map(|n| {
            Combinations::new(packages, n)
                .filter(|packages| packages.iter().sum::<u32>() == per_group)
                .map(quantum_entanglement)
                .min()
        })
        .skip_while(|lowest| lowest.is_none())
        .next()
        .map(|lowest| lowest.unwrap())
        .unwrap_or(0)
}

fn part1(input: &Input) -> u64 {
    ideal_quantum_entanglement(input, 3)
}

fn part2(input: &Input) -> u64 {
    ideal_quantum_entanglement(input, 4)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));

        // rayon::join(|| println!("Part1: {}", part1(&input)),
        //             || println!("Part2: {}", part2(&input)));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "1
        2
        3
        4
        5
        7
        8
        9
        10
        11";

    fn as_input(s: &str) -> Input {
        s.split('\n').map(|s| s.trim().parse::<u32>().unwrap()).collect()
    }

    #[test]
    fn test_combinations() {
        assert_eq!(
            Combinations::new(&vec![1, 2, 3], 1).collect::<Vec<_>>(),
            vec![
                vec![1],
                vec![2],
                vec![3]
            ]
        );
        assert_eq!(
            Combinations::new(&vec![1, 2, 3], 2).collect::<Vec<_>>(),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 3]
            ]
        );
        assert_eq!(
            Combinations::new(&vec![1, 2, 3], 3).collect::<Vec<_>>(),
            vec!
                [vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 99);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 44);
    }
}