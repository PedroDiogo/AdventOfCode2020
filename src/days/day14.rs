use super::lib::*;

extern crate regex;
use self::regex::Regex;

use std::collections::HashMap;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day14.txt";
    let inputs = read_inputs(&filename);

    let part_one_memory = apply_mask_to_inputs(&inputs);
    let part_one = Some(part_one_memory.values().sum::<usize>().to_string());
    let part_two_memory = apply_memory_address_decoder(&inputs);
    let part_two = Some(part_two_memory.values().sum::<usize>().to_string());

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
    let mut mask = "";
    let mut memory = HashMap::new();

    for line in input.lines() {
        if let Some(new_mask) = parse_mask(line) {
            mask = new_mask;
        } else if let Some((address, value)) = parse_memory(line) {
            let masked_value = apply_mask(&value, mask);
            memory.insert(address, masked_value);
        }
    }
    memory
}

fn apply_memory_address_decoder(input: &str) -> HashMap<usize, usize> {
    let mut mask = "";
    let mut memory = HashMap::new();

    for line in input.lines() {
        if let Some(new_mask) = parse_mask(line) {
            mask = new_mask;
        } else if let Some((address, value)) = parse_memory(line) {
            get_all_possible_addresses(&address, mask)
                .iter()
                .for_each(|address| {
                    memory.insert(*address, value);
                })
        }
    }
    memory
}
fn apply_mask(input: &usize, mask: &str) -> usize {
    let and_mask =
        usize::from_str_radix(&mask.replace("X", "1"), 2).expect("Expected a valid mask");
    let or_mask = usize::from_str_radix(&mask.replace("X", "0"), 2).expect("Expected a valid mask");

    (input & and_mask) | or_mask
}
fn get_all_possible_addresses(address: &usize, mask: &str) -> Vec<usize> {
    get_all_possible_masks(mask)
        .iter()
        .map(|mask| apply_mask(address, mask))
        .collect()
}

fn get_all_possible_masks(mask: &str) -> Vec<String> {
    let mask_split: Vec<String> = mask
        .replace("X", "T")
        .replace("0", "X")
        .split('T')
        .map(|x| x.to_string())
        .collect();

    mask_split
        .get(0..mask_split.len() - 1)
        .unwrap()
        .iter()
        .fold(Vec::new(), |all_masks, split| {
            let mut masks = Vec::new();
            let split_with_zero = format!("{}0", split);
            let split_with_one = format!("{}1", split);

            if all_masks.is_empty() {
                masks.push(split_with_zero);
                masks.push(split_with_one);
            } else {
                let masks_ones = all_masks
                    .iter()
                    .map(|x| format!("{}{}", *x, split_with_one));
                let masks_zeros = all_masks
                    .iter()
                    .map(|x| format!("{}{}", *x, split_with_zero));
                masks.extend(masks_ones);
                masks.extend(masks_zeros);
            }
            masks
        })
        .iter()
        .map(|mask| format!("{}{}", mask, mask_split.last().unwrap()))
        .collect()
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
    fn test_get_all_possible_addresses() {
        let mask = "000000000000000000000000000000X1001X";
        let address = 42;
        let expected_addresses = vec![26, 27, 58, 59];
        let mut all_addresses = get_all_possible_addresses(&address, mask);
        all_addresses.sort_unstable();

        assert_eq!(expected_addresses, all_addresses);

        let mask = "00000000000000000000000000000000X0XX";
        let address = 26;
        let expected_addresses = vec![16, 17, 18, 19, 24, 25, 26, 27];
        let mut all_addresses = get_all_possible_addresses(&address, mask);
        all_addresses.sort_unstable();

        assert_eq!(expected_addresses, all_addresses);

        let mask = "1001100011101100110010101110010010X1";
        let address = 26;
        let all_addresses = get_all_possible_addresses(&address, mask);

        assert_eq!(2, all_addresses.len());

        let mask = "100110001110110011001X101110X1XX10X1";
        let address = 26;
        let all_addresses = get_all_possible_addresses(&address, mask);

        assert_eq!(32, all_addresses.len());
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

    #[test]
    fn test_case_2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let memory = apply_memory_address_decoder(input);
        assert_eq!(10, memory.len());
        (58..=59).for_each(|address| assert_eq!(&100, memory.get(&address).unwrap()));
        (16..=19).for_each(|address| assert_eq!(&1, memory.get(&address).unwrap()));
        (24..=27).for_each(|address| assert_eq!(&1, memory.get(&address).unwrap()));
    }
}
