use super::lib::*;

use std::ops::RangeInclusive;

extern crate regex;
use self::regex::Regex;

use std::collections::HashSet;

type RuleType = (String, RangeInclusive<usize>, RangeInclusive<usize>);
type TicketType = Vec<usize>;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day16.txt";
    let inputs = read_inputs(&filename);

    let rules = parse_rules(&inputs);
    let nearby_tickets = parse_nearby_tickets(&inputs);

    let invalid_fields = find_invalid_fields(&nearby_tickets, &rules);
    let part_one = Some(invalid_fields.iter().sum::<usize>().to_string());

    let your_ticket = parse_your_ticket(&inputs).expect("Expected your ticket");
    let nearby_tickets = filter_valid_tickets(&nearby_tickets, &rules);

    let fields = find_fields(&nearby_tickets, &rules);
    let part_two = Some(
        fields
            .iter()
            .enumerate()
            .filter(|(_, field)| field.starts_with("departure"))
            .filter_map(|(idx, _)| your_ticket.get(idx))
            .product::<usize>()
            .to_string(),
    );

    (part_one, part_two)
}

fn find_invalid_fields(tickets: &[TicketType], rules: &[RuleType]) -> TicketType {
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

fn filter_valid_tickets(tickets: &[TicketType], rules: &[RuleType]) -> Vec<TicketType> {
    tickets
        .iter()
        .cloned()
        .filter(|ticket| ticket.iter().all(|field| is_field_valid(field, rules)))
        .collect()
}

fn is_field_valid(field: &usize, rules: &[RuleType]) -> bool {
    !valid_rules_for_field(field, rules).is_empty()
}

fn valid_rules_for_field(field: &usize, rules: &[RuleType]) -> HashSet<RuleType> {
    rules
        .iter()
        .cloned()
        .filter(|rule| rule.1.contains(field) || rule.2.contains(field))
        .collect()
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

fn parse_your_ticket(input: &str) -> Option<TicketType> {
    let your_ticket = input
        .split_by_blank_lines()
        .nth(1)
        .expect("Expected your ticket section");
    your_ticket.lines().nth(1).map(parse_ticket)
}

fn parse_nearby_tickets(input: &str) -> Vec<TicketType> {
    let your_ticket = input
        .split_by_blank_lines()
        .nth(2)
        .expect("Expected nearby tickets section");
    your_ticket.lines().skip(1).map(parse_ticket).collect()
}

fn parse_ticket(input: &str) -> TicketType {
    input
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn find_fields(tickets: &[TicketType], rules: &[RuleType]) -> Vec<String> {
    let number_of_fields = tickets.get(0).expect("Expected at least one ticket").len();
    let mut tickets_transposed: Vec<Vec<usize>> = Vec::with_capacity(number_of_fields);
    tickets_transposed.resize_with(number_of_fields, Vec::new);

    tickets.iter().for_each(|ticket| {
        ticket
            .iter()
            .cloned()
            .enumerate()
            .for_each(|(idx, field)| tickets_transposed.get_mut(idx).unwrap().push(field));
    });

    let valid_rules_for_fields: Vec<Vec<HashSet<RuleType>>> = tickets_transposed
        .iter()
        .map(|fields| {
            fields
                .iter()
                .map(|field| valid_rules_for_field(field, rules))
                .collect()
        })
        .collect();

    let valid_rules_intersection: Vec<HashSet<RuleType>> = valid_rules_for_fields
        .iter()
        .map(|field_column| {
            field_column.iter().fold(
                field_column.first().unwrap().clone(),
                |intersection, field| {
                    intersection
                        .intersection(&field)
                        .cloned()
                        .collect::<HashSet<RuleType>>()
                },
            )
        })
        .collect();

    let mut valid_rules: Vec<(usize, HashSet<RuleType>)> = valid_rules_intersection
        .iter()
        .cloned()
        .enumerate()
        .collect();
    valid_rules.sort_by_key(|rule| rule.1.len());

    for i in 1..number_of_fields {
        let last = valid_rules.get(i - 1).unwrap().clone().1;

        valid_rules
            .iter_mut()
            .skip(i)
            .for_each(|(_, rules)| *rules = rules.difference(&last).cloned().collect());
    }

    valid_rules.sort_by_key(|rule| rule.0);

    valid_rules
        .iter()
        .map(|(_, rule)| rule)
        .inspect(|field_rules| assert!(field_rules.len() == 1))
        .map(|field_rules| field_rules.iter().cloned().next().unwrap().0)
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

    const TEST_CASE_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

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

    #[test]
    fn test_filter_valid_tickets() {
        let nearby_tickets = parse_nearby_tickets(TEST_CASE_1);
        let rules = parse_rules(TEST_CASE_1);
        assert_eq!(
            vec![vec![7, 3, 47]],
            filter_valid_tickets(&nearby_tickets, &rules)
        );
    }

    #[test]
    fn test_find_field_candidates() {
        let expected_fields = vec!["row".to_string(), "class".to_string(), "seat".to_string()];
        let nearby_tickets = parse_nearby_tickets(TEST_CASE_2);
        let rules = parse_rules(TEST_CASE_2);

        assert_eq!(expected_fields, find_fields(&nearby_tickets, &rules));
    }
}
