extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;
extern crate rayon;

use rayon::prelude::*;
use std::u32;
use std::cmp;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

#[derive(Debug)]
struct Distance {
    loc_idxs: [usize; 2],
    dist: u32
}

#[derive(Debug)]
struct Input {
    locations: Vec<String>,
    distances: Vec<Distance>
}

impl Input {
    fn parse(iter: impl IntoIterator<Item = String>) -> Input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.*) to (.*) = (\d*)$").unwrap();
        }

        let mut locations: Vec<String> = vec![];
        let mut distances: Vec<Distance> = vec![];
        {
            let mut loc_idx = |name: String| {
                match locations.iter().position(|l| *l == name) {
                    Some(idx) => idx,
                    None => {
                        locations.push(name);
                        locations.len() - 1
                    }
                }
            };

            for i in iter {
                let caps = RE.captures(&i[..]).unwrap();
                let l1 = loc_idx(caps.get(1).unwrap().as_str().into());
                let l2 = loc_idx(caps.get(2).unwrap().as_str().into());
                distances.push(Distance {
                    loc_idxs: [l1, l2],
                    dist: caps.get(3).unwrap().as_str().parse::<u32>().unwrap()
                });
            }
        }

        Input { locations: locations, distances: distances }
    }
}

#[derive(Debug)]
struct Route {
    visited_loc_idxs: Vec<usize>,
    dist: u32
}

fn find_min_max_routes(input: &Input) -> (u32, u32) {
    // Build lookup for distances from one location to possible others
    let loc_dists = input.distances.iter()
        .fold(vec![HashMap::new(); input.locations.len()], |mut r: Vec<HashMap<usize, u32>>, dist| {
            r[dist.loc_idxs[0]].insert(dist.loc_idxs[1], dist.dist);
            r[dist.loc_idxs[1]].insert(dist.loc_idxs[0], dist.dist);
            r
        });

    // Run all possible routes from every location in parallel
    (0..input.locations.len()).into_par_iter()
        .map(|loc_idx| {
            let mut min_dist = u32::MAX;
            let mut max_dist = u32::MIN;
            let loc_dists_local = loc_dists.clone();
            let n_locations = input.locations.len();
            let mut route_q: VecDeque<Route> = VecDeque::new();
            route_q.push_back(Route { visited_loc_idxs: vec![loc_idx], dist: 0 });

            while let Some(route) = route_q.pop_front() {
                let last_visited_idx = route.visited_loc_idxs.last().unwrap();

                // Go through all possible reachable locations that have not been visited
                for (loc_idx, dist) in loc_dists_local[*last_visited_idx].iter() {
                    if !route.visited_loc_idxs.contains(loc_idx) {

                        // Check if all locations is visited
                        if route.visited_loc_idxs.len() == n_locations - 1 {
                            let tot_dist = route.dist + dist;
                            min_dist = cmp::min(min_dist, tot_dist);
                            max_dist = cmp::max(max_dist, tot_dist);
                        }
                        // Otherwise put on the queue
                        else {
                            let mut next_visited_loc_idxs = route.visited_loc_idxs.clone();
                            next_visited_loc_idxs.push(*loc_idx);
                            let next_route = Route {
                                visited_loc_idxs: next_visited_loc_idxs,
                                dist: route.dist + dist
                            };

                            // Do depth first by enqueing in the beginning of queue
                            route_q.push_front(next_route);
                        }
                    }
                }
            }
            (min_dist, max_dist)
        })
        .reduce(|| (u32::MAX, u32::MIN), |(min, max), (lmin, lmax)| {
            (cmp::min(min, lmin), cmp::max(max, lmax))
        })
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = find_min_max_routes(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
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

    const INPUT: &'static str =
       "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

    fn as_input(s: &str) -> Input {
        Input::parse(s.split('\n').map(|s| s.trim().into()))
    }

    #[test]
    fn test_part1() {
        assert_eq!(find_min_max_routes(&as_input(INPUT)).0, 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_min_max_routes(&as_input(INPUT)).1, 982);
    }
}