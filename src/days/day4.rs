extern crate advent;
use self::advent::*;

use std::collections::HashMap;

const BIRTH_YEAR: &str = "byr";
const ISSUE_YEAR: &str = "iyr";
const EXPIRATION_YEAR: &str = "eyr";
const HEIGHT: &str = "hgt";
const HAIR_COLOR: &str = "hcl";
const EYE_COLOR: &str = "ecl";
const PASSPORT_ID: &str = "pid";
const COUNTRY_ID: &str = "cid";

pub fn run() {
    let filename = "inputs/day4.txt";
    let inputs = read_inputs(&filename);
    let passwords = split_inputs_into_password_lines(&inputs);

    let part_one = passwords
        .iter()
        .map(|password_line| parse_password(password_line))
        .filter(|password| is_valid_password(password))
        .count();

    println!("Part one: {}", part_one);
}

fn parse_password(password_line: &str) -> HashMap<String, String> {
    password_line.replace("\n", " ").trim().split(" ").fold(
        HashMap::new(),
        |mut hashmap, element| {
            let element_parts: Vec<&str> = element.split(":").collect();
            let key = element_parts
                .get(0)
                .expect("Expected to have a key")
                .to_string();
            let value = element_parts
                .get(1)
                .expect("Expected to have a value")
                .to_string();
            hashmap.insert(key, value);
            hashmap
        },
    )
}

fn split_inputs_into_password_lines(inputs: &str) -> Vec<&str> {
    inputs.split("\n\n").collect()
}

fn is_valid_password(password: &HashMap<String, String>) -> bool {
    password.contains_key(BIRTH_YEAR)
        && password.contains_key(ISSUE_YEAR)
        && password.contains_key(EXPIRATION_YEAR)
        && password.contains_key(HEIGHT)
        && password.contains_key(HAIR_COLOR)
        && password.contains_key(EYE_COLOR)
        && password.contains_key(PASSPORT_ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_password() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

        let mut expected_hashmap: HashMap<String, String> = HashMap::new();
        expected_hashmap.insert(EYE_COLOR.to_string(), "gry".to_string());
        expected_hashmap.insert(PASSPORT_ID.to_string(), "860033327".to_string());
        expected_hashmap.insert(EXPIRATION_YEAR.to_string(), "2020".to_string());
        expected_hashmap.insert(HAIR_COLOR.to_string(), "#fffffd".to_string());
        expected_hashmap.insert(BIRTH_YEAR.to_string(), "1937".to_string());
        expected_hashmap.insert(ISSUE_YEAR.to_string(), "2017".to_string());
        expected_hashmap.insert(COUNTRY_ID.to_string(), "147".to_string());
        expected_hashmap.insert(HEIGHT.to_string(), "183cm".to_string());

        assert_eq!(expected_hashmap, parse_password(input));
    }

    #[test]
    fn test_split_inputs_into_password_lines() {
        let inputs = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let password_lines = split_inputs_into_password_lines(inputs);
        println!("{:?}", password_lines);

        assert_eq!(4, password_lines.len());
    }

    #[test]
    fn test_is_valid_password() {
        let mut valid_password: HashMap<String, String> = HashMap::new();
        valid_password.insert(EYE_COLOR.to_string(), "gry".to_string());
        valid_password.insert(PASSPORT_ID.to_string(), "860033327".to_string());
        valid_password.insert(EXPIRATION_YEAR.to_string(), "2020".to_string());
        valid_password.insert(HAIR_COLOR.to_string(), "#fffffd".to_string());
        valid_password.insert(BIRTH_YEAR.to_string(), "1937".to_string());
        valid_password.insert(ISSUE_YEAR.to_string(), "2017".to_string());
        valid_password.insert(COUNTRY_ID.to_string(), "147".to_string());
        valid_password.insert(HEIGHT.to_string(), "183cm".to_string());
        let mut invalid_password = HashMap::new();
        invalid_password.insert(EYE_COLOR.to_string(), "gry".to_string());

        assert_eq!(true, is_valid_password(&valid_password));
        assert_eq!(false, is_valid_password(&invalid_password));
    }
}
