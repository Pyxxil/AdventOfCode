use std::collections::HashMap;
use std::marker::PhantomData;

use crate::day::Day;

pub struct Seven<'a, T: 'a> {
    data: PhantomData<&'a T>,
}

type Edges<'a> = HashMap<&'a str, usize>;
type Graph<'a> = HashMap<&'a str, Edges<'a>>;

fn find_rules<'a>(parsed: &[&'a str]) -> Edges<'a> {
    parsed
        .iter()
        .next()
        .unwrap()
        .split(',')
        .map(|rule| rule.trim().split_once(' ').unwrap())
        .fold(HashMap::new(), |mut rules, rule| {
            if rule.0 == "no" {
                rules
            } else {
                rules.insert(
                    rule.1.rsplit_once(' ').unwrap().0,
                    rule.0.parse::<usize>().unwrap(),
                );
                rules
            }
        })
}

fn search<'a>(graph: &Graph<'a>, node: &'a str, mem: &mut HashMap<&'a str, bool>) -> bool {
    if !mem.contains_key(node) {
        let reaches = graph
            .get(node)
            .unwrap()
            .iter()
            .find(|(node, _)| **node == "shiny gold" || search(graph, node, mem))
            .is_some();

        mem.insert(node, reaches);
    }

    *mem.get(node).unwrap()
}

fn traverse<'a>(mapping: &Graph<'a>, rule: &HashMap<&'a str, usize>) -> usize {
    rule.iter().fold(1, |count, (r, v)| {
        count + v * traverse(mapping, mapping.get(r).unwrap())
    })
}

impl<'a> Day for Seven<'a, ()> {
    type Input = Graph<'a>;
    type Output = usize;

    fn part_one(graph: &Self::Input) -> Self::Output {
        let mut mem = HashMap::new();
        graph
            .iter()
            .filter(|(node, _)| search(graph, node, &mut mem))
            .count()
    }

    fn part_two(graph: &Self::Input) -> Self::Output {
        traverse(graph, graph.get("shiny gold").unwrap()) - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_seven");

        input.lines().fold(HashMap::new(), |mut graph, line| {
            let mut parse = line.split(" contain ");

            let for_rule = parse.next().unwrap().rsplit_once(' ').unwrap().0;

            graph.insert(for_rule, find_rules(&parse.collect::<Vec<_>>()));

            graph
        })
    }
}
