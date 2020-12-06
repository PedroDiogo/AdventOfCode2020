#![warn(clippy::all)]
use std::fs;

pub fn read_inputs(filename: &str) -> String {
    fs::read_to_string(filename).expect("Couldn't read file")
}

pub trait LinesOf {
    fn lines_of<T: std::str::FromStr>(&self) -> Vec<Option<T>>;
}

impl LinesOf for str {
    fn lines_of<T: std::str::FromStr>(&self) -> Vec<Option<T>> {
        self.lines().map(|line| line.parse::<T>().ok()).collect()
    }
}

pub trait SplitByBlankLines {
    fn split_by_blank_lines(&self) -> std::str::Split<&str>;
}

impl SplitByBlankLines for str {
    fn split_by_blank_lines(&self) -> std::str::Split<&str> {
        self.split("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lines_of_i64() {
        let input = "1
2
3
a";
        assert_eq!(
            vec![Some(1), Some(2), Some(3), None],
            input.lines_of::<i64>()
        );
        assert_eq!(
            vec![Some(1), Some(2), Some(3), None],
            input.to_string().lines_of::<i64>()
        );
        assert_eq!(
            vec![Some(1), Some(2), Some(3), None],
            input.lines_of::<usize>()
        );
    }

    #[test]
    fn test_split_by_blank_lines() {
        let input = "ab

cd";
        assert_eq!(
            vec!["ab", "cd"],
            input.split_by_blank_lines().collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["ab", "cd"],
            input
                .to_string()
                .split_by_blank_lines()
                .collect::<Vec<&str>>()
        );
    }
}
