use super::lib::*;

use std::collections::HashMap;

extern crate itertools;
use self::itertools::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day17.txt";
    let inputs = read_inputs(&filename);

    let mut grid = Grid::from_str(&inputs, &0);
    for _ in 0..6 {
        grid = grid.next_iteration();
    }

    let part_one = Some(grid.number_of_active_cells().to_string());
    let part_two = None;

    (part_one, part_two)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    map: HashMap<Vec<isize>, PositionType>,
}

impl Grid {
    pub fn new(map: HashMap<Vec<isize>, PositionType>) -> Self {
        Self { map }
    }

    pub fn from_str(grid_str: &str, z: &isize) -> Self {
        let map = grid_str
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| PositionType::from_str(&c).is_some())
                    .map(|(col, c)| {
                        (
                            vec![row as isize, col as isize, *z],
                            PositionType::from_str(&c).unwrap(),
                        )
                    })
                    .collect::<Vec<(Vec<isize>, PositionType)>>()
            })
            .flatten()
            .filter(|(_, position_type)| position_type == &PositionType::ACTIVE)
            .collect();
        Grid::new(map)
    }

    pub fn get(&self, coordinates: &[isize]) -> Option<&PositionType> {
        self.map.get(coordinates).or(Some(&PositionType::INACTIVE))
    }

    pub fn number_of_active_cells(&self) -> usize {
        self.map.len()
    }

    fn neightbours_coordinates(&self, coordinates: &[isize]) -> Vec<Vec<isize>> {
        let positions = [-1, 0, 1];
        positions
            .iter()
            .cartesian_product(positions.iter())
            .cartesian_product(positions.iter())
            .map(|x| vec![*x.0.0, *x.0.1, *x.1])
            .map(|x| {
                vec![
                    x[0] + coordinates[0],
                    x[1] + coordinates[1],
                    x[2] + coordinates[2],
                ]
            })
            .filter(|x| x != &coordinates.to_vec())
            .collect()
    }

    fn active_neighbours(&self, coordinates: &[isize]) -> usize {
        self.neightbours_coordinates(coordinates)
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
                let mut neighbours = self.neightbours_coordinates(x);
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

        Grid::new(map)
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
        let grid = Grid::from_str(TEST_CASE_1, &0);

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
        let grid = Grid::from_str(TEST_CASE_1, &0);
        assert_eq!(5, grid.active_neighbours(&[1, 1, 1]));
        assert_eq!(1, grid.active_neighbours(&[2, 0, 0]));
    }

    #[test]
    fn test_grid_next_position_for_coordinate() {
        let grid = Grid::from_str(TEST_CASE_1, &0);
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
        let mut grid = Grid::from_str(TEST_CASE_1, &0);
        for _ in 1..=6 {
            grid = grid.next_iteration();
        }

        assert_eq!(112, grid.number_of_active_cells());
    }
}
