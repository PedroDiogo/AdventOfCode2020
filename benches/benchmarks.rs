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

criterion_group!(benches, day1, day2, day3, day4, day5, day6);
criterion_main!(benches);
