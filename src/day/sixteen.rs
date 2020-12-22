use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use crate::Day;

pub struct Sixteen {}

pub struct Rule {
    field: String,
    ranges: [RangeInclusive<usize>; 2],
}

impl Rule {
    fn allows(&self, against: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&against))
    }
}

pub struct Notes {
    notes: Vec<Rule>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Day for Sixteen {
    type Input = Notes;
    type Output = usize;

    fn part_one(notes: &Self::Input) -> Self::Output {
        notes
            .nearby_tickets
            .iter()
            .map(|nearby| {
                nearby
                    .iter()
                    .filter(|near| notes.notes.iter().all(|rule| !rule.allows(**near)))
                    .sum::<usize>()
            })
            .sum()
    }

    fn part_two(notes: &Self::Input) -> Self::Output {
        let filtered = notes
            .nearby_tickets
            .iter()
            .filter(|nearby| {
                nearby
                    .iter()
                    .all(|near| notes.notes.iter().any(|rule| rule.allows(*near)))
            })
            .collect::<Vec<_>>();

        let mut unknown_ordering = notes
            .notes
            .iter()
            .map(|rule| {
                (
                    rule.field.clone(),
                    (0..notes.ticket.len())
                        .filter(|idx| filtered.iter().all(|ticket| rule.allows(ticket[*idx])))
                        .collect(),
                )
            })
            .collect::<HashMap<String, HashSet<_>>>();

        let mut known_ordering = HashMap::new();

        while !unknown_ordering.is_empty() {
            let (field, idx) = unknown_ordering
                .iter()
                .filter(|(_, indices)| indices.len() == 1)
                .map(|(field, indices)| (field.clone(), *indices.iter().next().unwrap()))
                .next()
                .unwrap();

            unknown_ordering.remove(&field);
            known_ordering.insert(field, idx);

            for indices in unknown_ordering.values_mut() {
                indices.remove(&idx);
            }
        }

        known_ordering
            .into_iter()
            .filter_map(|(field, idx)| {
                if field.starts_with("departure") {
                    Some(notes.ticket[idx])
                } else {
                    None
                }
            })
            .fold_first(|count, value| count * value)
            .unwrap_or(0)
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_sixteen");

        let notes = input
            .lines()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| line.split_once(": "))
            .map(|line| {
                let mut ranges = line
                    .1
                    .split(" or ")
                    .filter_map(|values| values.split_once('-'))
                    .map(|value| {
                        value.0.parse::<usize>().unwrap()..=value.1.parse::<usize>().unwrap()
                    });

                Rule {
                    field: line.0.to_string(),
                    ranges: [ranges.next().unwrap(), ranges.next().unwrap()],
                }
            })
            .collect();

        let leftover = input.lines().skip_while(|line| !line.is_empty());

        let ticket = leftover
            .clone()
            .take(3)
            .nth(2)
            .map(|line| line.split(','))
            .unwrap()
            .filter_map(|value| value.parse::<usize>().ok())
            .collect();

        let nearby_tickets = leftover
            .skip(5)
            .map(|nearby| {
                nearby
                    .split(',')
                    .filter_map(|value| value.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            })
            .collect();

        Notes {
            notes,
            ticket,
            nearby_tickets,
        }
    }
}
