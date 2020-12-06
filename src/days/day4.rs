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

pub fn run() {
    let filename = "inputs/day4.txt";
    let inputs = read_inputs(&filename);
    let passwords: Vec<&str> = inputs.split_by_blank_lines().collect();

    let part_one = count_valid_passwords(&passwords, is_valid_password);
    println!("Part one: {}", part_one);

    let part_two = count_valid_passwords(&passwords, is_valid_complex_password);
    println!("Part two: {}", part_two);
}

fn count_valid_passwords(
    password_lines: &[&str],
    validator_function: fn(&HashMap<String, String>) -> bool,
) -> usize {
    password_lines
        .iter()
        .map(|password_line| parse_password(password_line))
        .filter(|password| validator_function(password))
        .count()
}

fn parse_password(password_line: &str) -> HashMap<String, String> {
    password_line
        .split_whitespace()
        .fold(HashMap::new(), |mut hashmap, element| {
            let element_parts: Vec<&str> = element.split(':').collect();
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
        })
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

fn is_valid_complex_password(password: &HashMap<String, String>) -> bool {
    is_valid_password(password)
        && validate_birth_year(password.get(BIRTH_YEAR).unwrap())
        && validate_issue_year(password.get(ISSUE_YEAR).unwrap())
        && validate_expiration_year(password.get(EXPIRATION_YEAR).unwrap())
        && validate_height(password.get(HEIGHT).unwrap())
        && validate_hair_color(password.get(HAIR_COLOR).unwrap())
        && validate_eye_color(password.get(EYE_COLOR).unwrap())
        && validate_passport_id(password.get(PASSPORT_ID).unwrap())
}

fn validate_birth_year(year: &str) -> bool {
    matches!(year.parse::<usize>(), Ok(1920..=2002))
}

fn validate_issue_year(year: &str) -> bool {
    matches!(year.parse::<usize>(), Ok(2010..=2020))
}

fn validate_expiration_year(year: &str) -> bool {
    matches!(year.parse::<usize>(), Ok(2020..=2030))
}

fn validate_height(height: &str) -> bool {
    let (height, unit) = scan_fmt_some!(height, "{d}{/(cm|in)/}", u64, String);
    let unit: &str = &unit.unwrap_or_else(|| String::from(""));

    match (height, unit) {
        (Some(150..=193), "cm") => true,
        (Some(59..=76), "in") => true,
        _ => false,
    }
}

fn validate_hair_color(hair_color: &str) -> bool {
    scan_fmt!(hair_color, "#{/[0-9a-f]{6}/}", [hex i64]).is_ok()
}

fn validate_eye_color(eye_color: &str) -> bool {
    matches!(
        eye_color,
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
}

fn validate_passport_id(passport_id: &str) -> bool {
    passport_id.len() == 9 && passport_id.parse::<i64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const COUNTRY_ID: &str = "cid";

    #[test]
    fn test_count_valid_passwords() {
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
        assert_eq!(
            2,
            count_valid_passwords(
                &inputs.split_by_blank_lines().collect::<Vec<&str>>(),
                is_valid_password
            )
        );
    }

    #[test]
    fn test_invalid_complex_passports() {
        let passports = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(
            0,
            count_valid_passwords(
                &passports.split_by_blank_lines().collect::<Vec<&str>>(),
                is_valid_complex_password
            )
        );
    }

    #[test]
    fn test_valid_complex_passports() {
        let passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(
            4,
            count_valid_passwords(
                &passports.split_by_blank_lines().collect::<Vec<&str>>(),
                is_valid_complex_password
            )
        );
    }

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

        let password_lines: Vec<&str> = inputs.split_by_blank_lines().collect();
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

    #[test]
    fn test_validate_birth_year() {
        assert_eq!(false, validate_birth_year("1919"));
        assert_eq!(true, validate_birth_year("1920"));
        assert_eq!(true, validate_birth_year("2002"));
        assert_eq!(false, validate_birth_year("2003"));
        assert_eq!(false, validate_birth_year("2oo2"));
    }

    #[test]
    fn test_validate_issue_year() {
        assert_eq!(false, validate_issue_year("2009"));
        assert_eq!(true, validate_issue_year("2010"));
        assert_eq!(true, validate_issue_year("2020"));
        assert_eq!(false, validate_issue_year("2021"));
    }

    #[test]
    fn test_validate_expiration_year() {
        assert_eq!(false, validate_expiration_year("2019"));
        assert_eq!(true, validate_expiration_year("2020"));
        assert_eq!(true, validate_expiration_year("2030"));
        assert_eq!(false, validate_expiration_year("2031"));
    }

    #[test]
    fn test_validate_height() {
        assert_eq!(false, validate_height("149cm"));
        assert_eq!(true, validate_height("150cm"));
        assert_eq!(true, validate_height("193cm"));
        assert_eq!(false, validate_height("194cm"));

        assert_eq!(false, validate_height("58in"));
        assert_eq!(true, validate_height("59in"));
        assert_eq!(true, validate_height("76in"));
        assert_eq!(false, validate_height("77in"));

        assert_eq!(false, validate_height("190"));
    }

    #[test]
    fn test_validate_hair_color() {
        assert_eq!(true, validate_hair_color("#123abc"));
        assert_eq!(false, validate_hair_color("#123abz"));
        assert_eq!(false, validate_hair_color("#13abc"));
        assert_eq!(false, validate_hair_color("123abc"));
        assert_eq!(false, validate_hair_color("123ab"));
    }

    #[test]
    fn test_validate_eye_color() {
        assert_eq!(true, validate_eye_color("amb"));
        assert_eq!(true, validate_eye_color("blu"));
        assert_eq!(true, validate_eye_color("brn"));
        assert_eq!(true, validate_eye_color("gry"));
        assert_eq!(true, validate_eye_color("grn"));
        assert_eq!(true, validate_eye_color("hzl"));
        assert_eq!(true, validate_eye_color("oth"));
        assert_eq!(false, validate_eye_color("ptr"));
    }

    #[test]
    fn test_passport_id() {
        assert_eq!(true, validate_passport_id("000000001"));
        assert_eq!(false, validate_passport_id("0123456789"));
        assert_eq!(false, validate_passport_id("i23456789"));
    }
}
