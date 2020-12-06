use crate::day::Day;

pub struct Five {}

impl Day for Five {
    type Output = usize;
    type Input = Vec<String>;

    fn part_one(passes: &Self::Input) -> Self::Output {
        let (row, column) = passes.iter().fold((0, ""), |(max, st), pass| {
            let row = pass.chars().take(7).enumerate().fold(0, |id, (idx, ch)| {
                if ch == 'B' {
                    id + (1 << (9 - idx))
                } else {
                    id
                }
            });

            if row > max {
                (row, &pass)
            } else {
                (max, st)
            }
        });

        let column = column.chars().skip(7).enumerate().fold(0, |id, (idx, ch)| {
            if ch == 'R' {
                id + (1 << (3 - idx))
            } else {
                id
            }
        });

        row + column
    }

    fn part_two(passes: &Self::Input) -> Self::Output {
        let mut ids = passes
            .iter()
            .map(|pass| {
                pass.chars().enumerate().fold(0, |id, (idx, ch)| {
                    if ch == 'B' || ch == 'R' {
                        id + (1 << (9 - idx))
                    } else {
                        id
                    }
                })
            })
            .collect::<Vec<_>>();

        ids.sort_unstable();

        *ids.iter()
            .enumerate()
            .skip(1)
            .find(|(idx, id)| ids[idx - 1] != *id - 1)
            .unwrap()
            .1
            - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_five");

        input.lines().map(|line| line.trim().to_string()).collect()
    }
}
