use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! define_full_benches {
    ($($problem:ident),*$(,)?) => {
        pub fn ec_benchmark(c: &mut Criterion) {
            $(c.bench_function(stringify!($problem), |b| b.iter(|| black_box($problem::solve())));)+
            c.bench_function("all", |b| b.iter(|| ($(black_box($problem::solve())),+)));
        }

        criterion_group! {
            name = full_benches;

            config = Criterion::default();

            targets = ec_benchmark
        }

    };
}

macro_rules! define_part_benches {
    ($($problem:ident),*$(,)?) => {
        pub fn ec_benchmark2(c: &mut Criterion) {
            $(c.bench_function(concat!(stringify!($problem), "/part1"), |b| b.iter(|| black_box($problem::solve_part1())));)+
            $(c.bench_function(concat!(stringify!($problem), "/part2"), |b| b.iter(|| black_box($problem::solve_part2())));)+
            $(c.bench_function(concat!(stringify!($problem), "/part3"), |b| b.iter(|| black_box($problem::solve_part3())));)+
        }

        criterion_group! {
            name = part_benches;

            config = Criterion::default();

            targets = ec_benchmark2
        }

    };
}

#[rustfmt::skip]
define_part_benches!(
    problem01,
);

criterion_main!(full_benches, part_benches);

#[rustfmt::skip]
define_full_benches!(
    problem01,
);
