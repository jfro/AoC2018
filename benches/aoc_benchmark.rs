#[macro_use]
extern crate criterion;
extern crate aoc;

use criterion::Criterion;
use crate::aoc::days;



fn bench_day1(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| days::day1::run(1)));
    c.bench_function("day 1 part 2", |b| b.iter(|| days::day1::run(2)));
}

fn bench_day2(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| days::day2::run(1)));
    c.bench_function("day 2 part 2", |b| b.iter(|| days::day2::run(2)));
}

fn bench_day3(c: &mut Criterion) {
    c.bench_function("day 3 part 1", |b| b.iter(|| days::day3::run(1)));
    c.bench_function("day 3 part 2", |b| b.iter(|| days::day3::run(2)));
}

fn bench_day4(c: &mut Criterion) {
    c.bench_function("day 4 part 1", |b| b.iter(|| days::day4::run(1)));
    c.bench_function("day 4 part 2", |b| b.iter(|| days::day4::run(2)));
}

fn bench_day5(c: &mut Criterion) {
    c.bench_function("day 5 part 1", |b| b.iter(|| days::day5::bench(1)));
    // c.bench_function("day 5 part 2", |b| b.iter(|| days::day5::run(2)));
}

criterion_group!(benches, bench_day1, bench_day2, bench_day3, bench_day4, bench_day5);
criterion_main!(benches);
