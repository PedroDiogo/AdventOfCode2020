use super::lib::*;

extern crate regex;
use self::regex::Regex;

use std::collections::HashMap;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day14.txt";
    let inputs = read_inputs(&filename);

    let part_one_memory = apply_mask_to_inputs(&inputs);
    let part_one = Some(part_one_memory.values().sum::<usize>().to_string());
    let part_two = None;

    (part_one, part_two)
}

fn parse_mask(input: &str) -> Option<&str> {
    let re = Regex::new(r"mask = (?P<mask>[0|1|X]+)").unwrap();

    re.captures(input)
        .and_then(|cap| cap.name("mask").map(|x| x.as_str()))
}

fn parse_memory(input: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    re.captures(input).and_then(|cap| {
        let address = cap
            .name("address")
            .map(|x| x.as_str().parse::<usize>().ok())
            .flatten();
        let value = cap
            .name("value")
            .map(|x| x.as_str().parse::<usize>().ok())
            .flatten();
        match (address, value) {
            (Some(address), Some(value)) => Some((address, value)),
            _ => None,
        }
    })
}

fn apply_mask_to_inputs(input: &str) -> HashMap<usize, usize> {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut memory = HashMap::new();

    for line in input.lines() {
        if let Some(mask) = parse_mask(line) {
            and_mask =
                usize::from_str_radix(&mask.replace("X", "1"), 2).expect("Expected a valid mask");
            or_mask =
                usize::from_str_radix(&mask.replace("X", "0"), 2).expect("Expected a valid mask");
        } else if let Some((address, value)) = parse_memory(line) {
            let masked_value = (value & and_mask) | or_mask;
            memory.insert(address, masked_value);
        }
    }
    memory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mask() {
        assert_eq!(
            Some("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );
        assert_eq!(None, parse_mask("mem[8] = 11"));
    }

    #[test]
    fn test_parse_memory() {
        assert_eq!(Some((8usize, 11usize)), parse_memory("mem[8] = 11"));
        assert_eq!(
            None,
            parse_memory("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );
    }

    #[test]
    fn test_case_1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let memory = apply_mask_to_inputs(input);
        assert_eq!(2, memory.len());
        assert_eq!(&101, memory.get(&7).unwrap());
        assert_eq!(&64, memory.get(&8).unwrap());
    }
}
