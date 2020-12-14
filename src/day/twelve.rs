use crate::day::Day;

pub struct Twelve {}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance_from(self, from: &Position) -> usize {
        ((self.x - from.x).abs() + (self.y - from.y).abs()) as usize
    }

    fn moved(self, direction: Direction, steps: i32) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y + steps,
            },
            Direction::East => Self {
                x: self.x + steps,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - steps,
            },
            Direction::West => Self {
                x: self.x - steps,
                y: self.y,
            },
        }
    }

    fn moved2(self, waypoint: &Waypoint, steps: i32) -> Self {
        Self {
            x: self.x + steps * waypoint.position.x,
            y: self.y + steps * waypoint.position.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(self, degrees: i32) -> Self {
        match self {
            Self::North => match degrees {
                90 => Self::East,
                180 => Self::South,
                270 => Self::West,
                _ => self,
            },
            Self::East => match degrees {
                90 => Self::South,
                180 => Self::West,
                270 => Self::North,
                _ => self,
            },
            Self::South => match degrees {
                90 => Self::West,
                180 => Self::North,
                270 => Self::East,
                _ => self,
            },
            Self::West => match degrees {
                90 => Self::North,
                180 => Self::East,
                270 => Self::South,
                _ => self,
            },
        }
    }
}

#[derive(Debug)]
pub enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl From<&str> for Action {
    fn from(st: &str) -> Self {
        match (
            st.chars().next().unwrap(),
            st.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        ) {
            ('N', value) => Self::North(value),
            ('S', value) => Self::South(value),
            ('E', value) => Self::East(value),
            ('W', value) => Self::West(value),
            ('R', value) => Self::Right(value),
            ('L', value) => Self::Left(value),
            ('F', value) => Self::Forward(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    position: Position,
}

impl Waypoint {
    fn rotate(self, degrees: i32) -> Self {
        match degrees {
            90 => Self {
                position: Position {
                    x: self.position.y,
                    y: -self.position.x,
                },
            },
            180 => Self {
                position: Position {
                    x: -self.position.x,
                    y: -self.position.y,
                },
            },
            270 => Self {
                position: Position {
                    x: -self.position.y,
                    y: self.position.x,
                },
            },
            _ => self,
        }
    }

    fn moved(self, direction: Direction, steps: i32) -> Self {
        Self {
            position: self.position.moved(direction, steps),
        }
    }
}

impl Day for Twelve {
    type Input = Vec<Action>;
    type Output = usize;

    fn part_one(actions: &Self::Input) -> Self::Output {
        actions
            .iter()
            .fold(
                (Position { x: 0, y: 0 }, Direction::East),
                |position, action| match action {
                    Action::North(value) => {
                        (position.0.moved(Direction::North, *value), position.1)
                    }
                    Action::South(value) => {
                        (position.0.moved(Direction::South, *value), position.1)
                    }
                    Action::East(value) => (position.0.moved(Direction::East, *value), position.1),
                    Action::West(value) => (position.0.moved(Direction::West, *value), position.1),
                    Action::Right(degrees) => (position.0, position.1.rotate(*degrees)),
                    Action::Left(degrees) => (position.0, position.1.rotate(360 - *degrees)),
                    Action::Forward(value) => (position.0.moved(position.1, *value), position.1),
                },
            )
            .0
            .distance_from(&Position { x: 0, y: 0 })
    }

    fn part_two(actions: &Self::Input) -> Self::Output {
        actions
            .iter()
            .fold(
                (
                    Position { x: 0, y: 0 },
                    Waypoint {
                        position: Position { x: 10, y: 1 },
                    },
                    Direction::East,
                ),
                |position, action| {
                    match action {
                        Action::North(value) => (
                            position.0,
                            position.1.moved(Direction::North, *value),
                            position.2,
                        ),
                        Action::South(value) => (
                            position.0,
                            position.1.moved(Direction::South, *value),
                            position.2,
                        ),
                        Action::East(value) => (
                            position.0,
                            position.1.moved(Direction::East, *value),
                            position.2,
                        ),
                        Action::West(value) => (
                            position.0,
                            position.1.moved(Direction::West, *value),
                            position.2,
                        ),
                        Action::Right(degrees) => {
                            (position.0, position.1.rotate(*degrees), position.2)
                        }
                        Action::Left(degrees) => {
                            (position.0, position.1.rotate(360 - *degrees), position.2)
                        }
                        Action::Forward(value) => (
                            position.0.moved2(&position.1, *value),
                            position.1,
                            position.2,
                        ),
                    }
                },
            )
            .0
            .distance_from(&Position { x: 0, y: 0 })
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_twelve");

        input.lines().map(str::trim).map(Action::from).collect()
    }
}
