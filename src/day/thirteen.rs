use crate::Day;

pub struct Thirteen {}

pub struct Schedule {
    timestamp: i64,
    buses: Vec<i64>,
}

fn modular_inverse(a: i64, b: i64) -> i64 {
    let mut t = 0;
    let mut nt = 1;
    let mut r = b;
    let mut nr = a;

    while nr > 0 {
        let quotient = r / nr;
        (t, nt) = (nt, t - quotient * nt);
        (r, nr) = (nr, r - quotient * nr);
    }

    if t < 0 {
        t + b
    } else {
        t
    }
}

fn chinese_remainder(residues: &[i64], moduli: &[i64]) -> i64 {
    let m = moduli.iter().product::<i64>();

    let mi = moduli.iter().map(|mi| m / mi);
    let yi = moduli
        .iter()
        .zip(mi.clone())
        .map(|(&a, b)| modular_inverse(b, a));

    residues
        .iter()
        .zip(mi)
        .zip(yi)
        .map(|((&a, b), c)| a * b * c)
        .sum::<i64>()
        % m
}

impl Day for Thirteen {
    type Input = Schedule;
    type Output = i64;

    fn part_one(schedule: &Self::Input) -> Self::Output {
        let closest = schedule
            .buses
            .iter()
            .filter_map(|bus| {
                if *bus == 0 {
                    None
                } else {
                    Some((bus, ((schedule.timestamp / bus) + 1) * bus))
                }
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        (closest.1 - schedule.timestamp) * closest.0
    }

    fn part_two(schedule: &Self::Input) -> Self::Output {
        let (moduli, residues): (Vec<i64>, Vec<i64>) = schedule
            .buses
            .iter()
            .enumerate()
            .filter_map(|(idx, &bus)| {
                if bus == 0 {
                    None
                } else {
                    Some((bus, bus - idx as i64))
                }
            })
            .unzip();

        chinese_remainder(&residues, &moduli)
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_thirteen");

        let timestamp = input.lines().next().unwrap().trim().parse::<i64>().unwrap();
        let buses = input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|bus| bus.parse::<i64>().unwrap_or(0))
            .collect();

        Schedule { timestamp, buses }
    }
}
