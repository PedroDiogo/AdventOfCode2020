use super::lib::*;

use std::cmp::Ordering;
use std::collections::HashSet;

extern crate itertools;
use self::itertools::Itertools;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day9.txt";
    let inputs = read_inputs(&filename);
    let inputs: Vec<usize> = inputs
        .lines_of::<usize>()
        .iter()
        .filter_map(|x| *x)
        .collect();

    let part_one_solution = find_first_number_not_sum_of_two_previous(&inputs, 25);
    let part_one = Some(part_one_solution.to_string());
    let part_two_solution = find_continuous_list_that_sums_to_number(&inputs, &part_one_solution);
    let part_two_solution = part_two_solution.iter().minmax().into_option().unwrap();
    let part_two = Some((part_two_solution.0 + part_two_solution.1).to_string());

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

fn find_continuous_list_that_sums_to_number(inputs: &[usize], number: &usize) -> Vec<usize> {
    let mut tail_idx = 0;
    let mut head_idx = 1;
    let mut sum: usize = inputs.get(tail_idx..=head_idx).unwrap().iter().sum();

    loop {
        match sum.cmp(number) {
            Ordering::Greater => {
                sum -= inputs.get(tail_idx).unwrap();
                tail_idx += 1;
            }
            Ordering::Less => {
                head_idx += 1;
                sum += inputs.get(head_idx).unwrap();
            }
            Ordering::Equal => break,
        }
    }
    inputs.get(tail_idx..=head_idx).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [usize; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_first_number_not_sum_of_two_previous() {
        assert_eq!(127, find_first_number_not_sum_of_two_previous(&NUMBERS, 5));
    }

    #[test]
    fn test_find_continuous_list_that_sums_to_number() {
        assert_eq!(
            [15, 25, 47, 40].to_vec(),
            find_continuous_list_that_sums_to_number(&NUMBERS, &127),
        );
    }
}
