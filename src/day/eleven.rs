use crate::day::Day;

pub struct Eleven {}

type Life = Vec<Vec<char>>;

static DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn do_swap_part_one(seats: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut neighbours = DIRECTIONS.iter().filter_map(|&(dy, dx)| {
        seats
            .get((i as i64 + dy) as usize)
            .and_then(|row| row.get((j as i64 + dx) as usize))
    });

    match seats.get(i).and_then(|row| row.get(j)) {
        Some('L') => neighbours.all(|&seat| seat != '#'),
        Some('#') => neighbours.filter(|&&seat| seat == '#').count() >= 4,
        _ => false,
    }
}

fn nearest_neighbour_in_direction(
    seats: &[Vec<char>],
    i: usize,
    j: usize,
    (dx, dy): (i64, i64),
) -> Option<char> {
    let (mut i, mut j) = (i as i64, j as i64);

    loop {
        i += dy;
        j += dx;

        match seats.get(i as usize).and_then(|row| row.get(j as usize)) {
            Some('.') => {}
            Some(&c) => return Some(c),
            None => return None,
        }
    }
}

fn do_swap_part_two(seats: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut neighbours = DIRECTIONS
        .iter()
        .filter_map(|&direction| nearest_neighbour_in_direction(seats, i, j, direction));

    match seats.get(i).and_then(|row| row.get(j)) {
        Some('L') => neighbours.all(|seat| seat != '#'),
        Some('#') => neighbours.filter(|&seat| seat == '#').count() >= 5,
        _ => false,
    }
}

fn simulate<F: Fn(&[Vec<char>], usize, usize) -> bool>(seats: &[Vec<char>], do_swap: F) -> usize {
    let mut seats = seats.to_owned();
    let mut occupied_count = 0;

    loop {
        seats = seats
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, seat)| match (seat, do_swap(&seats, i, j)) {
                        (&'L', true) => '#',
                        (&'#', true) => 'L',
                        _ => *seat,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let occupied = seats
            .iter()
            .flat_map(|row| row.iter().filter(|&seat| *seat == '#'))
            .count();

        if occupied_count == occupied {
            break;
        }

        occupied_count = occupied;
    }

    occupied_count
}

impl Day for Eleven {
    type Input = Life;
    type Output = usize;

    fn part_one(seats: &Self::Input) -> Self::Output {
        simulate(&seats, do_swap_part_one)
    }

    fn part_two(seats: &Self::Input) -> Self::Output {
        simulate(&seats, do_swap_part_two)
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_eleven");

        input
            .lines()
            .map(str::trim)
            .map(|line| line.chars().collect())
            .collect()
    }
}
