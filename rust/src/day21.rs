extern crate utils;

use std::env;
use std::cmp;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Participant;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Participant {
    hp: i32,
    damage: i32,
    armor: i32
}

impl Participant {
    fn read<R: Read>(reader: BufReader<R>) -> Participant {
        let l: Vec<_> = reader.lines().map(|l| l.unwrap().split(":").skip(1).next().unwrap().trim().parse::<i32>().unwrap()).collect();
        Participant { hp: l[0], damage: l[1], armor: l[2] }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Item {
    name: &'static str,
    cost: i32,
    damage: i32,
    armor: i32
}

const WEAPONS: [Item; 5] = [
    Item { name: "Dagger",     cost: 8,  damage: 4, armor: 0 },
    Item { name: "Shortsword", cost: 10, damage: 5, armor: 0 },
    Item { name: "Warhammer",  cost: 25, damage: 6, armor: 0 },
    Item { name: "Longsword",  cost: 40, damage: 7, armor: 0 },
    Item { name: "Greataxe",   cost: 74, damage: 8, armor: 0 }
];

const ARMORS: [Item; 5] = [
    Item { name: "Leather",    cost: 13,  damage: 0, armor: 1 },
    Item { name: "Chainmail",  cost: 31,  damage: 0, armor: 2 },
    Item { name: "Splintmail", cost: 53,  damage: 0, armor: 3 },
    Item { name: "Bandedmail", cost: 75,  damage: 0, armor: 4 },
    Item { name: "Platemail",  cost: 102, damage: 0, armor: 5 }
];

const RINGS: [Item; 6] = [
    Item { name: "Damage +1",  cost: 25,  damage: 1, armor: 0 },
    Item { name: "Damage +2",  cost: 50,  damage: 2, armor: 0 },
    Item { name: "Damage +3",  cost: 100, damage: 3, armor: 0 },
    Item { name: "Defense +1", cost: 20,  damage: 0, armor: 1 },
    Item { name: "Defense +2", cost: 40,  damage: 0, armor: 2 },
    Item { name: "Defense +3", cost: 80,  damage: 0, armor: 3 }
];

fn battle(boss: &Participant, player: &Participant) -> i32 {
    let players = [player, boss];
    let mut hps = [player.hp, boss.hp];
    let mut round = 0;
    #[cfg(feature = "print")] let names = ["player", "boss"];

    while hps[0] > 0 && hps[1] > 0 {
        let attacker = round % 2;
        let target = (round + 1) % 2;
        let damage = cmp::max(players[attacker].damage - players[target].armor, 1);
        hps[target] -= damage;
        #[cfg(feature = "print")] {
            println!("The {} deals {}-{} = {} damage; the {} goes down to {} hit points.",
                    names[attacker], players[attacker].damage, players[target].armor, damage, names[target], hps[target]);
        }
        round += 1;
    }

    hps[0] - hps[1]
}

fn solve(input: &Input) -> (i32, i32) {

    let mut min_cost = std::i32::MAX;
    let mut max_cost = 0;

    for w_idx in 0..WEAPONS.len() {
        for a_idx in -1..(ARMORS.len() as i32) {
            for r0_idx in -1..(RINGS.len() as i32) {
                for r1_idx in -1..(RINGS.len() as i32) {
                    if r0_idx < 0 && r1_idx >= 0 || (r0_idx >= 0 && r0_idx == r1_idx) {
                        continue;
                    }

                    let damage = WEAPONS[w_idx].damage
                        + if a_idx  >= 0 { ARMORS[a_idx as usize].damage } else { 0 }
                        + if r0_idx >= 0 { RINGS[r0_idx as usize].damage } else { 0 }
                        + if r1_idx >= 0 { RINGS[r1_idx as usize].damage } else { 0 };
                    let armor = WEAPONS[w_idx].armor
                        + if a_idx  >= 0 { ARMORS[a_idx as usize].armor } else { 0 }
                        + if r0_idx >= 0 { RINGS[r0_idx as usize].armor } else { 0 }
                        + if r1_idx >= 0 { RINGS[r1_idx as usize].armor } else { 0 };
                    let cost = WEAPONS[w_idx].cost
                        + if a_idx  >= 0 { ARMORS[a_idx as usize].cost } else { 0 }
                        + if r0_idx >= 0 { RINGS[r0_idx as usize].cost } else { 0 }
                        + if r1_idx >= 0 { RINGS[r1_idx as usize].cost } else { 0 };

                    let player = Participant {
                        hp: 100,
                        damage: damage,
                        armor: armor
                    };

                    if battle(input, &player) > 0 {
                        min_cost = cmp::min(min_cost, cost);
                    } else {
                        max_cost = cmp::max(max_cost, cost);
                    }
                }
            }
        }
    }

    (min_cost, max_cost)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(Input::read(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "Hit Points: 12
        Damage: 7
        Armor: 2";

    fn as_input(s: &str) -> Input {
        let f = BufReader::new(s.as_bytes());
        Input::read(f)
    }

    #[test]
    fn test_battle() {
        let player = Participant { hp: 8, damage: 5, armor: 5 };
        assert_eq!(battle(&as_input(INPUT), &player), 2);
    }

    #[test]
    fn test_part1() {
        let (part1, _) = solve(&as_input(INPUT));
        assert_eq!(part1, 8);
    }

    #[test]
    fn test_part2() {
        let (_, part2) = solve(&as_input(INPUT));
        assert_eq!(part2, 0);
    }
}