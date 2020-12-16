use crate::Day;

pub struct Twelve {}

#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance_from(self, from: &Position) -> i32 {
        (self.x - from.x).abs() + (self.y - from.y).abs()
    }

    fn moved(self, (dx, dy): (i32, i32)) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotated(self, degrees: i32) -> Self {
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
                .expect("An Action requires a number as an operand"),
        ) {
            ('N', value) => Self::North(value),
            ('S', value) => Self::South(-value),
            ('E', value) => Self::East(value),
            ('W', value) => Self::West(-value),
            ('R', value) => Self::Right(value),
            ('L', value) => Self::Left(360 - value),
            ('F', value) => Self::Forward(value),
            _ => unreachable!(),
        }
    }
}

struct Waypoint {
    position: Position,
}

impl Waypoint {
    fn at(position: Position) -> Self {
        Self { position }
    }

    fn rotated(self, degrees: i32) -> Self {
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

    fn moved(self, (dx, dy): (i32, i32)) -> Self {
        Self {
            position: self.position.moved((dx, dy)),
        }
    }
}

impl Day for Twelve {
    type Input = Vec<Action>;
    type Output = i32;

    fn part_one(actions: &Self::Input) -> Self::Output {
        actions
            .iter()
            .fold(
                (Position::default(), Direction::East),
                |position, action|
                    match action {
                    Action::North(value) | Action::South(value) => {
                        (position.0.moved((0, *value)), position.1)
                    }
                    Action::East(value) | Action::West(value) => {
                        (position.0.moved((*value, 0)), position.1)
                    }
                    Action::Right(degrees) | Action::Left(degrees) => {
                        (position.0, position.1.rotated(*degrees))
                    }
                    Action::Forward(value) => (
                        position.0.moved(match position.1 {
                            Direction::North => (0, *value),
                            Direction::South => (0, -*value),
                            Direction::East => (*value, 0),
                            Direction::West => (-*value, 0)
                        }),
                        position.1,
                    ),
                }
            )
            .0
            .distance_from(&Position::default())
    }

    fn part_two(actions: &Self::Input) -> Self::Output {
        actions
            .iter()
            .fold(
                (Position::default(), Waypoint::at(Position { x: 10, y: 1 })),
                |position, action| match action {
                    Action::North(value) | Action::South(value) => {
                        (position.0, position.1.moved((0, *value)))
                    }
                    Action::East(value) | Action::West(value) => {
                        (position.0, position.1.moved((*value, 0)))
                    }
                    Action::Right(degrees) | Action::Left(degrees) => {
                        (position.0, position.1.rotated(*degrees))
                    }
                    Action::Forward(value) => (
                        position.0.moved((
                            *value * position.1.position.x,
                            *value * position.1.position.y,
                        )),
                        position.1,
                    ),
                },
            )
            .0
            .distance_from(&Position::default())
    }

    fn get_input() -> Self::Input {
        let input = include_str!("input/day_twelve");

        input.lines().map(str::trim).map(Action::from).collect()
    }
}
