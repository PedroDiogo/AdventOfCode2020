use super::lib::*;

use std::collections::HashMap;

extern crate itertools;
use self::itertools::*;

lazy_static! {
    static ref DELTA_COORDINATES: HashMap<usize, Vec<Vec<isize>>> = {
        let mut deltas = HashMap::new();
        deltas.insert(3, calculate_delta_coordinates(&3));
        deltas.insert(4, calculate_delta_coordinates(&4));
        deltas
    };
}

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day17.txt";
    let inputs = read_inputs(&filename);

    let grid_3d = run_n_times(&Grid::from_str(&inputs, &3), &6);
    let part_one = Some(grid_3d.number_of_active_cells().to_string());

    let grid_4d = run_n_times(&Grid::from_str(&inputs, &4), &6);
    let part_two = Some(grid_4d.number_of_active_cells().to_string());

    (part_one, part_two)
}

fn run_n_times(grid: &Grid, n: &usize) -> Grid {
    let mut grid = grid.clone();
    for _ in 0..*n {
        grid = grid.next_iteration();
    }

    grid
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    map: HashMap<Vec<isize>, PositionType>,
    dimensions: usize,
}

impl Grid {
    pub fn new(map: HashMap<Vec<isize>, PositionType>, dimensions: usize) -> Self {
        Self { map, dimensions }
    }

    pub fn from_str(grid_str: &str, dimensions: &usize) -> Self {
        let map = grid_str
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| PositionType::from_str(&c).is_some())
                    .map(|(col, c)| {
                        let mut coordinate = vec![row as isize, col as isize];
                        coordinate.resize(*dimensions, 0);

                        (coordinate, PositionType::from_str(&c).unwrap())
                    })
                    .collect::<Vec<(Vec<isize>, PositionType)>>()
            })
            .flatten()
            .filter(|(_, position_type)| position_type == &PositionType::ACTIVE)
            .collect();
        Grid::new(map, *dimensions)
    }

    pub fn get(&self, coordinates: &[isize]) -> Option<&PositionType> {
        self.map.get(coordinates).or(Some(&PositionType::INACTIVE))
    }

    pub fn number_of_active_cells(&self) -> usize {
        self.map.len()
    }

    fn neighbours_coordinates(&self, coordinates: &[isize]) -> Vec<Vec<isize>> {
        let dimensions = coordinates.len();
        let deltas = DELTA_COORDINATES.get(&dimensions).unwrap();

        deltas
            .iter()
            .map(|delta| add_position_by_position(delta, &coordinates))
            .collect()
    }

    fn active_neighbours(&self, coordinates: &[isize]) -> usize {
        self.neighbours_coordinates(coordinates)
            .iter()
            .filter_map(|position| self.get(&position))
            .filter(|x| **x == PositionType::ACTIVE)
            .count()
    }

    pub fn next_iteration(&self) -> Self {
        let coordinates = self
            .map
            .keys()
            .map(|x| {
                let mut neighbours = self.neighbours_coordinates(x);
                neighbours.push(x.clone());
                neighbours
            })
            .flatten();

        let map = coordinates
            .map(|coordinate| {
                (
                    coordinate.clone(),
                    self.next_position_for_coordinate(&coordinate),
                )
            })
            .filter(|(_, position_type)| position_type == &PositionType::ACTIVE)
            .collect();

        Grid::new(map, self.dimensions)
    }

    fn next_position_for_coordinate(&self, coordinates: &[isize]) -> PositionType {
        match (self.get(coordinates), self.active_neighbours(coordinates)) {
            (Some(PositionType::ACTIVE), x) if x == 2 || x == 3 => PositionType::ACTIVE,
            (Some(PositionType::INACTIVE), 3) => PositionType::ACTIVE,
            _ => PositionType::INACTIVE,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PositionType {
    ACTIVE,
    INACTIVE,
}

impl PositionType {
    fn from_str(position_str: &char) -> Option<PositionType> {
        match position_str {
            '#' => Some(PositionType::ACTIVE),
            '.' => Some(PositionType::INACTIVE),
            _ => None,
        }
    }
}

#[inline]
fn add_position_by_position(a: &[isize], b: &[isize]) -> Vec<isize> {
    a.iter()
        .zip(b.iter())
        .map(|(left, right)| left + right)
        .collect()
}

fn calculate_delta_coordinates(dimension: &usize) -> Vec<Vec<isize>> {
    let positions = [-1, 0, 1];

    let mut deltas: Vec<Vec<isize>> = positions
        .iter()
        .cartesian_product(positions.iter())
        .map(|(left, right)| vec![*left, *right])
        .collect();

    for _ in 2..*dimension {
        deltas = deltas
            .iter()
            .cartesian_product(positions.iter())
            .map(|(left, right)| {
                let mut left = left.clone();
                left.push(*right);
                left
            })
            .collect();
    }

    deltas
        .iter()
        .cloned()
        .filter(|x| x != &vec![0, 0, 0] && x != &vec![0, 0, 0, 0])
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = ".#.
..#
###";

    #[test]
    fn test_position_type_from_str() {
        assert_eq!(Some(PositionType::ACTIVE), PositionType::from_str(&'#'));
        assert_eq!(Some(PositionType::INACTIVE), PositionType::from_str(&'.'));
        assert_eq!(None, PositionType::from_str(&'-'));
    }

    #[test]
    fn test_grid_from_str() {
        let expected = [
            [
                PositionType::INACTIVE,
                PositionType::ACTIVE,
                PositionType::INACTIVE,
            ],
            [
                PositionType::INACTIVE,
                PositionType::INACTIVE,
                PositionType::ACTIVE,
            ],
            [
                PositionType::ACTIVE,
                PositionType::ACTIVE,
                PositionType::ACTIVE,
            ],
        ];
        let grid = Grid::from_str(TEST_CASE_1, &3);

        for (row_idx, row) in expected.iter().enumerate() {
            for (col_idx, position) in row.iter().enumerate() {
                assert_eq!(
                    Some(position),
                    grid.get(&[row_idx as isize, col_idx as isize, 0])
                );
            }
        }
    }

    #[test]
    fn test_grid_active_neighbours() {
        let grid = Grid::from_str(TEST_CASE_1, &3);
        assert_eq!(5, grid.active_neighbours(&[1, 1, 1]));
        assert_eq!(1, grid.active_neighbours(&[2, 0, 0]));
    }

    #[test]
    fn test_grid_neighbours_coordinates() {
        let grid = Grid::from_str(TEST_CASE_1, &3);
        assert_eq!(26, grid.neighbours_coordinates(&[5, 5, 5]).len());
        let grid = Grid::from_str(TEST_CASE_1, &4);
        assert_eq!(80, grid.neighbours_coordinates(&[5, 5, 5, 5]).len());
    }

    #[test]
    fn test_grid_next_position_for_coordinate() {
        let grid = Grid::from_str(TEST_CASE_1, &3);
        assert_eq!(
            PositionType::ACTIVE,
            grid.next_position_for_coordinate(&[1, 0, -1])
        );
        assert_eq!(
            PositionType::INACTIVE,
            grid.next_position_for_coordinate(&[2, 1, -1])
        );
        assert_eq!(
            PositionType::INACTIVE,
            grid.next_position_for_coordinate(&[2, 0, 0])
        );
    }

    #[test]
    fn test_grid_next_iteration() {
        let mut grid = Grid::from_str(TEST_CASE_1, &3);
        for _ in 1..=6 {
            grid = grid.next_iteration();
        }

        assert_eq!(112, grid.number_of_active_cells());
    }
}
