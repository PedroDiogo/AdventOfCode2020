use super::lib::*;

use std::collections::HashMap;
use std::collections::HashSet;

extern crate itertools;
use self::itertools::*;

type Rules<'a> = HashMap<&'a str, Vec<Vec<&'a str>>>;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day19.txt";
    let inputs = read_inputs(&filename);
    let rules = parse_rules(&inputs);
    let inputs = parse_inputs(&inputs);

    let rule_0: HashSet<String> = get_rule("0", &rules).iter().cloned().collect();

    let part_one = Some(
        inputs
            .iter()
            .filter(|input| rule_0.contains(**input))
            .count()
            .to_string(),
    );

    let part_two = None;

    (part_one, part_two)
}

fn parse_rules(inputs: &str) -> Rules {
    inputs
        .split_by_blank_lines()
        .collect::<Vec<&str>>()
        .get(0)
        .expect("Expected rules section")
        .lines()
        .map(|line| {
            let line: Vec<_> = line.split(':').collect();
            let rule_number = *line.get(0).expect("Expected rule number");
            let rule_dependencies = line.get(1).expect("Expected rule dependencies");

            let rule_dependencies: Vec<Vec<_>> = rule_dependencies
                .split('|')
                .map(|rule| rule.split_whitespace().collect())
                .collect();

            (rule_number, rule_dependencies)
        })
        .collect()
}

fn parse_inputs(inputs: &str) -> Vec<&str> {
    inputs
        .split_by_blank_lines()
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Expected inputs section")
        .lines()
        .collect()
}

fn get_rule<'a>(rule: &str, rules: &'a Rules<'a>) -> Vec<String> {
    let rule_dependencies = rules.get(rule).unwrap();

    rule_dependencies
        .iter()
        .map(|dependent_rule| {
            dependent_rule
                .iter()
                .map(|sequence| {
                    if sequence.starts_with('"') {
                        vec![sequence.to_string().replace("\"", "")]
                    } else {
                        get_rule(*sequence, rules)
                    }
                })
                .collect::<Vec<Vec<String>>>()
        })
        .map(|x| cartesian_product_rules(&x))
        .flatten()
        .collect()
}

fn cartesian_product_rules(rules: &[Vec<String>]) -> Vec<String> {
    rules
        .iter()
        .skip(1)
        .fold(rules[0].clone(), |result, element| {
            result
                .iter()
                .cartesian_product(element.iter())
                .map(|(left, right)| [left.clone(), right.clone()].concat())
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";

    const TEST_CASE_2: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_parse_rules() {
        let mut expected_result = HashMap::new();
        expected_result.insert("0", vec![vec!["1", "2"]]);
        expected_result.insert("1", vec![vec!["\"a\""]]);
        expected_result.insert("2", vec![vec!["1", "3"], vec!["3", "1"]]);
        expected_result.insert("3", vec![vec!["\"b\""]]);

        assert_eq!(expected_result, parse_rules(TEST_CASE_1));
    }

    #[test]
    fn test_parse_inputs() {
        let inputs = parse_inputs(TEST_CASE_2);
        let expected = vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];

        assert_eq!(expected, inputs);
    }

    #[test]
    fn test_get_rule() {
        let rules = parse_rules(TEST_CASE_1);
        assert_eq!(vec!["a"], get_rule("1", &rules));
        assert_eq!(vec!["ab", "ba"], get_rule("2", &rules));
        assert_eq!(vec!["aab", "aba"], get_rule("0", &rules));
        let rules = parse_rules(TEST_CASE_2);
        assert_eq!(
            vec!["aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb"],
            get_rule("0", &rules)
        )
    }

    #[test]
    fn test_cartesian_product_rules() {
        let a = vec!["ab".to_string()];
        let b = vec!["cd".to_string(), "ef".to_string()];
        let rule = vec![a, b];

        let expected = vec!["abcd", "abef"];

        assert_eq!(expected, cartesian_product_rules(&rule));
    }
}
