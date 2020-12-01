extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day1.txt";
    let inputs = read_inputs(&filename);
    let inputs = split_lines_into_vec_int(&inputs);

    let entries = find_two_entries_that_sum_to(&inputs, &2020).unwrap();
    let part_one = multiply_entries(&entries);
    println!("Part one: {}", part_one);

    let entries = find_three_entries_that_sum_to(&inputs, &2020).unwrap();
    let part_two = multiply_entries(&entries);
    println!("Part two: {}", part_two);
}

fn multiply_entries(entries: &Vec<i64>) -> i64 {
    entries
    .iter()
    .fold(1, |acc, entry| acc * entry)
}

fn find_three_entries_that_sum_to(lines: &Vec<i64>, sum: &i64) -> Option<Vec<i64>> {
    let mut sorted_entries = lines.clone();
    sorted_entries.sort();

    let entries = sorted_entries
    .iter()
    .find_map(|entry| find_two_entries_that_sum_to(lines, &(*sum-*entry)));

    return if let Some(mut entries) = entries {
        entries.push(sum - entries[0] - entries[1]);
        Some(entries)
    } else {
        None
    }
}

fn find_two_entries_that_sum_to(lines: &Vec<i64>, sum: &i64) -> Option<Vec<i64>> {
    let mut sorted_entries = lines.clone();
    sorted_entries.sort();

    let entry_idx = &sorted_entries.iter()
        .find_map(|entry| sorted_entries.binary_search(&(*sum-entry)).ok());

    return if let Some(entry_idx) = entry_idx {
        let entry = sorted_entries.get(*entry_idx).unwrap().clone();
        Some(vec![entry, *sum-entry])
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_two_entries_that_sum_to() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let expected_entries = Some(vec![1721, 299]);

        assert_eq!(expected_entries, find_two_entries_that_sum_to(&inputs, &2020));
    }

    #[test]
    fn test_find_three_entries_that_sum_to() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let expected_entries = Some(vec![979, 675, 366]);

        assert_eq!(expected_entries, find_three_entries_that_sum_to(&inputs, &2020));
    }
 
    #[test]
    fn test_case_1() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let entries = find_two_entries_that_sum_to(&inputs, &2020).unwrap();
        assert_eq!(514579, multiply_entries(&entries));
    }

    #[test]
    fn test_case_2() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let entries = find_three_entries_that_sum_to(&inputs, &2020).unwrap();
        assert_eq!(241861950, multiply_entries(&entries));
    }
}