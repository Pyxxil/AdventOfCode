use std::collections::HashMap;
use std::marker::PhantomData;

use crate::day::Day;

pub struct Seven<'a, T: 'a> {
    data: PhantomData<&'a T>,
}

type Edges<'a> = HashMap<&'a str, usize>;
type Graph<'a> = HashMap<&'a str, Edges<'a>>;

fn parse_edges(parsed: &str) -> Edges {
    parsed
        .split(',')
        .map(|rule| rule.trim().split_once(' ').unwrap())
        .filter_map(|rule| {
            if rule.0 == "no" {
                None
            } else {
                Some((
                    rule.1.rsplit_once(' ').unwrap().0,
                    rule.0.parse::<usize>().unwrap(),
                ))
            }
        })
        .collect()
}

fn search<'a>(graph: &Graph<'a>, node: &'a str, mem: &mut HashMap<&'a str, bool>) -> bool {
    if !mem.contains_key(node) {
        let reaches = graph
            .get(node)
            .unwrap()
            .keys()
            .any(|node| *node == "shiny gold" || search(graph, node, mem));

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

        input
            .lines()
            .map(|line| line.split_once(" contain ").unwrap())
            .map(|(node, edges)| (node.rsplit_once(' ').unwrap().0, parse_edges(edges)))
            .collect()
    }
}
