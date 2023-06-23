use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    sequence::preceded,
    Finish, IResult,
};
use core::fmt;
use std::{
    collections::VecDeque,
    fs::read_to_string,
};

const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Add(i32),
    Noop,
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let add = preceded(tag("addx "), nom::character::complete::i32);
        alt((value(Self::Noop, noop), map(add, Self::Add)))(i)
    }

    fn cycles(self) -> u32 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl From<Instruction> for i32 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Add(v) => v,
            Instruction::Noop => 0,
        }
    }
}

struct CPU {
    instructions: VecDeque<Instruction>,
    current_inst: Option<(Instruction, u32)>,
    x_reg: i32,
    cycle: u32,
    display: Vec<u64>,
}

impl CPU {
    fn from_str(i: &str) -> Self {
        let mut x = Self {
            instructions: i
                .lines()
                .map(|line| all_consuming(Instruction::parse)(line).finish().unwrap().1)
                .collect(),
            current_inst: None,
            x_reg: 1,
            cycle: 0,
            display: vec![],
        };
        x.set_inst();
        x
    }

    fn set_inst(&mut self) {
        self.current_inst = self
            .instructions
            .pop_front()
            .map(|inst| (inst, inst.cycles()));
    }

    fn draw(&mut self) {
        let line = (self.cycle / 40) as usize;
        if line + 1 > self.display.len() {
            self.display.push(0);
        }

        let line = self.display.get_mut(line).unwrap();
        let mask = cycle_mask(self.cycle);
        let sprite = sprite_value(self.x_reg as _);
        *line |= mask & sprite;
    }

    fn step(&mut self) -> bool {
        match self.current_inst.as_mut() {
            Some((inst, cycles_left)) => {
                *cycles_left -= 1;
                if *cycles_left == 0 {
                    match inst {
                        Instruction::Add(x) => self.x_reg += *x,
                        Instruction::Noop => {}
                    }
                    self.set_inst();
                }
                self.cycle += 1;
                true
            }
            None => false,
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.display {
            for i in 0..40 {
                let c = if line & cycle_mask(i) > 0 {'#'} else { '.' } ;
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn sprite_value(pos: i32) -> u64 {
    let bitmask = 0b11100000000000000000000000000000000000000_u64;
    let shft;

    if pos < 0 {
        (shft, _) = bitmask.overflowing_shl((-pos).try_into().unwrap());
    } else {
        (shft, _) = bitmask.overflowing_shr(pos.try_into().unwrap());
    }
    shft & DISPLAY_MASK
}

fn cycle_mask(cycle: u32) -> u64 {
    (0b1000000000000000000000000000000000000000 >> (cycle % 40)) & DISPLAY_MASK
}

fn part_1(input: &str) -> i32 {
    let mut cpu = CPU::from_str(input);
    let mut total = 0;
    let count = [20, 60, 100, 140, 180, 220];

    while cpu.step() {
        if count.contains(&(cpu.cycle + 1)) {
            total += (cpu.cycle as i32 + 1) * cpu.x_reg;
        }
    }

    total
}

fn part_2(input: &str) -> String {
    let mut cpu = CPU::from_str(input);

    while cpu.step() {
        cpu.draw();
    }

    format!("{cpu:?}")
}

fn main() -> Result<()> {
    let input = &read_to_string("./test_files/day_10.txt").expect("File does not exist");
    println!("Part 1: {}", part_1(input));
    println!("Part 2:\n{}", part_2(input));

    Ok(())
}

#[test]
fn test_basic_step() {
    let input = "noop\naddx 3\naddx -5";
    let mut cpu = CPU::from_str(input);

    while cpu.step() {}

    assert_eq!(cpu.x_reg, -1);
}

#[test]
fn test_part_1() {
    let input = "addx 15\r\naddx -11\r\naddx 6\r\naddx -3\r\naddx 5\r\naddx -1\r\naddx -8\r\naddx 13\r\naddx 4\r\nnoop\r\naddx -1\r\naddx 5\r\naddx -1\r\naddx 5\r\naddx -1\r\naddx 5\r\naddx -1\r\naddx 5\r\naddx -1\r\naddx -35\r\naddx 1\r\naddx 24\r\naddx -19\r\naddx 1\r\naddx 16\r\naddx -11\r\nnoop\r\nnoop\r\naddx 21\r\naddx -15\r\nnoop\r\nnoop\r\naddx -3\r\naddx 9\r\naddx 1\r\naddx -3\r\naddx 8\r\naddx 1\r\naddx 5\r\nnoop\r\nnoop\r\nnoop\r\nnoop\r\nnoop\r\naddx -36\r\nnoop\r\naddx 1\r\naddx 7\r\nnoop\r\nnoop\r\nnoop\r\naddx 2\r\naddx 6\r\nnoop\r\nnoop\r\nnoop\r\nnoop\r\nnoop\r\naddx 1\r\nnoop\r\nnoop\r\naddx 7\r\naddx 1\r\nnoop\r\naddx -13\r\naddx 13\r\naddx 7\r\nnoop\r\naddx 1\r\naddx -33\r\nnoop\r\nnoop\r\nnoop\r\naddx 2\r\nnoop\r\nnoop\r\nnoop\r\naddx 8\r\nnoop\r\naddx -1\r\naddx 2\r\naddx 1\r\nnoop\r\naddx 17\r\naddx -9\r\naddx 1\r\naddx 1\r\naddx -3\r\naddx 11\r\nnoop\r\nnoop\r\naddx 1\r\nnoop\r\naddx 1\r\nnoop\r\nnoop\r\naddx -13\r\naddx -19\r\naddx 1\r\naddx 3\r\naddx 26\r\naddx -30\r\naddx 12\r\naddx -1\r\naddx 3\r\naddx 1\r\nnoop\r\nnoop\r\nnoop\r\naddx -9\r\naddx 18\r\naddx 1\r\naddx 2\r\nnoop\r\nnoop\r\naddx 9\r\nnoop\r\nnoop\r\nnoop\r\naddx -1\r\naddx 2\r\naddx -37\r\naddx 1\r\naddx 3\r\nnoop\r\naddx 15\r\naddx -21\r\naddx 22\r\naddx -6\r\naddx 1\r\nnoop\r\naddx 2\r\naddx 1\r\nnoop\r\naddx -10\r\nnoop\r\nnoop\r\naddx 20\r\naddx 1\r\naddx 2\r\naddx 2\r\naddx -6\r\naddx -11\r\nnoop\r\nnoop\r\nnoop";

    let output = part_1(input);

    assert_eq!(output, 13140);
}
