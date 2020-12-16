use crate::Day;

pub struct Nine {}

fn validate(values: &[i64], value: i64) -> bool {
    for (idx, i) in values.iter().enumerate() {
        for j in values.iter().skip(idx + 1) {
            if i + j == value {
                return true;
            }
        }
    }

    false
}

fn find_sum(values: &[i64], value: i64) -> i64 {
    (0..values.len())
        .find_map(|i| {
            ((i + 1)..values.len()).find_map(|j| {
                let vals = &values[i..=j];

                if vals.iter().sum::<i64>() == value {
                    Some(vals.iter().min().unwrap() + vals.iter().max().unwrap())
                } else {
                    None
                }
            })
        })
        .unwrap()
}

impl Day for Nine {
    type Input = Vec<i64>;
    type Output = i64;

    fn part_one(code: &Self::Input) -> Self::Output {
        let iter = code.iter().enumerate().skip(25);

        for (idx, value) in iter {
            if !validate(&code[(idx - 25)..idx], *value) {
                return *value;
            }
        }

        0
    }

    fn part_two(code: &Self::Input) -> Self::Output {
        let iter = code.iter().enumerate().skip(25);

        for (idx, value) in iter {
            let combination = &code[(idx - 25)..idx];
            if !validate(combination, *value) {
                return find_sum(&code[0..idx], *value);
            }
        }

        0
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_nine");

        input
            .lines()
            .map(str::trim)
            .filter_map(|line| line.parse::<i64>().ok())
            .collect()
    }
}
