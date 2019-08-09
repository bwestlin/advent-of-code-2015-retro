extern crate utils;

use std::env;
use std::cmp;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = LightGrid;

#[cfg(test)]
const GRID_SIZE: usize = 6;
#[cfg(not(test))]
const GRID_SIZE: usize = 100;

#[derive(Clone)]
struct LightGrid {
    lights: [[bool; GRID_SIZE]; GRID_SIZE]
}

impl LightGrid {
    fn read<R: Read>(reader: BufReader<R>) -> LightGrid {
        reader.lines().map(|l| l.unwrap().trim().to_string()).enumerate()
            .fold(LightGrid { lights: [[false; GRID_SIZE]; GRID_SIZE] }, |mut lg, (y, l)| {
                for (x, c) in l.chars().enumerate() {
                    lg.lights[y][x] = c == '#';
                }
                lg
            })
    }

    fn n_adjacent_on(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        let mut on_cnt = 0;
        for ay in cmp::max(0, y - 1)..cmp::min(GRID_SIZE as i32, y + 2) {
            for ax in cmp::max(0, x as i32 - 1)..cmp::min(GRID_SIZE as i32, x + 2) {
                if self.lights[ay as usize][ax as usize] && !(ax == x && ay == y) {
                    on_cnt += 1;
                }
            }
        }
        on_cnt
    }

    fn step(&mut self, last: &LightGrid) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let n_adj_on = last.n_adjacent_on(x, y);
                self.lights[y][x] = last.lights[y][x] && (n_adj_on == 2 || n_adj_on == 3)
                                    || !last.lights[y][x] && n_adj_on == 3;
            }
        }
    }

    fn n_on(&self) -> i32 {
        let mut on = 0;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if self.lights[y][x] {
                    on += 1;
                }
            }
        }
        on
    }

    fn turn_on_stuck(&mut self) {
        self.lights[0][0] = true;
        self.lights[GRID_SIZE - 1][0] = true;
        self.lights[0][GRID_SIZE - 1] = true;
        self.lights[GRID_SIZE - 1][GRID_SIZE - 1] = true;
    }

    #[cfg(feature = "print")]
    fn print(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                print!("{}", if self.lights[y][x] { '#' } else { '.' });
            }
            println!();
        }

    }
}

fn simulate(input: &Input, steps: usize, turn_on_stuck: bool) -> i32 {
    #[cfg(feature = "print")] {
        println!("\nInitial");
        input.print();
    }

    let mut grids: VecDeque<LightGrid> = VecDeque::with_capacity(2);
    grids.push_back((*input).clone());
    grids.push_back((*input).clone());

    for s in 0..steps {
        let mut last_grid = grids.pop_front().unwrap();
        let mut next_grid = grids.pop_front().unwrap();

        if turn_on_stuck && s == 0 {
            last_grid.turn_on_stuck();
        }
        next_grid.step(&last_grid);
        if turn_on_stuck {
            next_grid.turn_on_stuck();
        }

        #[cfg(feature = "print")] {
            println!();
            println!("Step: {}", s + 1);
            next_grid.print();
        }

        grids.push_back(next_grid);
        grids.push_back(last_grid);
    }

    let last_grid = grids.pop_front().unwrap();
    last_grid.n_on()
}

fn part1(input: &Input, steps: usize) -> i32 {
    simulate(input, steps, false)
}

fn part2(input: &Input, steps: usize) -> i32 {
    simulate(input, steps, true)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input, 100));
        println!("Part2: {}", part2(&input, 100));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(LightGrid::read(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       ".#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..";

    fn as_input(s: &str) -> Input {
        let f = BufReader::new(s.as_bytes());
        LightGrid::read(f)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT), 4), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT), 5), 17);
    }
}