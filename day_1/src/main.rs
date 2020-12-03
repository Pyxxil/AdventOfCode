
fn part_one(input: &str) -> i64 {
    let mut expenses = input.lines().filter_map(|expense| expense.parse::<i64>().ok()).collect::<Vec<_>>();
    expenses.sort_unstable();

    let (mut l, mut r) = (0, expenses.len() - 1);

    loop {
        let result = (expenses[l] + expenses[r]).cmp(&2020);

        match result {
            std::cmp::Ordering::Less => l += 1,
            std::cmp::Ordering::Greater => r -= 1,
            _ => break,
        }
    }

    expenses[l] * expenses[r]
}

fn part_two(input: &str) -> i64 {
    let mut expenses = input.lines().filter_map(|expense| expense.parse::<i64>().ok()).collect::<Vec<_>>();
    expenses.sort_unstable();

    let (mut l, mut m, mut r) = (0, expenses.len() / 2, expenses.len() - 1);

    loop {
        let result = (expenses[l] + expenses[m] + expenses[r]).cmp(&2020);

        match result {
            std::cmp::Ordering::Less => {
                if l + 1 == m {
                    m += 1
                } else {
                    l += 1;
                }
            },
            std::cmp::Ordering::Greater => {
                if r - 1 == m {
                    m -= 1;
                } else {
                    r -= 1;
                }
            },
            _ => break,
        }
    }

    expenses[l] * expenses[m] * expenses[r]
}

fn main() {
    let input = include_str!("input");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
