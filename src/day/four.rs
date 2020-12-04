use std::collections::HashMap;

use crate::day::Day;

pub struct Four {}

static EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
static PASSPORT_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub struct Passport {
    hair_colour: String,
    eye_colour: String,
    birth_year: i32,
    issue_year: i32,
    height: i32,
    system: String,
    expiration_year: i32,
    id: String,
}

impl Passport {
    pub fn check(map: &HashMap<String, String>) -> bool {
        PASSPORT_KEYS.iter().all(|key| map.contains_key(*key))
    }

    pub fn from(map: &HashMap<String, String>) -> Result<Self, ()> {
        let birth_year = map.get("byr").ok_or(())?.parse::<i32>().or(Err(()))?;
        let issue_year = map.get("iyr").ok_or(())?.parse::<i32>().or(Err(()))?;
        let expiration_year = map.get("eyr").ok_or(())?.parse::<i32>().or(Err(()))?;
        let height = map
            .get("hgt")
            .ok_or(())?
            .chars()
            .take_while(|ch| ch.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .or(Err(()))?;
        let system = map
            .get("hgt")
            .ok_or(())?
            .chars()
            .skip_while(|ch| ch.is_digit(10))
            .collect::<String>();
        let hair_colour = map.get("hcl").ok_or(())?.clone();
        let eye_colour = map.get("ecl").ok_or(())?.clone();
        let passport_id = map.get("pid").ok_or(())?.clone();

        Ok(Self {
            hair_colour,
            eye_colour,
            birth_year,
            issue_year,
            height,
            system,
            expiration_year,
            id: passport_id,
        })
    }

    pub fn is_valid(&self) -> bool {
        if self.birth_year < 1920
            || self.birth_year > 2002
            || self.issue_year < 2010
            || self.issue_year > 2020
            || self.expiration_year < 2020
            || self.expiration_year > 2030
            || self.id.len() != 9
            || self.hair_colour.len() != 7
            || self.hair_colour.chars().next().unwrap_or(' ') != '#'
            || self.hair_colour.chars().skip(1).any(|ch| !ch.is_digit(16))
            || !EYE_COLOURS.iter().any(|c| c == &self.eye_colour)
        {
            false
        } else {
            (self.system == "cm" && self.height >= 150 && self.height <= 193)
                || (self.system == "in" && self.height >= 59 && self.height <= 76)
        }
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
        passports.len()
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
            .filter_map(|passport| Passport::from(passport).ok())
            .filter(&Passport::is_valid)
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
                        let item = item.split(':').collect::<Vec<_>>();

                        passports
                            .last_mut()
                            .unwrap()
                            .insert(item[0].to_string(), item[1].to_string());
                    });
                }

                passports
            })
            .into_iter()
            .filter(&Passport::check)
            .collect()
    }
}
