use super::lib::*;

use std::collections::HashMap;

extern crate itertools;
use self::itertools::sorted;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day10.txt";
    let inputs = read_inputs(&filename);

    let mut inputs: Vec<usize> = inputs
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    inputs.push(0);
    inputs.push(inputs.iter().max().unwrap() + 3);

    let jumps = get_jumps_for_longest_path(&inputs);
    let part_one = Some((jumps.0 * jumps.1).to_string());
    let part_two = Some(get_number_of_paths(&inputs).to_string());

    (part_one, part_two)
}

fn get_jumps_for_longest_path(inputs: &[usize]) -> (usize, usize) {
    let jumps = sorted(inputs.iter()).fold(
        (0, 0, 0 as usize),
        |(one_jumps, three_jumps, last_adapter), adapter| match adapter - last_adapter {
            1 => (one_jumps + 1, three_jumps, *adapter),
            3 => (one_jumps, three_jumps + 1, *adapter),
            _ => (one_jumps, three_jumps, *adapter),
        },
    );
    (jumps.0, jumps.1)
}

fn get_number_of_paths(inputs: &[usize]) -> usize {
    let max_input = *inputs.iter().max().unwrap();
    let mut initial_paths_for_node: HashMap<usize, usize> = HashMap::with_capacity(inputs.len());
    initial_paths_for_node.entry(max_input).or_insert(1);

    *sorted(inputs.iter())
        .rev()
        .skip(1)
        .fold(initial_paths_for_node, |mut paths_for_node, node| {
            let number_of_paths = (1..=3).fold(0, |acc, index| {
                acc + paths_for_node.get(&(node + index)).or(Some(&0)).unwrap()
            });
            paths_for_node.insert(*node, number_of_paths);
            paths_for_node
        })
        .get(&0)
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CASE_1: [usize; 14] = [22, 19, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0];
    const TEST_CASE_2: [usize; 33] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3, 52, 0,
    ];

    #[test]
    fn test_get_jumps_for_longest_path() {
        let test_case_1 = get_jumps_for_longest_path(&TEST_CASE_1);
        assert_eq!(7, test_case_1.0);
        assert_eq!(5, test_case_1.1);
        let test_case_2 = get_jumps_for_longest_path(&TEST_CASE_2);
        assert_eq!(22, test_case_2.0);
        assert_eq!(10, test_case_2.1);
    }

    #[test]
    fn test_get_number_of_paths() {
        assert_eq!(8, get_number_of_paths(&TEST_CASE_1));
        assert_eq!(19208, get_number_of_paths(&TEST_CASE_2));
    }
}
