use crate::Day;

pub struct One {}

fn sum(expenses: &[i64], value: i64) -> Option<(i64, i64)> {
    let mut iter = expenses.iter();

    let mut left = iter.next().unwrap();
    let mut right = iter.next_back().unwrap();

    loop {
        let result = (left + right).cmp(&value);

        match result {
            std::cmp::Ordering::Less => left = iter.next()?,
            std::cmp::Ordering::Greater => right = iter.next_back()?,
            std::cmp::Ordering::Equal => return Some((*left, *right)),
        }
    }
}

impl Day for One {
    type Input = Vec<i64>;
    type Output = i64;

    ///
    /// Task: Find two entries in a file that sum to the value '2020'.
    ///
    fn part_one(expenses: &Self::Input) -> Self::Output {
        let (l, r) = sum(&expenses, 2020).unwrap();
        l * r
    }

    ///
    /// Task: Find three entries in a file that sum to the value '2020'
    ///
    fn part_two(expenses: &Self::Input) -> Self::Output {
        let mut iter = expenses.iter();

        while let Some(&v) = iter.next() {
            if let Some((l, r)) = sum(iter.as_slice(), 2020 - v) {
                return v * l * r;
            }
        }

        0
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_one");

        let mut expenses = input
            .lines()
            .filter_map(|expense| expense.parse::<i64>().ok())
            .collect::<Vec<_>>();
        expenses.sort_unstable();

        expenses
    }
}
