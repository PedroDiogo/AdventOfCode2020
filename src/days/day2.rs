extern crate advent;
use self::advent::*;

use std::collections::HashMap;

pub fn run() {
    let filename = "inputs/day2.txt";
    let inputs = read_inputs(&filename);
    let inputs: Vec<&str> = inputs.lines().collect();

    let part_one = number_of_valid_passwords(&inputs, valid_password);
    println!("Part one: {}", part_one);

    let part_two = number_of_valid_passwords(&inputs, valid_password_part_two);
    println!("Part two: {}", part_two);
}

fn number_of_valid_passwords(
    passwords: &[&str],
    validator_function: fn(&usize, &usize, &char, &str) -> bool,
) -> usize {
    passwords
        .iter()
        .map(|password| {
            scan_fmt!(password, "{d}-{d} {[a-z]}: {}", usize, usize, char, String).unwrap()
        })
        .filter(|(min, max, character, password)| validator_function(min, max, character, password))
        .count()
}

fn valid_password(min: &usize, max: &usize, character: &char, password: &str) -> bool {
    if let Some(character_hits) = character_distribution(password).get(character) {
        character_hits >= min && character_hits <= max
    } else {
        false
    }
}

fn valid_password_part_two(
    position_1: &usize,
    position_2: &usize,
    character: &char,
    password: &str,
) -> bool {
    let char_position1 = password
        .chars()
        .nth(*position_1 - 1)
        .expect("Expected a character in position 1");
    let char_position2 = password
        .chars()
        .nth(*position_2 - 1)
        .expect("Expected a character in position 2");

    char_position1 != char_position2
        && (char_position1 == *character || char_position2 == *character)
}

fn character_distribution(line: &str) -> HashMap<char, usize> {
    line.chars().fold(HashMap::new(), |mut map, character| {
        if let Some(existing_character_value) = map.get_mut(&character) {
            *existing_character_value += 1;
        } else {
            map.insert(character, 1);
        }
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_valid_passwords() {
        let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(2, number_of_valid_passwords(&passwords, valid_password));
    }
    #[test]
    fn test_valid_password() {
        assert_eq!(true, valid_password(&1usize, &3usize, &'a', "abcde"));
        assert_eq!(false, valid_password(&2usize, &3usize, &'a', "abcde"));
    }

    #[test]
    fn test_valid_password_part_two() {
        assert_eq!(
            true,
            valid_password_part_two(&1usize, &3usize, &'a', "abcde")
        );
        assert_eq!(
            false,
            valid_password_part_two(&1usize, &3usize, &'b', "cdefg")
        );
        assert_eq!(
            false,
            valid_password_part_two(&1usize, &3usize, &'c', "ccccccccc")
        );
    }

    #[test]
    fn test_character_distribution() {
        let mut expected_distribution = HashMap::new();
        expected_distribution.insert('c', 5);
        expected_distribution.insert('a', 5);
        expected_distribution.insert('b', 2);

        assert_eq!(
            expected_distribution,
            character_distribution("abcabcacacac")
        );
    }
}
