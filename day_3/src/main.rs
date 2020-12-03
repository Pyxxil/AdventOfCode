
struct Point {
    x: usize,
    y: usize,
}

impl Point {
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

fn solve(input: &str, dx: usize, dy: usize) -> usize {
    let map = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = map[0].len();

    let mut pos = Point::new(0, 0);

    let mut count = 0;

    while pos.y() < map.len() {
        if map[pos.y()][pos.x()] == '#' {
            count += 1;
        }
        
        pos.slide(dx, dy, width);
    }

    count
}


fn main() {
    let input = include_str!("input");
    let mut count = solve(input, 3, 1);
    println!("{}", count);

    for (dx, dy) in vec![(1usize, 1usize), (5, 1), (7, 1), (1, 2)].into_iter() {
        count *= solve(input, dx, dy);
    }

    println!("{}", count);
}
