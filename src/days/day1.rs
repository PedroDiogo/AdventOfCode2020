extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day1.txt";
    let inputs = read_inputs(&filename);
    let inputs = split_lines_into_vec_int(&inputs);

    let part_one = multiply_2020_entries(&inputs);
    println!("Part one: {}", part_one);
}

fn multiply_2020_entries(lines: &Vec<i64>) -> i64 {
    find_2020_entries(lines)
    .iter()
    .fold(1, |acc, entry| acc * entry)
}

fn sum_line_digits(line: &str) -> u64 {
    line.chars()
    .map(|c| c.to_digit(10).expect("Expecting a digit") as u64)
    .fold(0, |acc, c| acc + c)
}

fn find_2020_entries(lines: &Vec<i64>) -> Vec<i64> {
    let mut sorted_entries = lines.clone();
    sorted_entries.sort();

    let entry_idx = &sorted_entries.iter()
        .find_map(|entry| sorted_entries.binary_search(&(2020-entry)).ok())
        .expect("Didn't find 2020 entries");
    let entry = sorted_entries.get(*entry_idx).unwrap().clone();

    vec![entry, 2020-entry]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_lines_digits() {
        assert_eq!(10, sum_line_digits("1234"));
    }

    #[test]
    fn test_find_2020_entries() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let expected_entries = vec![1721, 299];

        assert_eq!(expected_entries, find_2020_entries(&inputs));
    }

    #[test]
    fn test_case_1() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, multiply_2020_entries(&inputs));
    }
}