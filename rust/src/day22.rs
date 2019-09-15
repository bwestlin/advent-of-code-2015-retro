#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::cmp;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

use EffectModifier::*;

type Input = Boss;

#[derive(Clone, Debug)]
struct Boss {
    hp: i32,
    damage: i32
}

impl Boss {
    fn read<R: Read>(reader: BufReader<R>) -> Boss {
        let mut l = reader.lines().map(|l| l.unwrap().split(":").skip(1).next().unwrap().trim().parse::<i32>().unwrap());
        Boss { hp: l.next().unwrap(), damage: l.next().unwrap() }
    }
}

#[derive(Clone, Debug)]
struct Player {
    hp: i32,
    mana: i32,
    spent: i32
}

#[derive(Debug)]
enum EffectModifier {
    Damage, Healing, Armor, Mana
}

#[derive(Debug)]
struct Effect {
    modifier: EffectModifier,
    value: i32,
    turns: i32
}

impl Effect {
    fn apply(&self, boss: &mut Boss, player: &mut Player, armor: &mut i32) {
        match self.modifier {
            Damage => {
                boss.hp -= self.value;
            },
            Healing => {
                player.hp += self.value;
            },
            Armor => {
                *armor += self.value;
            },
            Mana => {
                player.mana += self.value;
            }
        }
    }
}

#[derive(Debug)]
struct Spell {
    name: &'static str,
    cost: i32,
    effects: Vec<Effect>
}

lazy_static! {
    static ref SPELLS: [Spell; 5] = [
        Spell {
            name: "Magic Missile",
            cost: 53,
            effects: vec![Effect { modifier: Damage, value: 4, turns: 0 }]
        },
        Spell {
            name: "Drain",
            cost: 73,
            effects: vec![Effect { modifier: Damage, value: 2, turns: 0 }, Effect { modifier: Healing, value: 2, turns: 0 }]
        },
        Spell {
            name: "Shield",
            cost: 113,
            effects: vec![Effect { modifier: Armor, value: 7, turns: 6 }]
        },
        Spell {
            name: "Poison",
            cost: 173,
            effects: vec![Effect { modifier: Damage, value: 3, turns: 6 }]
        },
        Spell {
            name: "Recharge",
            cost: 229,
            effects: vec![Effect { modifier: Mana, value: 101, turns: 5 }]
        },
    ];
}

type NextSpellsFn = Fn(&Step) -> Vec<usize>;

fn default_next_spells(step: &Step) -> Vec<usize> {
    let mut next_spells = Vec::with_capacity(SPELLS.len());
    for (idx, spell) in SPELLS.iter().enumerate() {
        if spell.cost < step.player.mana && !step.effects.iter().any(|(_, spell_idx, _)| *spell_idx == idx) {
            next_spells.push(idx);
        }
    }
    next_spells
}

#[derive(Debug)]
struct Step {
    turn: i32,
    boss: Boss,
    player: Player,
    effects: Vec<(i32, usize, usize)>
}

impl Step {
    fn run_battle(&mut self) {
        if self.player.hp > 0 {
            #[cfg(feature = "print")] {
                println!("-- {} turn --", if self.turn % 2 == 0 { "Player" } else { "Boss" });
            }

            let mut armor = 0;

            #[cfg(feature = "print")] {
                println!("- Player has {} hit points, {} armor, {} mana", self.player.hp, armor, self.player.mana);
                println!("- Boss has {} hit points", self.boss.hp);
            }

            let mut effects = Vec::with_capacity(self.effects.len());
            for (timer, spell_idx, effect_idx) in self.effects.iter() {
                let spell = &SPELLS[*spell_idx];
                let effect = &spell.effects[*effect_idx];
                effect.apply(&mut self.boss, &mut self.player, &mut armor);

                #[cfg(feature = "print")] {
                    println!("{} applies {} {:?}; its timer is now {}.", spell.name, effect.value, effect.modifier, timer - 1);
                }

                if *timer > 1 {
                    effects.push((timer - 1, *spell_idx, *effect_idx));
                } else {
                    #[cfg(feature = "print")] {
                        println!("{} wears off.", spell.name);
                    }
                }
            }
            self.effects = effects;

            if self.boss_turn() && !self.finished() {
                #[cfg(feature = "print")] {
                    println!("Boss attacks for {} - {} = {} damage.", self.boss.damage, armor, self.boss.damage - armor);
                }
                self.player.hp -= self.boss.damage - armor;
            }
        }
    }

    fn next_steps(&mut self, next_spells: &NextSpellsFn) -> Vec<Step> {
        let mut next = Vec::with_capacity(SPELLS.len());
        if self.player_turn() {
            for spell_idx in next_spells(&self) {
                let spell = &SPELLS[spell_idx];
                let mut player = self.player.clone();
                let mut boss = self.boss.clone();
                #[cfg(feature = "print")] {
                    println!("Player casts {}", spell.name);
                }
                player.mana -= spell.cost;
                player.spent += spell.cost;
                let mut next_effects = self.effects.clone();

                for (i, effect) in spell.effects.iter().enumerate() {
                    if effect.turns == 0 {
                        #[cfg(feature = "print")] {
                            println!("{} applies {} {:?}.", spell.name, effect.value, effect.modifier);
                        }
                        let mut armor = 0;
                        effect.apply(&mut boss, &mut player, &mut armor);
                    } else {
                        next_effects.push((effect.turns, spell_idx, i));
                    }
                }
                next.push(Step { turn: self.turn + 1, boss: boss, player: player, effects: next_effects });
            }
        }
        next
    }

    fn player_turn(&self) -> bool {
        self.turn % 2 == 0
    }

    fn boss_turn(&self) -> bool {
        self.turn % 2 == 1
    }

    fn player_lose_one(&mut self) {
        if self.player_turn() {
            self.player.hp -= 1;
        }
    }

    fn finished(&self) -> bool {
        self.boss.hp <= 0 || self.player.hp <= 0 || self.player.mana <= 0
    }

    fn player_wins(&self) -> bool {
        self.boss.hp <= 0 && self.player.hp > 0 && self.player.mana > 0
    }
}

fn least_mana(boss: &Boss, player: &Player, next_spells: &NextSpellsFn, player_lose_one: bool) -> i32 {
    let mut min_mana = std::i32::MAX;
    let mut queue = VecDeque::new();
    queue.push_front(Step { turn: 0, boss: boss.clone(), player: player.clone(), effects: vec![] });

    while let Some(mut step) = queue.pop_front() {
        if player_lose_one {
            step.player_lose_one();
        }

        step.run_battle();
        if step.finished() {
            if step.player_wins() {
                min_mana = cmp::min(min_mana, step.player.spent);
            }
            #[cfg(feature = "print")] {
                println!("{} wins.", if step.player_wins() { "Player" } else { "Boss" });
            }
        } else {
            if step.boss_turn() {
                step.turn += 1;
                queue.push_front(step);
            } else {
                for next_step in step.next_steps(next_spells) {
                    if next_step.player.spent < min_mana {
                        queue.push_front(next_step);
                    }
                }
            }
        }

        #[cfg(feature = "print")] {
            println!();
        }
    }

    min_mana
}

fn part1(boss: &Boss, player: &Player, next_spells: &NextSpellsFn) -> i32 {
    least_mana(boss, player, next_spells, false)
}

fn part2(boss: &Boss, player: &Player, next_spells: &NextSpellsFn) -> i32 {
    least_mana(boss, player, next_spells, true)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let player = Player { hp: 50, mana: 500, spent: 0 };
        println!("Part1: {}", part1(&input, &player, &default_next_spells));
        println!("Part2: {}", part2(&input, &player, &default_next_spells));
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
       "Hit Points: 13
        Damage: 8";

    fn as_input(s: &str) -> Input {
        let f = BufReader::new(s.as_bytes());
        Input::read(f)
    }

    #[test]
    fn test_least_mana() {
        #[cfg(feature = "print")] {
            println!("Test 1\n\n--\n");
        }
        let mut boss = as_input(INPUT);
        let player = Player { hp: 10, mana: 250, spent: 0 };
        fn next_spells(step: &Step) -> Vec<usize> {
            let spells_idx = vec![3, 0];
            vec![spells_idx[(step.turn / 2) as usize]]
        }
        assert_eq!(least_mana(&boss, &player, &next_spells, false), 173 + 53);

        #[cfg(feature = "print")] {
            println!("Test 2\n\n--\n");
        }
        boss.hp = 14;
        fn next_spells2(step: &Step) -> Vec<usize> {
            let spells_idx = vec![4, 2, 1, 3, 0];
            vec![spells_idx[(step.turn / 2) as usize]]
        }
        assert_eq!(least_mana(&boss, &player, &next_spells2, false), 229 + 113 + 73 + 173 + 53);
    }
}