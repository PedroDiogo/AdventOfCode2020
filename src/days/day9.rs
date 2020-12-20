use super::lib::*;

use std::collections::HashSet;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day9.txt";
    let inputs = read_inputs(&filename);
    let inputs: Vec<usize> = inputs
        .lines_of::<usize>()
        .iter()
        .filter_map(|x| *x)
        .collect();

    let part_one = Some(find_first_number_not_sum_of_two_previous(&inputs, 25).to_string());
    let part_two = None;

    (part_one, part_two)
}

fn find_first_number_not_sum_of_two_previous(inputs: &[usize], look_behind: usize) -> usize {
    let result = inputs
        .windows(look_behind + 1)
        .map(|window| {
            (
                window.get(look_behind).unwrap(),
                window
                    .get(0..look_behind)
                    .unwrap()
                    .iter()
                    .cloned()
                    .collect::<HashSet<usize>>(),
            )
        })
        .find(|(next_number, previous_numbers)| {
            !is_number_sum_of_two_others(next_number, previous_numbers)
        })
        .unwrap();
    *result.0
}

fn is_number_sum_of_two_others(number: &usize, previous_numbers: &HashSet<usize>) -> bool {
    previous_numbers
        .iter()
        .filter(|previous_number| *previous_number <= number)
        .map(|previous_number| previous_numbers.contains(&((*number) - previous_number)))
        .any(|x| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_number_not_sum_of_two_previous() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(127, find_first_number_not_sum_of_two_previous(&numbers, 5));
    }
}
