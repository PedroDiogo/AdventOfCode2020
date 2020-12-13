use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day11.txt";
    let inputs = read_inputs(&filename);

    let initial_grid = Grid::from_str(&inputs);
    let final_grid = initial_grid.run_until_stable();
    let part_one = Some(final_grid.number_of_occupied_positions().to_string());
    let part_two = None;

    (part_one, part_two)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    map: Vec<Vec<PositionType>>,
}

impl Grid {
    fn get(&self, row: &usize, col: &usize) -> Option<&PositionType> {
        match self.map.get(*row) {
            Some(r) => r.get(*col),
            _ => None,
        }
    }

    fn adjacent_occupied_seats(&self, row: &usize, col: &usize) -> usize {
        let positions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        positions
            .iter()
            .map(|position| (*row as i64 + position.0, *col as i64 + position.1))
            .filter(|position| position.0 >= 0 && position.1 >= 0)
            .filter_map(|position| self.get(&(position.0 as usize), &(position.1 as usize)))
            .filter(|position_type| **position_type == PositionType::OCCUPIED)
            .count()
    }

    fn number_of_occupied_positions(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|position| **position == PositionType::OCCUPIED)
            .count()
    }

    fn run_iteration(&self) -> Self {
        Grid {
            map: self
                .map
                .clone()
                .iter()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(col_idx, element)| {
                            match (element, self.adjacent_occupied_seats(&row_idx, &col_idx)) {
                                (PositionType::EMPTY, 0) => PositionType::OCCUPIED,
                                (PositionType::OCCUPIED, occupied) if occupied >= 4 => {
                                    PositionType::EMPTY
                                }
                                _ => *element,
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn run_until_stable(&self) -> Self {
        let mut current_iteration = self.clone();
        loop {
            let next_iteration = current_iteration.run_iteration();
            if current_iteration == next_iteration {
                break next_iteration;
            }
            current_iteration = next_iteration;
        }
    }

    fn from_str(grid_str: &str) -> Self {
        let map = grid_str
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| PositionType::from_str(&c))
                    .collect()
            })
            .collect();
        Grid { map }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PositionType {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

impl PositionType {
    fn from_str(position_str: &char) -> Option<PositionType> {
        match position_str {
            'L' => Some(PositionType::EMPTY),
            '#' => Some(PositionType::OCCUPIED),
            '.' => Some(PositionType::FLOOR),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #L#
    // .#.
    // ##.
    const SMALL_MAP_STR: &str = "#L#
.#.
##.";
    const SMALL_MAP: [[PositionType; 3]; 3] = [
        [
            PositionType::OCCUPIED,
            PositionType::EMPTY,
            PositionType::OCCUPIED,
        ],
        [
            PositionType::FLOOR,
            PositionType::OCCUPIED,
            PositionType::FLOOR,
        ],
        [
            PositionType::OCCUPIED,
            PositionType::OCCUPIED,
            PositionType::FLOOR,
        ],
    ];

    const TEST_CASE_1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_position_type_from_str() {
        assert_eq!(Some(PositionType::EMPTY), PositionType::from_str(&'L'));
        assert_eq!(Some(PositionType::FLOOR), PositionType::from_str(&'.'));
        assert_eq!(Some(PositionType::OCCUPIED), PositionType::from_str(&'#'));
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid {
            map: SMALL_MAP.to_vec().iter().map(|row| row.to_vec()).collect(),
        };
        assert_eq!(Some(&PositionType::EMPTY), grid.get(&0, &1));
        assert_eq!(Some(&PositionType::OCCUPIED), grid.get(&1, &1));
        assert_eq!(Some(&PositionType::FLOOR), grid.get(&2, &2));
        assert_eq!(None, grid.get(&1, &3));
        assert_eq!(None, grid.get(&3, &1));
        assert_eq!(None, grid.get(&3, &3));
    }

    #[test]
    fn test_number_of_adjacent_occupied_seats() {
        let grid = Grid {
            map: SMALL_MAP.to_vec().iter().map(|row| row.to_vec()).collect(),
        };

        assert_eq!(1, grid.adjacent_occupied_seats(&0, &0));
        assert_eq!(4, grid.adjacent_occupied_seats(&1, &1));
        assert_eq!(2, grid.adjacent_occupied_seats(&2, &1));
    }

    #[test]
    fn test_grid_from_str() {
        let grid = Grid {
            map: SMALL_MAP.to_vec().iter().map(|row| row.to_vec()).collect(),
        };
        let expected_grid = Grid::from_str(&SMALL_MAP_STR);

        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn test_run_iteration() {
        let grid = Grid {
            map: SMALL_MAP.to_vec().iter().map(|row| row.to_vec()).collect(),
        };
        let expected_grid = Grid {
            map: vec![
                vec![
                    PositionType::OCCUPIED,
                    PositionType::EMPTY,
                    PositionType::OCCUPIED,
                ],
                vec![
                    PositionType::FLOOR,
                    PositionType::EMPTY,
                    PositionType::FLOOR,
                ],
                vec![
                    PositionType::OCCUPIED,
                    PositionType::OCCUPIED,
                    PositionType::FLOOR,
                ],
            ],
        };

        assert_eq!(expected_grid, grid.run_iteration());
    }

    #[test]
    fn test_run_until_stable() {
        let expected_map = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";
        let expected_grid = Grid::from_str(expected_map);
        let grid = Grid::from_str(&TEST_CASE_1);
        let grid = grid.run_until_stable();
        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn test_number_of_occupied_positions() {
        let expected_map = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";
        let expected_grid = Grid::from_str(expected_map);
        assert_eq!(37, expected_grid.number_of_occupied_positions())
    }
}
