use super::lib::*;

use std::collections::HashMap;
use std::collections::HashSet;

extern crate itertools;
use self::itertools::*;

extern crate regex;
use self::regex::RegexSet;

type Rules<'a> = HashMap<&'a str, Vec<Vec<String>>>;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day19.txt";
    let inputs = read_inputs(&filename);
    let mut rules = parse_rules(&inputs);
    let inputs = parse_inputs(&inputs);

    let rule_0: HashSet<String> = get_rule("0", &mut rules).iter().cloned().collect();

    let part_one = Some(
        inputs
            .iter()
            .filter(|input| rule_0.contains(**input))
            .count()
            .to_string(),
    );

    let part_two = Some(number_of_matching_with_looped_rules(&inputs, &rules).to_string());

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

            let rule_dependencies: Vec<Vec<String>> = rule_dependencies
                .split('|')
                .map(|rule| rule.split_whitespace().map(|x| x.to_string()).collect())
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

fn get_rule(rule: &str, rules: &mut Rules) -> Vec<String> {
    let r = rules.clone();
    let rule_dependencies = r.get(rule).unwrap();

    rule_dependencies
        .iter()
        .map(|dependent_rule| {
            dependent_rule
                .iter()
                .map(|sequence| {
                    if sequence.starts_with('"') {
                        vec![sequence.to_string().replace("\"", "")]
                    } else {
                        let sequence_rule = get_rule(sequence, rules);
                        if let Some(x) = rules.get_mut(sequence.as_str()) {
                            *x = sequence_rule
                                .iter()
                                .map(|x| {
                                    let result = format!("\"{}\"", x);
                                    vec![result]
                                })
                                .collect();
                        };
                        sequence_rule
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
                .into_iter()
                .cartesian_product(element.iter())
                .map(|(left, right)| [left, right.to_string()].concat())
                .collect()
        })
}

fn number_of_matching_with_looped_rules(inputs: &[&str], rules: &Rules) -> usize {
    let rule_42 = rules
        .get("42")
        .unwrap()
        .iter()
        .flatten()
        .map(|x| x.replace("\"", ""))
        .join("|");
    let rule_31 = rules
        .get("31")
        .unwrap()
        .iter()
        .flatten()
        .map(|x| x.replace("\"", ""))
        .join("|");

    // Rust regex doesn't support recursive statements, so we're manually making sure
    // the last 42 appears the same amount of times as 31.
    // The number 4 comes from manually verifying the smallest number that gives me the same result for my input.
    let regexes = (1..=4)
        .map(|i| {
            let rule_8_re_pattern = format!("({})+", rule_42);
            let rule_11_re_pattern = format!("({}){{{}}}({}){{{}}}", rule_42, i, rule_31, i);
            format!("^{}{}$", rule_8_re_pattern, rule_11_re_pattern)
        })
        .collect::<Vec<String>>();

    let regexset = RegexSet::new(&regexes).unwrap();
    inputs
        .iter()
        .filter(|input| regexset.is_match(input))
        .count()
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

    const TEST_CASE_3: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_parse_rules() {
        let mut expected_result = HashMap::new();
        expected_result.insert("0", vec![vec!["1".to_string(), "2".to_string()]]);
        expected_result.insert("1", vec![vec!["\"a\"".to_string()]]);
        expected_result.insert(
            "2",
            vec![
                vec!["1".to_string(), "3".to_string()],
                vec!["3".to_string(), "1".to_string()],
            ],
        );
        expected_result.insert("3", vec![vec!["\"b\"".to_string()]]);

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
        let mut rules = parse_rules(TEST_CASE_1);
        assert_eq!(vec!["a"], get_rule("1", &mut rules));
        assert_eq!(vec!["ab", "ba"], get_rule("2", &mut rules));
        assert_eq!(vec!["aab", "aba"], get_rule("0", &mut rules));
        let mut rules = parse_rules(TEST_CASE_2);
        assert_eq!(
            vec!["aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb"],
            get_rule("0", &mut rules)
        )
    }

    #[test]
    fn test_number_of_matching_with_looped_rules() {
        let mut rules = parse_rules(TEST_CASE_3);
        let inputs = parse_inputs(TEST_CASE_3);
        get_rule("0", &mut rules);

        assert_eq!(12, number_of_matching_with_looped_rules(&inputs, &rules));
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
