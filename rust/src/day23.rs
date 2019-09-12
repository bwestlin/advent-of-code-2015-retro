extern crate utils;

use std::env;
use std::error;
use std::fmt;
use std::str::FromStr;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Program;

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn read<R: Read>(reader: BufReader<R>) -> Program {
        Program {
            instructions: reader.lines().map(|l| l.unwrap().trim().parse::<Instruction>().unwrap()).collect()
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32)
}

#[derive(Debug, Clone)]
pub struct InstructionParseError;
impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse instruction")
    }
}
impl error::Error for InstructionParseError {
    fn description(&self) -> &str {
        "unable to parse instruction"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let name = &s[0..3];
        let args: Vec<_> = (&s[3..]).split(',').map(|a| a.trim()).collect();

        fn to_rgister(r: &str) -> Register {
            match r {
                "a" => Register::A,
                "b" => Register::B,
                _ => unreachable!()
            }
        }

        fn offset(o: &str) -> i32 {
            o.parse().unwrap()
        }

        match name {
            "hlf" => Ok(Hlf(to_rgister(args[0]))),
            "tpl" => Ok(Tpl(to_rgister(args[0]))),
            "inc" => Ok(Inc(to_rgister(args[0]))),
            "jmp" => Ok(Jmp(offset(args[0]))),
            "jie" => Ok(Jie(to_rgister(args[0]), offset(args[1]))),
            "jio" => Ok(Jio(to_rgister(args[0]), offset(args[1]))),
            _ => Err(InstructionParseError {})
        }
    }
}

#[derive(Clone, Debug)]
enum Register {
    A, B
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [i32; 2],
    pc: usize
}

fn r_to_i(r: &Register) -> usize {
    match r {
        Register::A => 0,
        Register::B => 1,
    }
}

impl Computer {
    fn new() -> Computer {
        Computer { registers: [0, 0], pc: 0 }
    }

    fn run(&mut self, program: &Program) {
        use Instruction::*;

        while self.pc < program.instructions.len() {
            match &program.instructions[self.pc] {
                Hlf(r) => {
                    self.registers[r_to_i(r)] /= 2;
                    self.pc += 1;
                },
                Tpl(r) => {
                    self.registers[r_to_i(r)] *= 3;
                    self.pc += 1;
                },
                Inc(r) => {
                    self.registers[r_to_i(r)] += 1;
                    self.pc += 1;
                },
                Jmp(o) => {
                    self.pc = (self.pc as i32 + o) as usize;
                },
                Jie(r, o) => {
                    if self.registers[r_to_i(r)] % 2 == 0 {
                        self.pc = (self.pc as i32 + o) as usize;
                    } else {
                        self.pc += 1;
                    }
                },
                Jio(r, o) => {
                    if self.registers[r_to_i(r)] == 1 {
                        self.pc = (self.pc as i32 + o) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    fn set_register(&mut self, register: Register, value: i32) {
        self.registers[r_to_i(&register)] = value;
    }

    fn peek_register(&self, register: Register) -> i32 {
        self.registers[r_to_i(&register)]
    }
}

fn part1(input: &Input) -> i32 {
    let mut computer = Computer::new();
    computer.run(input);
    computer.peek_register(Register::B)
}

fn part2(input: &Input) -> i32 {
    let mut computer = Computer::new();
    computer.set_register(Register::A, 1);
    computer.run(input);
    computer.peek_register(Register::B)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
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
       "inc a
        jio a, +2
        tpl a
        inc a";

    fn as_input(s: &str) -> Input {
        let f = BufReader::new(s.as_bytes());
        Input::read(f)
    }

    #[test]
    fn test_computer() {
        let mut computer = Computer::new();
        computer.run(&as_input(INPUT));
        assert_eq!(computer.peek_register(Register::A), 2);
    }
}