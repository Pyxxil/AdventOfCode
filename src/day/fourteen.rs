use std::collections::HashMap;

use crate::Day;

pub struct Fourteen {}

#[derive(Copy, Clone)]
enum Bit {
    One,
    Zero,
    Floating,
}

impl From<char> for Bit {
    fn from(ch: char) -> Self {
        match ch {
            'X' => Bit::Floating,
            '1' => Bit::One,
            '0' => Bit::Zero,
            _ => unreachable!(),
        }
    }
}

pub struct Program {
    mask: [Bit; 36],
    memory: HashMap<usize, i64>,
}

impl Program {
    fn new() -> Self {
        Self {
            mask: [Bit::Zero; 36],
            memory: HashMap::new(),
        }
    }

    fn write(&mut self, address: usize, value: i64) {
        *self.memory.entry(address).or_insert(0) = value;
    }

    fn with(mut self, operations: &[Operation]) -> i64 {
        operations.iter().for_each(|operation| match operation {
            Operation::SetMask(mask) => mask
                .chars()
                .rev()
                .enumerate()
                .for_each(|(idx, bit)| self.mask[idx] = Bit::from(bit)),
            Operation::Write(address, value) => self.write(
                *address,
                self.mask.iter().enumerate().fold(0, |val, (idx, bit)| {
                    val | match bit {
                        Bit::Floating => value & (1 << idx),
                        Bit::One => 1 << idx,
                        Bit::Zero => 0,
                    }
                }),
            ),
        });

        self.memory.values().into_iter().sum()
    }

    fn with2(mut self, operations: &[Operation]) -> i64 {
        operations.iter().for_each(|operation| match operation {
            Operation::SetMask(mask) => mask
                .chars()
                .rev()
                .enumerate()
                .for_each(|(idx, bit)| self.mask[idx] = Bit::from(bit)),
            Operation::Write(address, value) => {
                self.mask
                    .iter()
                    .enumerate()
                    .fold(vec![*address], |mut addresses, (idx, bit)| {
                        match bit {
                            Bit::Floating => {
                                addresses = addresses
                                    .iter()
                                    .flat_map(|addr| vec![*addr & !(1 << idx), *addr | (1 << idx)])
                                    .collect();
                            }
                            Bit::One => {
                                addresses.iter_mut().for_each(|addr| *addr |= 1 << idx);
                            }
                            Bit::Zero => {}
                        };
                        addresses
                    })
                    .iter()
                    .for_each(|address| self.write(*address, *value));
            }
        });

        self.memory.values().into_iter().sum()
    }
}

pub enum Operation {
    SetMask(String),
    Write(usize, i64),
}

impl From<&str> for Operation {
    fn from(st: &str) -> Self {
        let (operand, value) = st.split_once(" = ").unwrap();
        match (operand, value) {
            ("mask", _) => Self::SetMask(value.to_string()),
            _ => Self::Write(
                {
                    let (addr, _): (String, String) = operand.chars().partition(|c| c.is_digit(10));
                    addr.parse::<usize>().unwrap()
                },
                value.parse::<i64>().unwrap(),
            ),
        }
    }
}

impl Day for Fourteen {
    type Input = Vec<Operation>;
    type Output = i64;

    fn part_one(operations: &Self::Input) -> Self::Output {
        Program::new().with(operations)
    }

    fn part_two(operations: &Self::Input) -> Self::Output {
        Program::new().with2(operations)
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_fourteen");

        input.lines().map(str::trim).map(Operation::from).collect()
    }
}
