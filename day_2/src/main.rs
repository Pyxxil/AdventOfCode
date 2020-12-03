use regex::Regex;

fn part_one(input: &str) -> i64 {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    input.lines().fold(0, |count, line| {
        let line = re.captures(line).unwrap();
        let min = line[1].parse::<i64>().unwrap();
        let max = line[2].parse::<i64>().unwrap();
        let ch = line[3].chars().nth(0).unwrap();
        let string = line[4].to_string();
        
        let found = string.chars().filter(|c| *c == ch).count() as i64;

        count + if found >= min && found <= max { 1 } else { 0 }
    })
}

fn part_two(input: &str) -> i64 {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    input.lines().fold(0, |count, line| {
        let line = re.captures(line).unwrap();
        let first = line[1].parse::<i64>().unwrap() - 1;
        let second = line[2].parse::<i64>().unwrap() - 1;
        let ch = line[3].chars().nth(0).unwrap();
        let string = line[4].to_string();

        let (l, r) = (string.chars().nth(first as usize).unwrap(), string.chars().nth(second as usize).unwrap());

        let valid = l == ch && r != ch || r == ch && l != ch;

        count + if valid { 1 } else { 0 }
    })
}

fn main() {
    let input = include_str!("input");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
