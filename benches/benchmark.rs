use criterion::{criterion_group, criterion_main, Criterion};

extern crate calendar;

use calendar::{
    day::Day, eight::Eight, eleven::Eleven, five::Five, four::Four, nine::Nine, one::One,
    seven::Seven, six::Six, ten::Ten, three::Three, two::Two, twelve::Twelve,
};

macro_rules! bench_day {
    ($t:ty, $c:expr) => {
        let input = <$t>::get_input();
        $c.bench_function(&format!("Day {} Part 1", stringify!($t)), |b| {
            b.iter(|| <$t>::part_one(&input))
        });
        $c.bench_function(&format!("Day {} Part 2", stringify!($t)), |b| {
            b.iter(|| <$t>::part_two(&input))
        });
    };
}

pub fn day_one(c: &mut Criterion) {
    bench_day!(One, c);
}

pub fn day_two(c: &mut Criterion) {
    bench_day!(Two, c);
}

pub fn day_three(c: &mut Criterion) {
    bench_day!(Three, c);
}

pub fn day_four(c: &mut Criterion) {
    bench_day!(Four, c);
}

pub fn day_five(c: &mut Criterion) {
    bench_day!(Five, c);
}

pub fn day_six(c: &mut Criterion) {
    bench_day!(Six, c);
}

pub fn day_seven(c: &mut Criterion) {
    bench_day!(Seven, c);
}

pub fn day_eight(c: &mut Criterion) {
    bench_day!(Eight, c);
}

pub fn day_nine(c: &mut Criterion) {
    bench_day!(Nine, c);
}

pub fn day_ten(c: &mut Criterion) {
    bench_day!(Ten, c);
}

pub fn day_eleven(c: &mut Criterion) {
    bench_day!(Eleven, c);
}

pub fn day_twelve(c: &mut Criterion) {
    bench_day!(Twelve, c);
}

criterion_group!(
    benches, day_one, day_two, day_three, day_four, day_five, day_six, day_seven, day_eight,
    day_nine, day_ten, day_eleven, day_twelve,
);
criterion_main!(benches);
