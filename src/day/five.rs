use crate::day::Day;

pub struct Five { }


impl Day for Five {
    type Output = usize;
    type Input = Vec<String>;

    fn part_one(passes: &Self::Input) -> Self::Output {
        passes.iter().map(|pass| {
            let (row, column): (String, String) = pass.chars().partition(|ch| ch == &'F' || ch == &'B');
            let row = row.chars().fold((0, 127), |(min, max), p| {
                if p == 'F' {
                    (min, (max - min) / 2 + min)
                } else {
                    ((max - min) / 2 + min + 1, max)
                }
            }).0;

            let column = column.chars().fold((0, 7), |(min, max), p| {
                if p == 'L' {
                    (min, (max - min) / 2 + min)
                } else {
                    ((max - min) / 2 + min + 1, max)
                }
            }).1;

            row * 8 + column
        }).max().unwrap()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        0
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_five");

        input.lines().map(|line| line.trim().to_string()).collect()
    }

}
