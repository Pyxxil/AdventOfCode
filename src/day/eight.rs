use std::collections::HashSet;

use crate::day::Day;

pub struct Eight { }

fn run(program: &[(String, i64)]) -> Option<i64> {
    let mut pc = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();

    loop {
        if executed.contains(&pc) {
            return None;
        } else {
            executed.insert(pc);
        }

        if let Some((instruction, value)) = program.get(pc as usize) {
            match instruction.as_str() {
                "nop" => pc += 1,
                "acc" => {
                    acc += value;
                    pc += 1;
                }
                "jmp" => {
                    pc += value;
                }
                _ => {}
            }
        } else {
            break;
        }
    }

    Some(acc)
}

impl Day for Eight {
    type Input = Vec<(String, i64)>;
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

            let (instruction, value) = program.get(pc as usize).unwrap();

            match instruction.as_str() {
                "nop" => pc += 1,
                "acc" => {
                    acc += value;
                    pc += 1;
                }
                "jmp" => {
                    pc += value;
                }
                _ => {}
            }
        }

        acc
    }

    fn part_two(program: &Self::Input) -> Self::Output {
        let mut cloned_program = program.clone();
        program
            .iter()
            .enumerate()
            .find_map(|(idx, ins)| match ins.0.as_str() {
                "jmp" => {
                    cloned_program.get_mut(idx).unwrap().0 = "nop".to_string();
                    let acc = run(&cloned_program);
                    cloned_program.get_mut(idx).unwrap().0 = "jmp".to_string();
                    acc
                }
                "nop" => {
                    cloned_program.get_mut(idx).unwrap().0 = "jmp".to_string();
                    let acc = run(&cloned_program);
                    cloned_program.get_mut(idx).unwrap().0 = "nop".to_string();
                    acc
                }
                _ => None,
            })
            .expect("Could not find a swap that let's the program terminate")
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_eight");

        input
            .lines()
            .map(&str::trim)
            .map(|line| line.split_once(' ').unwrap())
            .map(|(ins, val)| (ins.to_string(), val.parse::<i64>().unwrap()))
            .collect()
    }
}
