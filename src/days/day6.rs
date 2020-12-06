use super::lib::*;
use std::collections::HashSet;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day6.txt";
    let inputs = read_inputs(&filename);
    let group_questions = inputs.split_by_blank_lines();

    let part_one: usize = group_questions
        .clone()
        .map(get_unique_group_answers)
        .map(|group_answers| group_answers.len())
        .sum();

    let part_two: usize = group_questions
        .map(get_unanimous_answers)
        .map(|group_answers| group_answers.len())
        .sum();

    (Some(part_one.to_string()), Some(part_two.to_string()))
}

fn get_unique_group_answers(group_answers: &str) -> HashSet<char> {
    group_answers.replace("\n", "").chars().collect()
}

fn get_unanimous_answers(group_answers: &str) -> HashSet<char> {
    let all_possible_answers: HashSet<char> = ('a'..='z').collect();

    group_answers
        .lines()
        .map(|person_answers| person_answers.chars().collect::<HashSet<char>>())
        .fold(
            all_possible_answers,
            |intersection_result, person_answers| {
                intersection_result
                    .intersection(&person_answers)
                    .map(|x| x.to_owned())
                    .collect()
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_group_answers() {
        let answer1 = "abcabc";
        let answer2 = "ab
ac";
        let expected_answers: HashSet<char> = vec!['a', 'b', 'c'].into_iter().collect();

        assert_eq!(expected_answers, get_unique_group_answers(answer1));
        assert_eq!(expected_answers, get_unique_group_answers(answer2));
    }

    #[test]
    fn test_get_unanimous_answers() {
        let answer1 = "abc";
        let answer2 = "ab
ac";

        assert_eq!(
            vec!['a', 'b', 'c'].into_iter().collect::<HashSet<char>>(),
            get_unanimous_answers(answer1)
        );
        assert_eq!(
            vec!['a'].into_iter().collect::<HashSet<char>>(),
            get_unanimous_answers(answer2)
        );
    }
}
