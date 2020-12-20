use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day11.txt";
    let inputs = read_inputs(&filename);

    let initial_grid = Grid::from_str(&inputs);
    let part_one_grid = initial_grid.run_until_stable(Grid::get_next_position);
    let part_one = Some(part_one_grid.number_of_occupied_positions().to_string());
    let part_two_grid = initial_grid.run_until_stable(Grid::get_next_position_advanced);
    let part_two = Some(part_two_grid.number_of_occupied_positions().to_string());

    (part_one, part_two)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    map: Vec<Vec<PositionType>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

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

    fn viewed_occupied_seats(&self, row: &usize, col: &usize) -> usize {
        let mut positions: Vec<Vec<(usize, usize)>> = Vec::new();

        positions.push(
            (0..*row)
                .rev()
                .map(|position_row| (position_row, *col))
                .collect(),
        );
        positions.push(
            (row + 1..self.height())
                .map(|position_row| (position_row, *col))
                .collect(),
        );
        positions.push(
            (0..*col)
                .rev()
                .map(|position_col| (*row, position_col))
                .collect(),
        );
        positions.push(
            (col + 1..self.width())
                .map(|position_col| (*row, position_col))
                .collect(),
        );
        positions.push(
            (1..)
                .take_while(|x| row + x <= self.width() && col + x <= self.height())
                .map(|i| (*row + i, *col + i))
                .collect(),
        );
        positions.push(
            (1..)
                .take_while(|x| row >= x && col >= x)
                .map(|i| (*row - i, *col - i))
                .collect(),
        );
        positions.push(
            (1..)
                .take_while(|x| row >= x && col + x <= self.height())
                .map(|i| (*row - i, *col + i))
                .collect(),
        );
        positions.push(
            (1..)
                .take_while(|x| row + x <= self.width() && col >= x)
                .map(|i| (*row + i, *col - i))
                .collect(),
        );
        positions
            .iter()
            .filter_map(|positions| {
                let found_position = positions
                    .iter()
                    .filter_map(|position| self.get(&position.0, &position.1))
                    .find(|position_type| **position_type != PositionType::FLOOR);

                match found_position {
                    Some(PositionType::OCCUPIED) => Some(PositionType::OCCUPIED),
                    _ => None,
                }
            })
            .count()
    }

    fn number_of_occupied_positions(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|position| **position == PositionType::OCCUPIED)
            .count()
    }

    fn run_iteration(
        &self,
        new_position_function: fn(&Self, usize, usize, PositionType) -> PositionType,
    ) -> Self {
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
                            new_position_function(self, row_idx, col_idx, *element)
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn run_until_stable(
        &self,
        new_position_function: fn(&Self, usize, usize, PositionType) -> PositionType,
    ) -> Self {
        let mut current_iteration = self.clone();
        loop {
            let next_iteration = current_iteration.run_iteration(new_position_function);
            if current_iteration == next_iteration {
                break next_iteration;
            }
            current_iteration = next_iteration;
        }
    }

    fn get_next_position(
        grid: &Self,
        row_idx: usize,
        col_idx: usize,
        current_position: PositionType,
    ) -> PositionType {
        match (
            current_position,
            grid.adjacent_occupied_seats(&row_idx, &col_idx),
        ) {
            (PositionType::EMPTY, 0) => PositionType::OCCUPIED,
            (PositionType::OCCUPIED, occupied) if occupied >= 4 => PositionType::EMPTY,
            _ => current_position,
        }
    }

    fn get_next_position_advanced(
        grid: &Self,
        row_idx: usize,
        col_idx: usize,
        current_position: PositionType,
    ) -> PositionType {
        match (
            current_position,
            grid.viewed_occupied_seats(&row_idx, &col_idx),
        ) {
            (PositionType::EMPTY, 0) => PositionType::OCCUPIED,
            (PositionType::OCCUPIED, occupied) if occupied >= 5 => PositionType::EMPTY,
            _ => current_position,
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
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|position| position.to_char())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", grid)
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

    fn to_char(&self) -> char {
        match self {
            PositionType::EMPTY => 'L',
            PositionType::OCCUPIED => '#',
            PositionType::FLOOR => '.',
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
    fn test_grid_dimensions() {
        let map = ".............
.L.L.#.#.#.#.
.............";
        let grid = Grid::from_str(&map);
        assert_eq!(13, grid.width());
        assert_eq!(3, grid.height());
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
    fn test_view_occupied_seats() {
        let map_1 = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
        let map_2 = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let map_3 = ".............
.L.L.#.#.#.#.
.............";

        let map_4 = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

        assert_eq!(8, Grid::from_str(&map_1).viewed_occupied_seats(&4, &3));
        assert_eq!(0, Grid::from_str(&map_2).viewed_occupied_seats(&3, &3));
        assert_eq!(0, Grid::from_str(&map_3).viewed_occupied_seats(&1, &1));
        assert_eq!(1, Grid::from_str(&map_3).viewed_occupied_seats(&1, &3));
        assert_eq!(0, Grid::from_str(&map_4).viewed_occupied_seats(&0, &3));
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

        assert_eq!(expected_grid, grid.run_iteration(Grid::get_next_position));
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
        let grid = grid.run_until_stable(Grid::get_next_position);
        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn test_run_until_stable_advanced() {
        let expected_map = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";
        let expected_grid = Grid::from_str(expected_map);
        let grid = Grid::from_str(&TEST_CASE_1);
        let grid = grid.run_until_stable(Grid::get_next_position_advanced);
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
