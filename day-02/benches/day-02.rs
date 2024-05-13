use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day_02::part1::process;

fn part1(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/part1-real.txt").expect("Can't read the file");

    c.bench_function("part1", |b| {
        b.iter_batched(
            || &input,
            |data| process(data.clone()),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, part1);
criterion_main!(benches);
