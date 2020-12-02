extern crate advent;
use self::advent::*;

use std::collections::HashMap;


pub fn run() {
    let filename = "inputs/day2.txt";
    let inputs = read_inputs(&filename);
    let inputs = inputs.split("\n").collect();

    let part_one = number_of_valid_passwords(&inputs);
    println!("Part one: {}", part_one);
}

fn number_of_valid_passwords(passwords: &Vec<&str>) -> usize {
    passwords.iter()
    .map(|password| scan_fmt!(password, "{d}-{d} {[a-z]}: {}", usize, usize, char, String).unwrap())
    .filter(|(min, max, character, password)| valid_password(min, max, character, password))
    .count()
}

fn valid_password(min: &usize, max: &usize, character: &char, password: &str) -> bool {
    return if let Some(character_hits) = character_distribution(password).get(character) {
        if character_hits >= min && character_hits <= max {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn character_distribution(line: &str) -> HashMap<char, usize> {
    line.chars()
    .fold(HashMap::new(), |mut map, character| {
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
        assert_eq!(2, number_of_valid_passwords(&passwords));
    }
    #[test]
    fn test_valid_password() {
        assert_eq!(true, valid_password(&1usize, &3usize, &'a', "abcde"));
        assert_eq!(false, valid_password(&2usize, &3usize, &'a', "abcde"));
    }

    #[test]
    fn test_character_distribution() {
        let mut expected_distribution = HashMap::new();
        expected_distribution.insert('c', 5);
        expected_distribution.insert('a', 5);
        expected_distribution.insert('b', 2);

        assert_eq!(expected_distribution, character_distribution("abcabcacacac"));
    }
}