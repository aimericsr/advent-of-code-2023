use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day_04::part1::process;

fn part1(c: &mut Criterion) {
    let input = include_str!("../data/part1-real2.txt");

    c.bench_function("part1-real", move |b| {
        b.iter_batched(|| &input, |data| process(data), BatchSize::SmallInput)
    });
}

criterion_group!(benches, part1);
criterion_main!(benches);
