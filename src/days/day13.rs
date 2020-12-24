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
    let buses: Vec<(usize, usize)> = inputs
        .next()
        .expect("Expected to have buses")
        .split(',')
        .enumerate()
        .filter(|x| x.1.parse::<usize>().is_ok())
        .map(|x| (x.0, x.1.parse::<usize>().unwrap()))
        .collect();

    let bus_numbers: Vec<usize> = buses.iter().map(|x| x.1).collect();

    let (first_bus_id, waiting_minutes) = find_first_bus(&offset, &bus_numbers);
    let part_one = Some((first_bus_id * waiting_minutes).to_string());

    let chinese_remainder_theorem_inputs: Vec<(isize, isize)> = buses
        .iter()
        .map(|bus| (-(bus.0 as isize), bus.1 as isize))
        .collect();
    let (min_timestamp, _) = chinese_remainder_theorem(&chinese_remainder_theorem_inputs);
    let part_two = Some(min_timestamp.to_string());

    (part_one, part_two)
}

fn find_first_bus(offset: &usize, buses: &[usize]) -> (usize, usize) {
    buses
        .iter()
        .map(|bus| (*bus, bus - (offset % bus)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
}

fn extended_gcd(a: &isize, b: &isize) -> (isize, isize, isize) {
    let mut s = 0;
    let mut r = *b;
    let mut old_s = 1;
    let mut old_r = *a;

    while r != 0 {
        let quocient = old_r / r;

        let new_r = old_r - (quocient * r);
        let new_s = old_s - (quocient * s);

        old_r = r;
        old_s = s;

        r = new_r;
        s = new_s;
    }

    let bezout_t = match b {
        0 => 0,
        _ => ((old_r - old_s) * a) / b,
    };
    (old_s, bezout_t, old_r)
}

fn chinese_remainder_theorem(inputs: &[(isize, isize)]) -> (isize, isize) {
    let big_n: isize = inputs.iter().map(|x| x.1).product();

    let a: isize = inputs
        .iter()
        .map(|(a_i, n_i)| {
            let big_n_i = big_n / n_i;
            let (big_m_i, _, _) = extended_gcd(&big_n_i, n_i);

            a_i * big_m_i * big_n_i
        })
        .sum();

    match a % big_n {
        a if a >= 0 => (a, big_n),
        a => (a + big_n, big_n),
    }
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

    #[test]
    fn test_chinese_remainder_theorem() {
        let (a, _) = chinese_remainder_theorem(&[(0, 7), (-1, 13), (-4, 59), (-6, 31), (-7, 19)]);
        assert_eq!(1068781, a);
        let (a, _) = chinese_remainder_theorem(&[(0, 17), (-2, 13), (-3, 19)]);
        assert_eq!(3417, a);
        let (a, _) = chinese_remainder_theorem(&[(0, 67), (-1, 7), (-2, 59), (-3, 61)]);
        assert_eq!(754018, a);
        let (a, _) = chinese_remainder_theorem(&[(0, 67), (-2, 7), (-3, 59), (-4, 61)]);
        assert_eq!(779210, a);
        let (a, _) = chinese_remainder_theorem(&[(0, 67), (-1, 7), (-3, 59), (-4, 61)]);
        assert_eq!(1261476, a);
        let (a, _) = chinese_remainder_theorem(&[(0, 1789), (-1, 37), (-2, 47), (-3, 1889)]);
        assert_eq!(1202161486, a);
    }
}
