use std::collections::HashMap;

use crate::day::Day;

pub struct Seven {}

fn find_rules(parsed: &[&str]) -> HashMap<String, usize> {
    let rules = parsed.iter().skip(1).collect::<Vec<&&str>>();
    rules
        .first()
        .unwrap()
        .split(',')
        .map(|rule| rule.trim().split_once(' ').unwrap())
        .fold(HashMap::new(), |mut rules, rule| {
            if rule.0 == "no" {
                rules
            } else {
                rules.insert(
                    rule.1.split(' ').take(2).collect::<String>(),
                    rule.0.parse::<usize>().unwrap(),
                );
                rules
            }
        })
}

fn search<'a>(
    mapping: &'a HashMap<String, HashMap<String, usize>>,
    rule: &'a HashMap<String, usize>,
    r: &'a str,
    mem: &mut HashMap<&'a str, bool>,
) -> bool {
    if !mem.contains_key(r) {
        let res = rule.iter().any(|(r, v)| {
            if *v == 0 {
                false
            } else if r == "shinygold" {
                true
            } else {
                search(mapping, mapping.get(r).unwrap(), r, mem)
            }
        });

        mem.insert(r, res);
    }

    *mem.get(r).unwrap()
}

fn traverse(
    mapping: &HashMap<String, HashMap<String, usize>>,
    rule: &HashMap<String, usize>,
) -> usize {
    rule.iter().fold(1, |count, (r, v)| {
        count + v * traverse(mapping, mapping.get(r).unwrap())
    })
}

impl Day for Seven {
    type Input = HashMap<String, HashMap<String, usize>>;
    type Output = usize;

    fn part_one(mapping: &Self::Input) -> Self::Output {
        let mut mem = HashMap::new();
        mapping
            .iter()
            .filter(|(r, rule)| search(mapping, rule, r, &mut mem))
            .count()
    }

    fn part_two(mapping: &Self::Input) -> Self::Output {
        traverse(mapping, mapping.get("shinygold").unwrap()) - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_seven");

        input.lines().fold(HashMap::new(), |mut mapping, line| {
            let parse = line.split(" contain ").collect::<Vec<_>>();

            let for_rule = parse
                .first()
                .unwrap()
                .split(' ')
                .take(2)
                .collect::<String>();

            mapping.insert(for_rule, find_rules(&parse));

            mapping
        })
    }
}
