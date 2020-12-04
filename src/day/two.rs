use regex::Regex;

use crate::day::Day;

pub struct Two {}

impl Day for Two {
    type Input = Vec<(usize, usize, char, String)>;
    type Output = usize;

    fn part_one(passwords: &Self::Input) -> Self::Output {
        passwords
            .iter()
            .filter(|(min, max, ch, password)| {
                let found = password.chars().filter(|c| *c == *ch).count();
                found >= *min && found <= *max
            })
            .count()
    }

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

    fn print_results(one: Self::Output, two: Self::Output) {
        println!("Answer for Part One: {}", one);
        println!("Answer for Part Two: {}", two);
    }
}
