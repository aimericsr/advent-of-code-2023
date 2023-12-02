use std::fs::File;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day_01::part1::process;

fn part1(c: &mut Criterion) {
    let input = File::open("part1.txt").expect("Can't read the file");

    c.bench_function("part1", move |b| {
        b.iter_batched(|| &input, |data| process(data), BatchSize::SmallInput)
    });
}

criterion_group!(benches, part1);
criterion_main!(benches);
