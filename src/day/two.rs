use regex::Regex;

use crate::Day;

pub struct Two {}

impl Day for Two {
    type Input = Vec<(usize, usize, char, String)>;
    type Output = usize;

    ///
    /// Task: Determine the number of valid passwords, where a valid
    ///       password contains a specific character a certain number
    ///       of times (provided as 'x-y' in the file).
    ///
    fn part_one(passwords: &Self::Input) -> Self::Output {
        passwords
            .iter()
            .filter(|(min, max, ch, password)| {
                let found = password.chars().filter(|c| *c == *ch).count();
                found >= *min && found <= *max
            })
            .count()
    }

    ///
    /// Task: Determine the number of valid passwords, where a valid
    ///       password has a particular character occur at position x
    ///       or position y, but not both (where position x and y are
    ///       provided as in `part_one`).
    ///
    fn part_two(passwords: &Self::Input) -> Self::Output {
        passwords
            .iter()
            .filter(|(first, second, ch, password)| {
                let mut characters = password.chars();
                let (l, r) = (
                    characters.nth(*first).unwrap(),
                    characters.nth(*second - *first - 1).unwrap(),
                );

                l == *ch && r != *ch || r == *ch && l != *ch
            })
            .count()
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_two");

        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        let passwords = input
            .lines()
            .map(|line| {
                let line = re.captures(line).unwrap();
                let first = line[1].parse::<usize>().unwrap() - 1;
                let second = line[2].parse::<usize>().unwrap() - 1;
                let ch = line[3].chars().next().unwrap();
                let string = line[4].to_string();

                (first, second, ch, string)
            })
            .collect::<Self::Input>();

        passwords
    }
}
