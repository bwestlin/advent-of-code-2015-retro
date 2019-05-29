extern crate utils;

use std::env;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Sue>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Component {
    Children, Cats, Samoyeds, Pomeranians, Akitas, Vizslas, Goldfish, Trees, Cars, Perfumes
}

impl Component {
    fn resolve(s: &str) -> Component {
        use Component::*;
        match s {
            "children"    => Children,
            "cats"        => Cats,
            "samoyeds"    => Samoyeds,
            "pomeranians" => Pomeranians,
            "akitas"      => Akitas,
            "vizslas"     => Vizslas,
            "goldfish"    => Goldfish,
            "trees"       => Trees,
            "cars"        => Cars,
            "perfumes" => Perfumes,
            _ => unreachable!()
        }
    }
}

// TODO Simplify
impl From<usize> for Component {
    fn from(idx: usize) -> Self {
        use Component::*;
        match idx {
            0 => Children,
            1 => Cats,
            2 => Samoyeds,
            3 => Pomeranians,
            4 => Akitas,
            5 => Vizslas,
            6 => Goldfish,
            7 => Trees,
            8 => Cars,
            9 => Perfumes,
            _ => unreachable!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Components( [Option<i32>; 10] );

impl Components {
    fn new() -> Components {
        Components( [None; 10] )
    }
}

impl std::ops::Index<Component> for Components  {
    type Output = Option<i32>;
    fn index(&self, c: Component) -> &Self::Output {
        &self.0[c as usize]
    }
}

impl std::ops::IndexMut<Component> for Components {
    fn index_mut<'a>(&'a mut self, c: Component) -> &'a mut Self::Output {
        &mut self.0[c as usize]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Sue {
    nr: i32,
    components: Components
}

impl Sue {
    fn alike_fns(&self, other: &Sue, match_fns: &Vec<&ComponentMatchFn>) -> Vec<i32> {
        let mut matches = vec![0; match_fns.len()];

        for i in 0..self.components.0.len() {
            let c: Component = i.into();

            for i2 in 0..match_fns.len() {
                if match_fns[i2](c, self.components.0[i], other.components.0[i]) {
                    matches[i2] += 1;
                }
            }
        }

        matches
    }
}

impl FromStr for Sue {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, c) = s.split_at(s.find(':').unwrap());
        let c = &c[2..];
        let mut components = Components::new();
        for p in c.split(',') {
            let (c, v) = p.split_at(p.find(':').unwrap());
            let c = Component::resolve(c.trim());
            let v = v[1..].trim().parse::<i32>()?;
            components[c] = Some(v);
        }
        Ok(Sue {
            nr: n[4..].parse()?,
            components: components
        })
    }
}

fn real_sue() -> Sue {
    "Sue 0:
     children: 3,
     cats: 7,
     samoyeds: 2,
     pomeranians: 3,
     akitas: 0,
     vizslas: 0,
     goldfish: 5,
     trees: 3,
     cars: 2,
     perfumes: 1".parse::<Sue>().unwrap()
}

type ComponentMatchFn = Fn(Component, Option<i32>, Option<i32>) -> bool;

fn match_part1(_c: Component, a: Option<i32>, b: Option<i32>) -> bool {
    a == b
}

fn match_part2(c: Component, a: Option<i32>, b: Option<i32>) -> bool {
    use Component::*;
    match (a, b) {
        (Some(a), Some(b)) if c == Cats || c == Trees => a > b,
        (Some(a), Some(b)) if c == Pomeranians || c == Goldfish => a < b,
        (Some(a), Some(b)) => a == b,
        _ => false
    }
}

fn find_sue(input: &Input) -> (i32, i32) {
    let real_sue = real_sue();
    let match_fns: Vec<&ComponentMatchFn> = vec![&match_part1, &match_part2];

    let ((p1, _), (p2, _)) = input.iter()
        .fold(((0, 0), (0, 0)), |((mnr1, mal1), (mnr2, mal2)), sue| {
            let alike = sue.alike_fns(&real_sue, &match_fns);

            (
                if alike[0] > mal1 { (sue.nr, alike[0]) } else { (mnr1, mal1) },
                if alike[1] > mal2 { (sue.nr, alike[1]) } else { (mnr2, mal2) }
            )
        });

    (p1, p2)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = find_sue(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap().parse::<Sue>().unwrap()).collect())
}
