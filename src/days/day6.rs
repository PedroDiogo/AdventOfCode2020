extern crate advent;
use self::advent::*;

use std::collections::HashSet;

pub fn run() {
    let filename = "inputs/day6.txt";
    let inputs = read_inputs(&filename);
    let group_questions = inputs.split_by_blank_lines();

    let part_one: usize = group_questions
        .map(get_unique_group_answers)
        .map(|group_answers| group_answers.len())
        .sum();

    println!("Part one: {}", part_one);

    let part_two = 0;
    println!("Part two: {}", part_two);
}

fn get_unique_group_answers(group_answers: &str) -> HashSet<char> {
    group_answers.replace("\n", "").chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_group_answers() {
        let answer1 = "abcabc";
        let answer2 = "ab
ac";
        let mut expected_answers = HashSet::new();
        expected_answers.insert('a');
        expected_answers.insert('b');
        expected_answers.insert('c');

        assert_eq!(expected_answers, get_unique_group_answers(answer1));
        assert_eq!(expected_answers, get_unique_group_answers(answer2));
    }
}
