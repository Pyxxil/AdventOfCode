use std::collections::HashMap;

use crate::day::Day;

pub struct Four {}

impl Day for Four {
    type Input = Vec<HashMap<String, String>>;
    type Output = usize;

    ///
    /// Task:
    ///
    fn part_one(passports: &Self::Input) -> Self::Output {
        passports
            .iter()
            .filter(|passport| {
                passport.contains_key("byr")
                    && passport.contains_key("iyr")
                    && passport.contains_key("eyr")
                    && passport.contains_key("hgt")
                    && passport.contains_key("hcl")
                    && passport.contains_key("ecl")
                    && passport.contains_key("pid")
            })
            .count()
    }

    ///
    /// Task:
    ///
    fn part_two(passports: &Self::Input) -> Self::Output {
        passports
            .iter()
            .filter(|passport| {
                let empty = String::new();
                let zero = "0".to_string();
                let birth_year = passport.get("byr").map_or_else(|| 0, |p| p.parse::<i32>().unwrap());
                let issue_year = passport.get("iyr").map_or_else(|| 0, |p| p.parse::<i32>().unwrap());
                let expiration_year = passport.get("eyr").map_or_else(|| 0, |p| p.parse::<i32>().unwrap());
                let hair_colour = passport.get("hcl").unwrap_or(&empty);
                let eye_colour = passport.get("ecl").unwrap_or(&empty);
                let passport_id = passport.get("pid").unwrap_or(&empty);
                let height = passport.get("hgt").unwrap_or(&zero).chars();

                let (height_value, system): (String, String) = height.partition(|d| d.is_digit(10));
                let height_value = height_value.parse::<i32>().unwrap();

                if birth_year < 1920 || birth_year > 2002 {
                    false
                } else if issue_year < 2010 || issue_year > 2020 {
                    false
                } else if expiration_year < 2020 || expiration_year > 2030 {
                    false
                } else if hair_colour.chars().next().unwrap_or(' ') != '#' || hair_colour.len() < 7 || !hair_colour.chars().skip(1).all(|ch| ch.is_digit(16)) {
                    false
                } else if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|colour| colour == eye_colour) {
                    false
                } else if passport_id.len() != 9 || passport_id.chars().any(|d| !d.is_digit(10)) {
                    false
                } else if (system != "cm" && system != "in") || (system == "cm" && (height_value < 150 || height_value > 193)) || (system == "in" && (height_value < 59 || height_value > 76)) {
                    false
                } else {
                    true
                }
            }).count()
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_four");

        input
            .lines()
            .map(|line| line.trim())
            .fold(vec![HashMap::new()], |mut passports, line| {
                if line.is_empty() {
                    passports.push(HashMap::new());
                } else {
                    line.split(" ").for_each(|item| {
                        let item = item.split(':').collect::<Vec<_>>();

                        passports
                            .last_mut()
                            .unwrap()
                            .insert(item[0].to_string(), item[1].to_string());
                    });
                }

                passports
            })
    }
}
