extern crate advent_of_code_2020;
extern crate criterion;

use advent_of_code_2020::days::*;
use criterion::*;

macro_rules! benchmark {
    ($name:ident, $fn:expr) => {
        pub fn $name(c: &mut Criterion) {
            c.bench_function(stringify!($name), |b| b.iter(|| $fn()));
        }
    };
}

benchmark!(day1, day1::run);
benchmark!(day2, day2::run);
benchmark!(day3, day3::run);
benchmark!(day4, day4::run);
benchmark!(day5, day5::run);
benchmark!(day6, day6::run);
benchmark!(day7, day7::run);
benchmark!(day8, day8::run);
benchmark!(day9, day9::run);
benchmark!(day10, day10::run);
benchmark!(day11, day11::run);
benchmark!(day12, day12::run);
benchmark!(day13, day13::run);
benchmark!(day14, day14::run);
benchmark!(day15, day15::run);
benchmark!(day16, day16::run);
benchmark!(day17, day17::run);
benchmark!(day18, day18::run);
benchmark!(day19, day19::run);

criterion_group!(
    benches, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19
);
criterion_main!(benches);
