use super::lib::*;

const START_POSITION: Position = Position {
    x: 0,
    y: 0,
    angle: 90,
};

const WAYPOINT_START_POSITION: Position = Position {
    x: 10,
    y: 1,
    angle: 90,
};

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day12.txt";
    let inputs = read_inputs(&filename);
    let inputs: Vec<Move> = inputs
        .lines()
        .filter_map(|line| Move::from_str(line))
        .collect();

    let part_one_final_position = run_all_moves(&START_POSITION, &inputs);
    let part_one = Some(
        START_POSITION
            .manhattan_distance(&part_one_final_position)
            .to_string(),
    );
    let part_two_final_position =
        run_all_moves_relative_to_waypoint(&START_POSITION, &WAYPOINT_START_POSITION, &inputs);
    let part_two = Some(
        START_POSITION
            .manhattan_distance(&part_two_final_position)
            .to_string(),
    );

    (part_one, part_two)
}

fn run_all_moves(starting_position: &Position, moves: &[Move]) -> Position {
    moves
        .iter()
        .fold(*starting_position, |current_position, current_move| {
            current_position.move_position(current_move)
        })
}

fn run_all_moves_relative_to_waypoint(
    starting_position: &Position,
    waypoint_starting_position: &Position,
    moves: &[Move],
) -> Position {
    moves
        .iter()
        .fold(
            (*starting_position, *waypoint_starting_position),
            |(current_position, current_waypoint_position), current_move| {
                let mut new_position = current_position;
                let mut new_waypoint = current_waypoint_position;

                match current_move.move_type {
                    MoveType::North | MoveType::South | MoveType::East | MoveType::West => {
                        new_waypoint = current_waypoint_position.move_position(&current_move);
                    }
                    MoveType::Right | MoveType::Left => {
                        new_waypoint =
                            current_waypoint_position.rotate_around_origin(&current_move);
                    }
                    MoveType::Forward => {
                        new_position = current_position
                            .move_to_waypoint(&current_move, &current_waypoint_position);
                    }
                };
                (new_position, new_waypoint)
            },
        )
        .0
}

#[derive(Debug, PartialEq)]
struct Move {
    move_type: MoveType,
    units: usize,
}

impl Move {
    pub fn from_str(move_str: &str) -> Option<Self> {
        let move_type = MoveType::from_str(move_str);
        let units = move_str
            .get(1..)
            .map(|units_str| units_str.parse::<usize>().ok())
            .flatten();

        match (move_type, units) {
            (Some(move_type), Some(units)) => Some(Move { move_type, units }),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MoveType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl MoveType {
    pub fn from_str(move_str: &str) -> Option<Self> {
        match move_str.chars().next() {
            Some('N') => Some(MoveType::North),
            Some('S') => Some(MoveType::South),
            Some('E') => Some(MoveType::East),
            Some('W') => Some(MoveType::West),
            Some('L') => Some(MoveType::Left),
            Some('R') => Some(MoveType::Right),
            Some('F') => Some(MoveType::Forward),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    angle: i32,
}

impl Position {
    const ORIGIN: Position = Position {
        x: 0,
        y: 0,
        angle: 0,
    };

    fn manhattan_distance(&self, point: &Position) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }

    fn rotate_around_origin(&self, move_desc: &Move) -> Self {
        let angle = Self::ORIGIN.move_position(move_desc).angle;
        match angle {
            90 => Position {
                x: self.y,
                y: -self.x,
                ..*self
            },
            180 => Position {
                x: -self.x,
                y: -self.y,
                ..*self
            },
            270 => Position {
                x: -self.y,
                y: self.x,
                ..*self
            },
            _ => *self,
        }
    }

    fn move_to_waypoint(&self, move_desc: &Move, waypoint_position: &Position) -> Self {
        let vertical_move = match waypoint_position.y {
            y if y >= 0 => Move {
                move_type: MoveType::North,
                units: move_desc.units * (y as usize),
            },
            y => Move {
                move_type: MoveType::South,
                units: move_desc.units * (-y as usize),
            },
        };
        let horizontal_move = match waypoint_position.x {
            x if x >= 0 => Move {
                move_type: MoveType::East,
                units: move_desc.units * (x as usize),
            },
            x => Move {
                move_type: MoveType::West,
                units: move_desc.units * (-x as usize),
            },
        };

        self.move_position(&vertical_move)
            .move_position(&horizontal_move)
    }

    fn move_type_from_angle(&self) -> Option<MoveType> {
        match self.angle {
            0 => Some(MoveType::North),
            90 => Some(MoveType::East),
            180 => Some(MoveType::South),
            270 => Some(MoveType::West),
            _ => None,
        }
    }

    fn move_position(&self, move_desc: &Move) -> Self {
        match move_desc.move_type {
            MoveType::North => Position {
                y: self.y + move_desc.units as i32,
                ..*self
            },
            MoveType::South => Position {
                y: self.y - move_desc.units as i32,
                ..*self
            },
            MoveType::East => Position {
                x: self.x + move_desc.units as i32,
                ..*self
            },
            MoveType::West => Position {
                x: self.x - move_desc.units as i32,
                ..*self
            },
            MoveType::Left => Position {
                angle: match (self.angle - move_desc.units as i32) % 360 {
                    x if x < 0 => x + 360,
                    x => x,
                },
                ..*self
            },
            MoveType::Right => Position {
                angle: ((self.angle + move_desc.units as i32) % 360).abs(),
                ..*self
            },
            MoveType::Forward => self.move_position(&Move {
                move_type: self.move_type_from_angle().expect("Expected a move type"),
                ..*move_desc
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_move_from_str() {
        assert_eq!(
            Some(Move {
                move_type: MoveType::North,
                units: 12
            }),
            Move::from_str("N12")
        );
        assert_eq!(
            Some(Move {
                move_type: MoveType::South,
                units: 12
            }),
            Move::from_str("S12")
        );
        assert_eq!(None, MoveType::from_str("K12"));
        assert_eq!(None, MoveType::from_str(""));
    }

    #[test]
    fn test_move_type_from_str() {
        assert_eq!(Some(MoveType::North), MoveType::from_str("N12"));
        assert_eq!(Some(MoveType::South), MoveType::from_str("S12"));
        assert_eq!(Some(MoveType::East), MoveType::from_str("E12"));
        assert_eq!(Some(MoveType::West), MoveType::from_str("W12"));
        assert_eq!(Some(MoveType::Left), MoveType::from_str("L12"));
        assert_eq!(Some(MoveType::Right), MoveType::from_str("R12"));
        assert_eq!(Some(MoveType::Forward), MoveType::from_str("F12"));
        assert_eq!(None, MoveType::from_str("K12"));
        assert_eq!(None, MoveType::from_str(""));
    }

    #[test]
    fn test_manhattan_distance() {
        let first_point = Position {
            x: 1,
            y: 2,
            angle: 0,
        };
        let second_point = Position {
            x: 3,
            y: 4,
            angle: 0,
        };
        assert_eq!(4, first_point.manhattan_distance(&second_point));
    }

    #[test]
    fn test_manhattan_distance_negative() {
        let first_point = START_POSITION;
        let second_point = Position {
            x: 158,
            y: -12,
            angle: 0,
        };
        assert_eq!(170, first_point.manhattan_distance(&second_point));
    }

    #[test]
    fn test_move_position() {
        let looking_north = Position {
            angle: 0,
            ..START_POSITION
        };
        let looking_south = Position {
            angle: 180,
            ..START_POSITION
        };
        let looking_west = Position {
            angle: 270,
            ..START_POSITION
        };
        assert_eq!(
            Position {
                y: 10,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::North,
                units: 10
            })
        );
        assert_eq!(
            Position {
                y: -10,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::South,
                units: 10
            })
        );
        assert_eq!(
            Position {
                x: 10,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::East,
                units: 10
            })
        );
        assert_eq!(
            Position {
                x: -10,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::West,
                units: 10
            })
        );
        assert_eq!(
            Position {
                x: 10,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Forward,
                units: 10
            })
        );
        assert_eq!(
            Position {
                y: 10,
                ..looking_north
            },
            looking_north.move_position(&Move {
                move_type: MoveType::Forward,
                units: 10
            })
        );
        assert_eq!(
            Position {
                y: -10,
                ..looking_south
            },
            looking_south.move_position(&Move {
                move_type: MoveType::Forward,
                units: 10
            })
        );
        assert_eq!(
            Position {
                x: -10,
                ..looking_west
            },
            looking_west.move_position(&Move {
                move_type: MoveType::Forward,
                units: 10
            })
        );
        assert_eq!(
            Position {
                angle: 0,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Left,
                units: 90
            })
        );
        assert_eq!(
            Position {
                angle: 270,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Left,
                units: 180
            })
        );
        assert_eq!(
            Position {
                angle: 180,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Right,
                units: 90
            })
        );
        assert_eq!(
            Position {
                angle: 0,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Right,
                units: 90 * 3
            })
        );
        assert_eq!(
            Position {
                angle: 90,
                ..START_POSITION
            },
            START_POSITION.move_position(&Move {
                move_type: MoveType::Right,
                units: 90 * 4
            })
        );
    }

    #[test]
    fn test_run_all_moves() {
        let moves: Vec<Move> = TEST_CASE_1
            .lines()
            .filter_map(|x| Move::from_str(x))
            .collect();
        let final_position = run_all_moves(&START_POSITION, &moves);
        assert_eq!(25, START_POSITION.manhattan_distance(&final_position));
    }

    #[test]
    fn test_run_all_moves_relative_to_waypoint() {
        let moves: Vec<Move> = TEST_CASE_1
            .lines()
            .filter_map(|x| Move::from_str(x))
            .collect();
        let final_position =
            run_all_moves_relative_to_waypoint(&START_POSITION, &WAYPOINT_START_POSITION, &moves);
        assert_eq!(286, START_POSITION.manhattan_distance(&final_position));
    }

    #[test]
    fn test_rotate_around_origin() {
        let starting_position = Position {
            x: 10,
            y: 4,
            angle: 90,
        };
        let right_one = Position {
            x: 4,
            y: -10,
            angle: 90,
        };
        let right_two = Position {
            x: -10,
            y: -4,
            angle: 90,
        };
        let right_three = Position {
            x: -4,
            y: 10,
            angle: 90,
        };

        assert_eq!(
            right_one,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Right,
                units: 90
            })
        );
        assert_eq!(
            right_two,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Right,
                units: 180
            })
        );
        assert_eq!(
            right_three,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Right,
                units: 270
            })
        );
        assert_eq!(
            starting_position,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Right,
                units: 360
            })
        );
        assert_eq!(
            right_one,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Right,
                units: 360 + 90
            })
        );

        assert_eq!(
            right_three,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Left,
                units: 90
            })
        );
        assert_eq!(
            right_two,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Left,
                units: 180
            })
        );
        assert_eq!(
            right_one,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Left,
                units: 270
            })
        );
        assert_eq!(
            starting_position,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Left,
                units: 360
            })
        );
        assert_eq!(
            right_three,
            starting_position.rotate_around_origin(&Move {
                move_type: MoveType::Left,
                units: 360 + 90
            })
        );
    }

    #[test]
    fn test_move_to_waypoint() {
        assert_eq!(
            Position {
                x: 100,
                y: 10,
                ..START_POSITION
            },
            START_POSITION.move_to_waypoint(
                &Move {
                    move_type: MoveType::Forward,
                    units: 10
                },
                &WAYPOINT_START_POSITION
            )
        );
    }
}
