use std::collections::HashMap;

use crate::Day;

pub struct Six {}

impl Day for Six {
    type Input = Vec<(usize, HashMap<char, usize>)>;
    type Output = usize;

    fn part_one(answers: &Self::Input) -> Self::Output {
        answers
            .iter()
            .fold(0, |count, (_, answer)| count + answer.keys().count())
    }

    fn part_two(answers: &Self::Input) -> Self::Output {
        answers.iter().fold(0, |count, (c, answer)| {
            count + answer.values().filter(|value| *value == c).count()
        })
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_six");

        input
            .lines()
            .map(&str::trim)
            .fold(vec![Vec::new()], |mut lines, line| {
                if line.is_empty() {
                    lines.push(Vec::new());
                } else {
                    lines.last_mut().unwrap().push(line.to_string());
                }

                lines
            })
            .iter()
            .map(|answer| {
                (
                    answer.len(),
                    answer.iter().fold(HashMap::new(), |mut values, value| {
                        value.chars().for_each(|ch| {
                            values.insert(ch, values.get(&ch).unwrap_or(&0) + 1);
                        });
                        values
                    }),
                )
            })
            .collect()
    }
}
