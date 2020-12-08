use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use crate::day::Day;

pub struct Seven<'a, T: 'a> {
    data: PhantomData<&'a T>,
}

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn find_rules<'a>(parsed: &[&'a str]) -> HashMap<&'a str, usize> {
    parsed
        .iter()
        .skip(1)
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

fn reverse<'a>(graph: &Graph<'a>) -> Graph<'a> {
    let mut rev = Graph::new();

    graph.iter().for_each(|(node, edges)| {
        edges.iter().for_each(|(n, w)| {
            rev.entry(*n).or_insert_with(HashMap::new).insert(node, *w);
        })
    });

    rev
}

fn search<'a>(graph: &Graph<'a>) -> usize {
    fn inner<'a>(graph: &Graph<'a>, node: &'a str, mem: &mut HashSet<&'a str>) -> usize {
        if mem.contains(node) {
            0
        } else {
            mem.insert(node);

            if let Some(edges) = graph.get(node) {
                edges
                    .iter()
                    .fold(1, |count, (node, _)| count + inner(graph, node, mem))
            } else {
                1
            }
        }
    }

    inner(&reverse(graph), "shiny gold", &mut HashSet::new()) - 1
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
        search(graph)
    }

    fn part_two(graph: &Self::Input) -> Self::Output {
        traverse(graph, graph.get("shiny gold").unwrap()) - 1
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_seven");

        input.lines().fold(HashMap::new(), |mut graph, line| {
            let parse = line.split(" contain ").collect::<Vec<_>>();

            let for_rule = parse.first().unwrap().rsplit_once(' ').unwrap().0;

            graph.insert(for_rule, find_rules(&parse));

            graph
        })
    }
}
