extern crate advent;
use self::advent::*;

pub fn run() {
    let filename = "inputs/day5.txt";
    let inputs = read_inputs(&filename);

    let seat_ids = inputs
        .lines()
        .map(|line| convert_boarding_pass_to_binary(line))
        .filter_map(|line| convert_from_binary(&line).ok());

    let part_one = seat_ids.max().unwrap();
    println!("Part one: {}", part_one);
}

fn convert_boarding_pass_to_binary(line: &str) -> String {
    line.replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1")
}

fn convert_from_binary(line: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(line, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_boarding_pass_to_binary() {
        assert_eq!("0101100", convert_boarding_pass_to_binary("FBFBBFF"));
    }

    #[test]
    fn test_convert_from_binary() -> Result<(), std::num::ParseIntError> {
        assert_eq!(4, convert_from_binary("0100")?);
        assert_eq!(44, convert_from_binary("0101100")?);
        Ok(())
    }
}
