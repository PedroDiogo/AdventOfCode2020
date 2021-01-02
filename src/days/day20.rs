use super::lib::*;

use std::collections::HashMap;

extern crate itertools;
use self::itertools::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day20.txt";
    let inputs = read_inputs(&filename);
    let tiles = parse_tiles(&inputs);

    let corner_tiles = find_corner_tiles(&tiles);
    let part_one = Some(corner_tiles.iter().product::<usize>().to_string());
    let part_two = None;

    (part_one, part_two)
}

fn parse_tiles(inputs: &str) -> HashMap<usize, String> {
    inputs
        .split_by_blank_lines()
        .map(|tile| {
            let mut tile_lines = tile.lines();
            let tile_number = tile_lines
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_end_matches(':')
                .parse::<usize>()
                .unwrap();
            let tile: String = tile_lines
                .map(|line| format!("{}\n", line))
                .collect::<String>()
                .trim()
                .to_string();
            (tile_number, tile)
        })
        .collect()
}

fn find_corner_tiles(tiles: &HashMap<usize, String>) -> Vec<usize> {
    let tiles_sides: HashMap<usize, Vec<String>> = tiles
        .iter()
        .map(|(tile_number, tile)| (*tile_number, all_tile_sides_combinations(tile)))
        .collect();
    let side_to_tiles = convert_to_side_tiles_hashmap(&tiles_sides);

    let number_of_unique_sides_by_tile = side_to_tiles
        .iter()
        .filter(|(_, tiles)| tiles.len() == 1)
        .map(|(_, tiles)| tiles[0])
        .fold(HashMap::new(), |mut hashmap, tile| {
            let tile_entry = hashmap.entry(tile).or_insert(0);
            *tile_entry += 1;
            hashmap
        });

    number_of_unique_sides_by_tile
        .into_iter()
        .filter(|(_, number_of_shared_sides)| number_of_shared_sides == &4)
        .map(|(tile, _)| tile)
        .sorted()
        .collect()
}

fn all_tile_sides_combinations(tile: &str) -> Vec<String> {
    let tile: Vec<Vec<char>> = tile
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let tile_side_len = &tile[0].len();

    let top_side: String = tile.get(0).unwrap().iter().collect();
    let right_side: String = tile.iter().map(|x| x[tile_side_len - 1]).collect();
    let bottom_side: String = tile.get(tile_side_len - 1).unwrap().iter().collect();
    let left_side: String = tile.iter().map(|x| x[0]).collect();

    let all_sides = vec![top_side, right_side, bottom_side, left_side];
    all_sides
        .into_iter()
        .map(|side| vec![side.clone(), side.chars().rev().collect()])
        .flatten()
        .collect()
}

fn convert_to_side_tiles_hashmap(
    tiles: &HashMap<usize, Vec<String>>,
) -> HashMap<String, Vec<usize>> {
    tiles
        .iter()
        .fold(HashMap::new(), |mut hashmap, (tile_number, tile_sides)| {
            tile_sides.iter().for_each(|side| {
                let tile_numbers = hashmap.entry(side.clone()).or_insert_with(Vec::new);
                tile_numbers.push(*tile_number);
            });
            hashmap
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    const TILE_2311: &str = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

    const TILE_1951: &str = "#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..";

    #[test]
    fn test_parse_tiles() {
        let parsed_tiles = parse_tiles(TEST_CASE_1);
        assert_eq!(9, parsed_tiles.len());
        assert_eq!(TILE_2311, parsed_tiles.get(&2311).unwrap());
        assert_eq!(TILE_1951, parsed_tiles.get(&1951).unwrap());
    }

    #[test]
    fn test_find_corner_tiles() {
        let tiles = parse_tiles(TEST_CASE_1);
        assert_eq!(vec![1171, 1951, 2971, 3079], find_corner_tiles(&tiles));
    }

    #[test]
    fn test_all_tile_sides_combinations() {
        let expected_sides = vec![
            "..##.#..#.",
            ".#..#.##..",
            "...#.##..#",
            "#..##.#...",
            "..###..###",
            "###..###..",
            ".#####..#.",
            ".#..#####.",
        ];

        assert_eq!(expected_sides, all_tile_sides_combinations(TILE_2311));
    }

    #[test]
    fn test_convert_to_side_tiles_hashmap() {
        let tiles = parse_tiles(TEST_CASE_1);
        let tiles: HashMap<usize, Vec<String>> = tiles
            .iter()
            .map(|(tile_number, tile)| (*tile_number, all_tile_sides_combinations(tile)))
            .collect();
        let sides_to_tiles = convert_to_side_tiles_hashmap(&tiles);
        println!("{:?}", sides_to_tiles);
    }
}
