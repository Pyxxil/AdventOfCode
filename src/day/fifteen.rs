use std::collections::HashMap;

use crate::Day;

pub struct Fifteen {}

fn play(numbers: &[usize], until: usize) -> usize {
    let mut said = numbers
        .iter()
        .take(numbers.len() - 1)
        .enumerate()
        .map(|(idx, &num)| (num, (idx + 1, 0)))
        .collect::<HashMap<_, _>>();

    (numbers.len()..until).fold(*numbers.last().unwrap(), |current, i| {
        let entry = said.entry(current).or_insert((i, i));
        let (prev, _) = *entry;

        *entry = (i, prev);

        i - prev
    })
}

impl Day for Fifteen {
    type Input = Vec<usize>;
    type Output = usize;

    fn part_one(numbers: &Self::Input) -> Self::Output {
        play(numbers, 2020)
    }

    fn part_two(numbers: &Self::Input) -> Self::Output {
        play(numbers, 30000000)
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_fifteen");

        input
            .trim()
            .split(',')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect()
    }
}
