use std::collections::HashSet;

use crate::Day;

pub struct Eight {}

#[derive(Clone, Copy)]
pub enum Op {
    Nop(i64),
    Jmp(i64),
    Acc(i64),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s.split_once(' ').unwrap() {
            ("nop", v) => Self::Nop(v.parse::<i64>().unwrap()),
            ("jmp", v) => Self::Jmp(v.parse::<i64>().unwrap()),
            ("acc", v) => Self::Acc(v.parse::<i64>().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn run(program: &[Op]) -> Option<i64> {
    let mut pc = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();

    loop {
        if executed.contains(&pc) {
            return None;
        } else {
            executed.insert(pc);
        }

        if let Some(instruction) = program.get(pc as usize) {
            match instruction {
                Op::Nop(_) => pc += 1,
                Op::Acc(value) => {
                    acc += value;
                    pc += 1;
                }
                Op::Jmp(value) => {
                    pc += value;
                }
            }
        } else {
            break;
        }
    }

    Some(acc)
}

impl Day for Eight {
    type Input = Vec<Op>;
    type Output = i64;

    fn part_one(program: &Self::Input) -> Self::Output {
        let mut pc = 0;
        let mut acc = 0;
        let mut executed = HashSet::new();

        loop {
            if executed.contains(&pc) {
                break;
            } else {
                executed.insert(pc);
            }

            let instruction = program.get(pc as usize).unwrap();

            match instruction {
                Op::Nop(_) => pc += 1,
                Op::Acc(value) => {
                    acc += value;
                    pc += 1;
                }
                Op::Jmp(value) => {
                    pc += value;
                }
            }
        }

        acc
    }

    fn part_two(program: &Self::Input) -> Self::Output {
        let mut cloned_program = program.clone();

        program
            .iter()
            .enumerate()
            .find_map(|(idx, instruction)| match instruction {
                Op::Jmp(v) => {
                    *cloned_program.get_mut(idx).unwrap() = Op::Nop(*v);
                    let acc = run(&cloned_program);
                    *cloned_program.get_mut(idx).unwrap() = Op::Jmp(*v);
                    acc
                }
                Op::Nop(v) => {
                    *cloned_program.get_mut(idx).unwrap() = Op::Jmp(*v);
                    let acc = run(&cloned_program);
                    *cloned_program.get_mut(idx).unwrap() = Op::Nop(*v);
                    acc
                }
                _ => None,
            })
            .expect("Could not find a swap that let's the program terminate")
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_eight");

        input.lines().map(str::trim).map(Op::from).collect()
    }
}
