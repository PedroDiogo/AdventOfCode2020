use super::lib::*;
use std::ops::{Add, Mul};

extern crate regex;
use self::regex::Regex;

extern crate itertools;
use self::itertools::*;
lazy_static! {
    static ref RE: Regex = Regex::new(r"\((?P<inner_expression>[\d*+ ]+)\)").unwrap();
}

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day18.txt";
    let inputs = read_inputs(&filename);

    let sum_of_results: isize = inputs
        .lines()
        .map(eval_expression)
        .filter_map(|result| result.parse::<isize>().ok())
        .sum();

    let part_one = Some(sum_of_results.to_string());
    let part_two = None;

    (part_one, part_two)
}

fn eval_expression(expression: &str) -> String {
    let expression = simplify_expression(expression);
    solve_simple_expression(&expression).to_string()
}

fn simplify_expression(expression: &str) -> String {
    if RE.is_match(expression) {
        let new_expression = RE
            .captures_iter(expression)
            .map(|capture| capture.name("inner_expression").unwrap().as_str())
            .dedup()
            .fold(expression.to_string(), |new_expression, capture| {
                let capture_with_brackets = format!("({})", capture);
                new_expression.replace(
                    capture_with_brackets.as_str(),
                    eval_expression(capture).as_str(),
                )
            });
        simplify_expression(&new_expression)
    } else {
        expression.to_string()
    }
}

fn solve_simple_expression(expression: &str) -> isize {
    let mut expr_vec: Vec<Element> = expression
        .split_whitespace()
        .filter_map(|x| Element::from_str(x))
        .collect();

    while expr_vec.len() > 1 {
        let (left, right) = expr_vec.split_at(3);
        let left = Element::eval(&left[0], &left[1], &left[2]).unwrap();

        expr_vec = vec![vec![left], right.to_vec()].concat();
    }
    expr_vec[0].value.unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Element {
    element_type: ElementType,
    value: Option<isize>,
}

impl Element {
    fn new_number(value: isize) -> Self {
        Element {
            element_type: ElementType::NUMBER,
            value: Some(value),
        }
    }

    fn new_add() -> Self {
        Element {
            element_type: ElementType::ADD,
            value: None,
        }
    }

    fn new_mul() -> Self {
        Element {
            element_type: ElementType::MULTIPLY,
            value: None,
        }
    }

    fn from_str(input: &str) -> Option<Self> {
        let input = input.trim();
        match input {
            "+" => Some(Self::new_add()),
            "*" => Some(Self::new_mul()),
            x => x.parse::<isize>().ok().map(Self::new_number),
        }
    }

    fn eval(left: &Element, op: &Element, right: &Element) -> Option<Element> {
        match (left.element_type, op.element_type, right.element_type) {
            (ElementType::NUMBER, ElementType::ADD, ElementType::NUMBER) => *left + *right,
            (ElementType::NUMBER, ElementType::MULTIPLY, ElementType::NUMBER) => *left * *right,
            _ => None,
        }
    }
}

impl Add for Element {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Option<Self> {
        match (self.element_type, other.element_type) {
            (ElementType::NUMBER, ElementType::NUMBER) => Some(Element::new_number(
                self.value.unwrap() + other.value.unwrap(),
            )),
            _ => None,
        }
    }
}

impl Mul for Element {
    type Output = Option<Self>;
    fn mul(self, other: Self) -> Option<Self> {
        match (self.element_type, other.element_type) {
            (ElementType::NUMBER, ElementType::NUMBER) => Some(Element::new_number(
                self.value.unwrap() * other.value.unwrap(),
            )),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ElementType {
    NUMBER,
    ADD,
    MULTIPLY,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_expression() {
        assert_eq!("71", &eval_expression("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!("11", eval_expression("(5 + 6)"));
        assert_eq!("51", eval_expression("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!("26", eval_expression("2 * 3 + (4 * 5)"));
        assert_eq!("437", eval_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            "12240",
            eval_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            "13632",
            eval_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
    #[test]
    fn test_simplify_expression() {
        assert_eq!(
            "6810 + 2 + 4 * 2",
            simplify_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn test_solve_simple_expression() {
        assert_eq!(71, solve_simple_expression("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_element_eval() {
        assert_eq!(
            Some(Element::new_number(15)),
            Element::eval(
                &Element::new_number(7),
                &Element::new_add(),
                &Element::new_number(8)
            )
        );
        assert_eq!(
            Some(Element::new_number(56)),
            Element::eval(
                &Element::new_number(7),
                &Element::new_mul(),
                &Element::new_number(8)
            )
        );
        assert_eq!(
            None,
            Element::eval(
                &Element::new_number(7),
                &Element::new_number(7),
                &Element::new_number(8)
            )
        );
    }

    #[test]
    fn test_element_from_str() {
        assert_eq!(Some(Element::new_mul()), Element::from_str(" * "));
        assert_eq!(Some(Element::new_add()), Element::from_str(" + "));
        assert_eq!(Some(Element::new_number(15)), Element::from_str("15"));
        assert_eq!(Some(Element::new_number(-15)), Element::from_str("-15"));
        assert_eq!(None, Element::from_str(" / "));
    }
}
