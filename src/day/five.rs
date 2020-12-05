use crate::day::Day;

pub struct Five {}

impl Day for Five {
    type Output = usize;
    type Input = Vec<String>;

    fn part_one(passes: &Self::Input) -> Self::Output {
        let (row, column) = passes.iter().fold((0, ""), |(max, st), pass| {
            let row = pass
                .chars()
                .take_while(|ch| ch == &'F' || ch == &'B')
                .fold((0, 127), |(min, max), p| {
                    if p == 'F' {
                        (min, (max - min) / 2 + min)
                    } else {
                        ((max - min) / 2 + min + 1, max)
                    }
                })
                .0;

            if row > max {
                (row, &pass)
            } else {
                (max, st)
            }
        });

        let column = column
            .chars()
            .skip_while(|ch| ch != &'L' && ch != &'R')
            .fold((0, 7), |(min, max), p| {
                if p == 'L' {
                    (min, (max - min) / 2 + min)
                } else {
                    ((max - min) / 2 + min + 1, max)
                }
            })
            .1;

        row * 8 + column
    }

    fn part_two(passes: &Self::Input) -> Self::Output {
        let mut ids = passes
            .iter()
            .map(|pass| {
                let (row, column): (String, String) =
                    pass.chars().partition(|ch| ch == &'F' || ch == &'B');

                let row = row
                    .chars()
                    .fold((0, 127), |(min, max), p| {
                        if p == 'F' {
                            (min, (max - min) / 2 + min)
                        } else {
                            ((max - min) / 2 + min + 1, max)
                        }
                    })
                    .0;
                let column = column
                    .chars()
                    .fold((0, 7), |(min, max), p| {
                        if p == 'L' {
                            (min, (max - min) / 2 + min)
                        } else {
                            ((max - min) / 2 + min + 1, max)
                        }
                    })
                    .1;

                row * 8usize + column
            })
            .collect::<Vec<_>>();

        ids.sort_unstable();

        *ids.iter()
            .enumerate()
            .skip(1)
            .find(|(idx, id)| ids[idx - 1] != *id - 1)
            .unwrap()
            .1 - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_five");

        input.lines().map(|line| line.trim().to_string()).collect()
    }
}
