use super::lib::*;

use std::collections::HashMap;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day15.txt";
    let inputs = read_inputs(&filename);
    let starting_list: Vec<usize> = inputs
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let part_one = Some(get_nth_spoken_number(&starting_list, &2020).to_string());
    let part_two = None;

    (part_one, part_two)
}

fn get_nth_spoken_number(starting_list: &[usize], nth_number: &usize) -> usize {
    let mut memory: HashMap<usize, usize> = starting_list[0..starting_list.len() - 1]
        .iter()
        .enumerate()
        .map(|x| (*x.1, x.0 + 1))
        .collect();

    let mut last_number = *starting_list.last().unwrap();
    (starting_list.len() + 1..=*nth_number).for_each(|i| {
        let last_turn = i - 1;
        let turns_number = memory
            .get(&last_number)
            .map_or(0, |last_seen| last_turn - *last_seen);
        memory.insert(last_number, last_turn);
        last_number = turns_number;
    });
    last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_spoken_number() {
        assert_eq!(0, get_nth_spoken_number(&[0, 3, 6], &10));
        assert_eq!(436, get_nth_spoken_number(&[0, 3, 6], &2020));
        assert_eq!(1, get_nth_spoken_number(&[1, 3, 2], &2020));
        assert_eq!(10, get_nth_spoken_number(&[2, 1, 3], &2020));
        assert_eq!(27, get_nth_spoken_number(&[1, 2, 3], &2020));
        assert_eq!(78, get_nth_spoken_number(&[2, 3, 1], &2020));
        assert_eq!(438, get_nth_spoken_number(&[3, 2, 1], &2020));
        assert_eq!(1836, get_nth_spoken_number(&[3, 1, 2], &2020));
    }
}
