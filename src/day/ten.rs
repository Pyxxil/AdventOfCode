use std::collections::HashMap;

use crate::Day;

pub struct Ten {}

impl Day for Ten {
    type Input = Vec<i64>;
    type Output = i64;

    fn part_one(adapters: &Self::Input) -> Self::Output {
        let (mut one, mut three) = (0, 1);

        adapters.iter().fold(0, |current, next| {
            match next - current {
                1 => one += 1,
                3 => three += 1,
                _ => {}
            }

            *next
        });

        one * three
    }

    fn part_two(adapters: &Self::Input) -> Self::Output {
        let mut tree = HashMap::new();
        tree.insert(0, 1);
        adapters.iter().fold(tree, |mut tree, i| {
            let arrangements = tree.get(&(i - 1)).unwrap_or(&0)
                + tree.get(&(i - 2)).unwrap_or(&0)
                + tree.get(&(i - 3)).unwrap_or(&0);
            tree.insert(*i, arrangements);
            tree
        })[adapters.last().unwrap()]
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_ten");

        let mut adapters = input
            .lines()
            .map(str::trim)
            .filter_map(|line| line.parse::<i64>().ok())
            .collect::<Vec<_>>();

        adapters.sort_unstable();

        adapters
    }
}
