/// Simple macro for benchmarking AoC solutions
///
/// Usage in your year's `benches/benchmarks.rs`:
/// ```rust,ignore
/// use aoc_common::aoc_bench;
/// use criterion::{criterion_group, criterion_main};
///
/// benchmark!(day01);
/// benchmark!(day02);
///
/// criterion_group!(benches, day01, day02);
/// criterion_main!(benches);
/// ```
///
/// Requirements:
/// - Day modules must be exposed in lib.rs: `pub mod day01;`
/// - Each day must have `pub fn part1(input: &str)` and `pub fn part2(input: &str)`
/// - Input files at `data/inputs/{day}.txt`
#[macro_export]
macro_rules! benchmark {
    ($day:ident) => {
        fn $day(c: &mut criterion::Criterion) {
            let day_num = stringify!($day).trim_start_matches("day");
            let input = include_str!(concat!("../data/inputs/", day_num, ".txt"));

            c.bench_function(&format!("{}/part1", stringify!($day)), |b| {
                b.iter(|| {
                    use criterion::black_box;
                    crate::$day::part1(black_box(input))
                })
            });

            c.bench_function(&format!("{}/part2", stringify!($day)), |b| {
                b.iter(|| {
                    use criterion::black_box;
                    crate::$day::part2(black_box(input))
                })
            });
        }
    };
}
