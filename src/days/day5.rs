use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day5.txt";
    let inputs = read_inputs(&filename);

    let seat_ids = inputs
        .lines()
        .map(|line| convert_boarding_pass_to_binary(line))
        .filter_map(|line| convert_from_binary(&line).ok());

    let part_one = seat_ids.clone().max().unwrap();
    let part_two = find_gap(&seat_ids.collect::<Vec<usize>>()).unwrap();

    (Some(part_one.to_string()), Some(part_two.to_string()))
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

fn find_gap(list: &[usize]) -> Option<usize> {
    let mut sorted_list = list.to_owned();
    sorted_list.sort_unstable();

    let mut gap = None;
    for idx in 1..sorted_list.len() {
        let current = *sorted_list.get(idx).unwrap();
        let previous = *sorted_list.get(idx - 1).unwrap();

        if current != previous + 1 {
            gap = Some(previous + 1);
            break;
        }
    }
    gap
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

    #[test]
    fn test_find_gap() {
        assert_eq!(Some(4), find_gap(&[1, 2, 3, 5, 6]));
        assert_eq!(Some(4), find_gap(&[1, 2, 3, 5]));
    }
}
