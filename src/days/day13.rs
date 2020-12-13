use super::lib::*;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day13.txt";
    let inputs = read_inputs(&filename);
    let mut inputs = inputs.lines();
    let offset = inputs
        .next()
        .expect("Expected an offset")
        .parse::<usize>()
        .expect("Expect offset to be usize");
    let buses: Vec<usize> = inputs
        .next()
        .expect("Expected to have buses")
        .split(',')
        .filter_map(|bus| bus.parse::<usize>().ok())
        .collect();

    let (first_bus_id, waiting_minutes) = find_first_bus(&offset, &buses);
    let part_one = Some((first_bus_id * waiting_minutes).to_string());
    let part_two = None;

    (part_one, part_two)
}

fn find_first_bus(offset: &usize, buses: &[usize]) -> (usize, usize) {
    buses
        .iter()
        .map(|bus| (*bus, bus - (offset % bus)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_bus() {
        let (bus_id, waiting_minutes) = find_first_bus(&939, &[7, 13, 59, 31, 19]);

        assert_eq!(59, bus_id);
        assert_eq!(5, waiting_minutes);
    }
}
