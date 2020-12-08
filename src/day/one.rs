use crate::day::Day;

pub struct One {}

impl Day for One {
    type Input = Vec<i64>;
    type Output = i64;

    ///
    /// Task: Find two entries in a file that sum to the value '2020'.
    ///
    fn part_one(expenses: &Self::Input) -> Self::Output {
        let (mut l, mut r) = (0, expenses.len() - 1);

        loop {
            let result = (expenses[l] + expenses[r]).cmp(&2020);

            match result {
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Greater => r -= 1,
                std::cmp::Ordering::Equal => break,
            }
        }

        expenses[l] * expenses[r]
    }

    ///
    /// Task: Find three entries in a file that sum to the value '2020'
    ///
    fn part_two(expenses: &Self::Input) -> Self::Output {
        let (mut l, mut m, mut r) = (0, expenses.len() / 2, expenses.len() - 1);

        loop {
            let result = (expenses[l] + expenses[m] + expenses[r]).cmp(&2020);

            match result {
                std::cmp::Ordering::Less => {
                    if l + 1 == m {
                        // We've reached the biggest value that's smaller than
                        // the middle value
                        m += 1
                    } else {
                        l += 1;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if r - 1 == m {
                        // We've reached the smallest value that's bigger than
                        // the middle value
                        m -= 1;
                    } else {
                        r -= 1;
                    }
                }
                std::cmp::Ordering::Equal => break,
            }
        }

        expenses[l] * expenses[m] * expenses[r]
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
