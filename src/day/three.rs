use crate::day::Day;

struct Toboggan {
    x: usize,
    y: usize,
}

impl Toboggan {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn slide(&mut self, dx: usize, dy: usize, width: usize) {
        self.x = (self.x + dx) % width;
        self.y += dy;
    }
}

pub struct Three {}

impl Three {
    fn solve(map: &[Vec<char>], dx: usize, dy: usize) -> usize {
        let width = map[0].len();

        let mut toboggan = Toboggan::new(0, 0);
        let mut count = 0;

        while toboggan.y() < map.len() {
            if map[toboggan.y()][toboggan.x()] == '#' {
                count += 1;
            }

            toboggan.slide(dx, dy, width);
        }

        count
    }
}

impl Day for Three {
    type Input = Vec<Vec<char>>;
    type Output = usize;

    fn part_one(input: &Self::Input) -> Self::Output {
        Three::solve(input, 3, 1)
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let product = vec![(1_usize, 1_usize), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .fold(Three::solve(input, 3, 1), |prod, (dx, dy)| {
                prod * Three::solve(input, dx, dy)
            });

        product
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_three");
        let map = input
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        map
    }

    fn print_results(one: Self::Output, two: Self::Output) {
        println!("Answer for Part One: {}", one);
        println!("Answer for Part Two: {}", two);
    }
}
