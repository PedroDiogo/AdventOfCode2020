use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day1.txt";
    let inputs = read_inputs(&filename);
    let inputs: Vec<i64> = inputs
        .lines_of::<i64>()
        .into_iter()
        .filter_map(|x| x)
        .collect();

    let entries = find_two_entries_that_sum_to(&inputs, &2020).unwrap();
    let part_one = multiply_entries(&entries);

    let entries = find_three_entries_that_sum_to(&inputs, &2020).unwrap();
    let part_two = multiply_entries(&entries);

    (Some(part_one.to_string()), Some(part_two.to_string()))
}

fn multiply_entries(entries: &[i64]) -> i64 {
    entries.iter().product()
}

fn find_three_entries_that_sum_to(lines: &[i64], sum: &i64) -> Option<Vec<i64>> {
    let mut sorted_entries = lines.to_owned();
    sorted_entries.sort_unstable();

    let entries = sorted_entries
        .iter()
        .find_map(|entry| find_two_entries_that_sum_to(lines, &(*sum - *entry)));

    if let Some(mut entries) = entries {
        entries.push(sum - entries[0] - entries[1]);
        Some(entries)
    } else {
        None
    }
}

fn find_two_entries_that_sum_to(lines: &[i64], sum: &i64) -> Option<Vec<i64>> {
    let mut sorted_entries = lines.to_owned();
    sorted_entries.sort_unstable();

    let entry_idx = &sorted_entries
        .iter()
        .find_map(|entry| sorted_entries.binary_search(&(*sum - entry)).ok());

    if let Some(entry_idx) = entry_idx {
        let entry = *sorted_entries.get(*entry_idx).unwrap();
        Some(vec![entry, *sum - entry])
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

        assert_eq!(
            expected_entries,
            find_two_entries_that_sum_to(&inputs, &2020)
        );
    }

    #[test]
    fn test_find_three_entries_that_sum_to() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let expected_entries = Some(vec![979, 675, 366]);

        assert_eq!(
            expected_entries,
            find_three_entries_that_sum_to(&inputs, &2020)
        );
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
