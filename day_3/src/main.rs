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

fn main() {
    let input = include_str!("input");
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let count = solve(&map, 3, 1);
    println!("Answer for Part One: {}", count);

    let product = vec![(1_usize, 1_usize), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .fold(count, |prod, (dx, dy)| prod * solve(&map, dx, dy));

    println!("Answer for Part Two: {}", product);
}
