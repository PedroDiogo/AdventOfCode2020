use super::lib::*;

use std::ops::RangeInclusive;

extern crate regex;
use self::regex::Regex;

type RuleType = (String, RangeInclusive<usize>, RangeInclusive<usize>);

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day16.txt";
    let inputs = read_inputs(&filename);

    let rules = parse_rules(&inputs);
    let _your_ticket = parse_your_ticket(&inputs);
    let nearby_tickets = parse_nearby_tickets(&inputs);

    let invalid_fields = find_invalid_fields(&nearby_tickets, &rules);
    let part_one = Some(invalid_fields.iter().sum::<usize>().to_string());
    let part_two = None;

    (part_one, part_two)
}

fn find_invalid_fields(tickets: &[Vec<usize>], rules: &[RuleType]) -> Vec<usize> {
    let invalid_tickets = tickets
        .iter()
        .map(|ticket| {
            let invalid_fields: Vec<usize> = ticket
                .iter()
                .cloned()
                .filter(|field| !is_field_valid(field, rules))
                .collect();

            invalid_fields
        })
        .filter(|ticket| !ticket.is_empty());

    invalid_tickets.flatten().collect()
}

fn is_field_valid(field: &usize, rules: &[RuleType]) -> bool {
    rules
        .iter()
        .any(|rule| rule.1.contains(field) || rule.2.contains(field))
}

fn parse_rules(input: &str) -> Vec<RuleType> {
    let re = Regex::new(
        r"(?P<field>[^:]*): (?P<range1_min>\d+)-(?P<range1_max>\d+) or (?P<range2_min>\d+)-(?P<range2_max>\d+)",
    ).unwrap();
    let field_rules = input
        .split_by_blank_lines()
        .next()
        .expect("Expected field rules section");

    re.captures_iter(field_rules)
        .filter_map(|capture| {
            let field_name = capture.name("field").map(|x| x.as_str().trim().to_string());
            let range1 = create_range(
                &capture.name("range1_min").map(|x| x.as_str()),
                &capture.name("range1_max").map(|x| x.as_str()),
            );
            let range2 = create_range(
                &capture.name("range2_min").map(|x| x.as_str()),
                &capture.name("range2_max").map(|x| x.as_str()),
            );

            match (field_name, range1, range2) {
                (Some(field_name), Some(range1), Some(range2)) => {
                    Some((field_name, range1, range2))
                }
                _ => None,
            }
        })
        .collect()
}

fn create_range(min: &Option<&str>, max: &Option<&str>) -> Option<RangeInclusive<usize>> {
    let (min, max): (Option<usize>, Option<usize>) = match (min, max) {
        (Some(min), Some(max)) => (min.parse::<usize>().ok(), max.parse::<usize>().ok()),
        _ => (None, None),
    };

    match (min, max) {
        (Some(min), Some(max)) => Some(RangeInclusive::new(min, max)),
        _ => None,
    }
}

fn parse_your_ticket(input: &str) -> Option<Vec<usize>> {
    let your_ticket = input
        .split_by_blank_lines()
        .nth(1)
        .expect("Expected your ticket section");
    your_ticket.lines().nth(1).map(parse_ticket)
}

fn parse_nearby_tickets(input: &str) -> Vec<Vec<usize>> {
    let your_ticket = input
        .split_by_blank_lines()
        .nth(2)
        .expect("Expected nearby tickets section");
    your_ticket.lines().skip(1).map(parse_ticket).collect()
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_parse_rules() {
        let expected_rules = vec![
            ("class".to_string(), 1..=3, 5..=7),
            ("row".to_string(), 6..=11, 33..=44),
            ("seat".to_string(), 13..=40, 45..=50),
        ];

        assert_eq!(expected_rules, parse_rules(TEST_CASE_1));
    }

    #[test]
    fn test_parse_your_ticket() {
        assert_eq!(Some(vec![7, 1, 14]), parse_your_ticket(TEST_CASE_1));
    }

    #[test]
    fn test_parse_nearby_tickets() {
        let expected_nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];
        assert_eq!(expected_nearby_tickets, parse_nearby_tickets(TEST_CASE_1));
    }

    #[test]
    fn test_find_invalid_fields() {
        let nearby_tickets = parse_nearby_tickets(TEST_CASE_1);
        let rules = parse_rules(TEST_CASE_1);

        assert_eq!(
            vec![4, 55, 12],
            find_invalid_fields(&nearby_tickets, &rules)
        );
    }
}
