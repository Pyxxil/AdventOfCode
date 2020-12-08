use std::collections::HashMap;

use crate::day::Day;

pub struct Four {}

static EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
static PASSPORT_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub struct Passport {}

impl Passport {
    pub fn check(passport: &HashMap<String, String>) -> bool {
        PASSPORT_KEYS.iter().all(|key| passport.contains_key(*key))
    }

    pub fn valid(passport: &HashMap<String, String>) -> bool {
        let empty = String::new();

        let birth_year = passport
            .get("byr")
            .unwrap_or(&String::from("0"))
            .parse::<i32>()
            .unwrap_or(0);
        if birth_year < 1920 || birth_year > 2002 {
            return false;
        }

        let issue_year = passport
            .get("iyr")
            .unwrap_or(&String::from("0"))
            .parse::<i32>()
            .unwrap_or(0);
        if issue_year < 2010 || issue_year > 2020 {
            return false;
        }

        let expiration_year = passport
            .get("eyr")
            .unwrap_or(&String::from("0"))
            .parse::<i32>()
            .unwrap_or(0);
        if expiration_year < 2020 || expiration_year > 2030 {
            return false;
        }

        let hair_colour = passport.get("hcl").unwrap_or(&empty);
        if hair_colour.len() != 7
            || !hair_colour.starts_with('#')
            || hair_colour.chars().skip(1).any(|ch| !ch.is_digit(16))
        {
            return false;
        }

        let eye_colour = passport.get("ecl").unwrap_or(&empty);
        if !EYE_COLOURS.iter().any(|c| c == eye_colour) {
            return false;
        }

        let passport_id = passport.get("pid").unwrap_or(&empty).clone();
        if passport_id.len() != 9 || !passport_id.chars().all(|ch| ch.is_digit(10)) {
            return false;
        }

        let height = passport
            .get("hgt")
            .unwrap_or(&String::from("0"))
            .chars()
            .take_while(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let system = passport
            .get("hgt")
            .unwrap_or(&String::from(""))
            .chars()
            .skip_while(|ch| ch.is_digit(10))
            .collect::<String>();

        (system == "cm" && height >= 150 && height <= 193)
            || (system == "in" && height >= 59 && height <= 76)
    }
}

impl Day for Four {
    type Input = Vec<HashMap<String, String>>;
    type Output = usize;

    ///
    /// Task: Determine how many valid passports there are, where a
    ///       valid passport contains the following:
    ///        - byr (Birth Year)
    ///        - iyr (Issue Year)
    ///        - eyr (Expiration Year)
    ///        - hgt (Height)
    ///        - hcl (Hair Color)
    ///        - ecl (Eye Color)
    ///        - pid (Passport ID)
    ///
    ///       There is an optional field (cid: Country ID).
    ///
    fn part_one(passports: &Self::Input) -> Self::Output {
        // Invalid (in accordance with this parts rules) will have been
        // filtered out before now
        passports
            .iter()
            .filter(|passport| Passport::check(*passport))
            .count()
    }

    ///
    /// Task: Determine how many passwords have the correct
    ///       corresponding values to them as well:
    ///        -byr (Birth Year) - four digits; at least 1920 and at most 2002.
    ///        -iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    ///        -eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    ///        -hgt (Height) - a number followed by either cm or in:
    ///             If cm, the number must be at least 150 and at most 193.
    ///             If in, the number must be at least 59 and at most 76.
    ///        -hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ///        -ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    ///        -pid (Passport ID) - a nine-digit number, including leading zeroes.
    ///        -cid (Country ID) - ignored, missing or not.
    ///
    fn part_two(passports: &Self::Input) -> Self::Output {
        passports
            .iter()
            .filter(|passport| Passport::valid(*passport))
            .count()
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_four");

        input
            .lines()
            .map(&str::trim)
            .fold(vec![HashMap::new()], |mut passports, line| {
                if line.is_empty() {
                    passports.push(HashMap::new());
                } else {
                    line.split(' ').for_each(|item| {
                        let mut item = item.split(':');

                        passports.last_mut().unwrap().insert(
                            item.next().unwrap().to_string(),
                            item.next().unwrap().to_string(),
                        );
                    });
                }

                passports
            })
    }
}
