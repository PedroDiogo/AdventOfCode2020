use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day3.txt";
    let inputs = read_inputs(&filename);
    let map: Vec<&str> = inputs.lines().collect();

    let delta = (1, 3);

    let part_one = get_multiplied_slopes(&map, &[&delta]);

    let deltas = vec![&(1, 1), &(1, 3), &(1, 5), &(1, 7), &(2, 1)];
    let part_two = get_multiplied_slopes(&map, &deltas);

    (Some(part_one.to_string()), Some(part_two.to_string()))
}

fn get_multiplied_slopes(map: &[&str], deltas: &[&(usize, usize)]) -> usize {
    let map_size = (map.len(), map.first().unwrap().len());

    deltas.iter().fold(1, |mult, delta| {
        mult * get_number_of_trees(&map, &get_coordinates(&delta, &map_size))
    })
}

fn get_number_of_trees(map: &[&str], coordinates: &[(usize, usize)]) -> usize {
    coordinates
        .iter()
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
    (0..map_size.0)
        .step_by(delta.0)
        .skip(1)
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
    fn test_get_multiplied_slopes() {
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
        let deltas = vec![&(1, 1), &(1, 3), &(1, 5), &(1, 7), &(2, 1)];
        assert_eq!(336, get_multiplied_slopes(&map, &deltas));
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

    #[test]
    fn test_get_coordinates_twice_down() {
        let expected_coordinates = vec![(2, 3), (4, 6), (6, 9), (8, 1), (10, 4)];
        assert_eq!(expected_coordinates, get_coordinates(&(2, 3), &(11, 11)));
    }
}
