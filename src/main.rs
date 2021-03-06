#![warn(clippy::all)]
#[macro_use]
extern crate scan_fmt;

#[macro_use]
extern crate lazy_static;

mod days;
use days::*;
use std::env;

fn main() {
    let day = get_day();
    let (part_one, part_two) = match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        11 => day11::run(),
        12 => day12::run(),
        13 => day13::run(),
        14 => day14::run(),
        15 => day15::run(),
        16 => day16::run(),
        17 => day17::run(),
        18 => day18::run(),
        19 => day19::run(),
        20 => day20::run(),
        21 => day21::run(),
        _ => (None, None),
    };

    match (part_one, part_two) {
        (None, None) => println!("Unknown day - {}", day),
        (Some(part_one), None) => println!("# Day {}:\n  Part one: {}", day, part_one),
        (None, Some(part_two)) => println!("# Day {}:\n  Part two: {}", day, part_two),
        (Some(part_one), Some(part_two)) => println!(
            "# Day {}:\n  Part one: {}\n  Part two: {}",
            day, part_one, part_two
        ),
    };
}

fn get_day() -> i32 {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Need to pass one argument as the day number. Example: cargo run 1")
        .parse::<i32>()
        .expect("Expecting first argument to be an integer")
}
