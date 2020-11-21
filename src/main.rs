pub mod days;
use days::*;
use std::env;

fn main() {
    match get_day() {
        _ => println!("Unknown day - {}", get_day())
    }
}

fn get_day() -> i32 {
    return env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Need to pass one argument as the day number. Example: cargo run 1")
        .parse::<i32>()
        .expect("Expecting first argument to be an integer");
}