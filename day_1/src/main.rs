fn main() {
    let input = include_str!("input");

    let mut expenses = input.lines().filter_map(|expense| expense.parse::<i32>().ok()).collect::<Vec<_>>();
    expenses.sort_unstable();

    let mut found = false;
    let (mut l, mut r) = (0, expenses.len() - 1);

    while !found {
        let result = (expenses[l] + expenses[r]).cmp(&2020);

        match result {
            std::cmp::Ordering::Less => l += 1,
            std::cmp::Ordering::Greater => r -= 1,
            _ => found = true
        }
    }

    println!("{}", expenses[l] * expenses[r]);
}
