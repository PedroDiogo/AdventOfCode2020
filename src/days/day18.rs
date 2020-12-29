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

    let sum_of_results: isize = inputs
        .lines()
        .map(eval_expression_with_add_precedence)
        .filter_map(|result| result.parse::<isize>().ok())
        .sum();

    let part_two = Some(sum_of_results.to_string());

    (part_one, part_two)
}

fn eval_expression(expression: &str) -> String {
    let find_first_operation_idx: fn(&[Element]) -> usize = |_| 3;

    let expression = simplify_expression(expression, eval_expression);
    solve_expression(&expression, find_first_operation_idx).to_string()
}

fn eval_expression_with_add_precedence(expression: &str) -> String {
    let expression = simplify_expression(expression, eval_expression_with_add_precedence);
    solve_expression(&expression, find_next_add_idx).to_string()
}

fn find_next_add_idx(elements: &[Element]) -> usize {
    elements
        .iter()
        .enumerate()
        .find(|(_, x)| x.element_type == ElementType::ADD)
        .map_or(3, |(idx, _)| idx + 2)
}

fn simplify_expression(expression: &str, eval_function: fn(&str) -> String) -> String {
    if RE.is_match(expression) {
        let new_expression = RE
            .captures_iter(expression)
            .map(|capture| capture.name("inner_expression").unwrap().as_str())
            .dedup()
            .fold(expression.to_string(), |new_expression, capture| {
                let capture_with_brackets = format!("({})", capture);
                new_expression.replace(
                    capture_with_brackets.as_str(),
                    eval_function(capture).as_str(),
                )
            });
        simplify_expression(&new_expression, eval_function)
    } else {
        expression.to_string()
    }
}

fn solve_expression(expression: &str, next_index: fn(&[Element]) -> usize) -> isize {
    let mut expr_vec: Vec<Element> = expression
        .split_whitespace()
        .filter_map(|x| Element::from_str(x))
        .collect();

    while expr_vec.len() > 1 {
        let idx = next_index(&expr_vec);
        let (left, right) = expr_vec.split_at(idx);
        let new_element = Element::eval(&left[idx - 3], &left[idx - 2], &left[idx - 1]).unwrap();

        expr_vec = vec![left[0..idx - 3].to_vec(), vec![new_element], right.to_vec()].concat();
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
        assert_eq!("71", eval_expression("1 + 2 * 3 + 4 * 5 + 6"));
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
    fn test_eval_expression_with_add_precedence() {
        assert_eq!(
            "231",
            eval_expression_with_add_precedence("1 + 2 * 3 + 4 * 5 + 6")
        );
        assert_eq!(
            "51",
            eval_expression_with_add_precedence("1 + (2 * 3) + (4 * (5 + 6))")
        );
        assert_eq!("46", eval_expression_with_add_precedence("2 * 3 + (4 * 5)"));
        assert_eq!(
            "1445",
            eval_expression_with_add_precedence("5 + (8 * 3 + 9 + 3 * 4 * 3)")
        );
        assert_eq!(
            "669060",
            eval_expression_with_add_precedence("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            "23340",
            eval_expression_with_add_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn test_simplify_expression() {
        assert_eq!(
            "6810 + 2 + 4 * 2",
            simplify_expression(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                eval_expression
            )
        );
        assert_eq!("11", eval_expression("((5 + 6))"));
    }

    #[test]
    fn test_solve_expression() {
        assert_eq!(71, solve_expression("1 + 2 * 3 + 4 * 5 + 6", |_| 3));
        assert_eq!(
            231,
            solve_expression("1 + 2 * 3 + 4 * 5 + 6", find_next_add_idx)
        );
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
