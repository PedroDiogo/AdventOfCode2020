extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day3.txt";
    let inputs = read_inputs(&filename);
    let map: Vec<&str> = inputs.split("\n").collect();

    let map_size = (map.len(), map.first().unwrap().len());
    let delta = (1, 3);

    let part_one = get_number_of_trees(&map, &get_coordinates(&delta, &map_size));
    println!("Part one: {}", part_one);
}

fn get_number_of_trees(map: &Vec<&str>, coordinates: &Vec<(usize, usize)>) -> usize {
    coordinates
        .into_iter()
        .filter(|coordinate| {
            map.get(coordinate.0)
                .unwrap()
                .chars()
                .nth(coordinate.1)
                .unwrap()
                == '#'
        })
        .count()
}

fn get_coordinates(delta: &(usize, usize), map_size: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut col = 0;
    (1..map_size.0)
        .step_by(delta.0)
        .map(|row| {
            col = (col + delta.1) % map_size.1;
            (row, col)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_trees() {
        let map = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        assert_eq!(
            7,
            get_number_of_trees(&map, &get_coordinates(&(1, 3), &(11, 11)))
        );
    }

    #[test]
    fn test_get_coordinates() {
        let expected_coordinates = vec![
            (1, 3),
            (2, 6),
            (3, 9),
            (4, 1),
            (5, 4),
            (6, 7),
            (7, 10),
            (8, 2),
            (9, 5),
            (10, 8),
        ];
        assert_eq!(expected_coordinates, get_coordinates(&(1, 3), &(11, 11)));
    }
}
