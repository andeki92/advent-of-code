# Advent of Code 2024

Solutions for [Advent of Code 2024](https://adventofcode.com/2024).

## Progress

| Day                                        | Part 1 | Part 2 | Solution                     | Benchmark |
| ------------------------------------------ | ------ | ------ | ---------------------------- | --------- |
| [01](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ | [day01.rs](src/bin/day01.rs) | - |
| [02](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ | [day02.rs](src/bin/day02.rs) | - |
| [03](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ | [day03.rs](src/bin/day03.rs) | - |
| [04](https://adventofcode.com/2024/day/4) | ⭐ | ⭐ | [day04.rs](src/bin/day04.rs) | - |
| [05](https://adventofcode.com/2024/day/5) | ⭐ | ⭐ | [day05.rs](src/bin/day05.rs) | - |
| [06](https://adventofcode.com/2024/day/6) | ⭐ | ⭐ | [day06.rs](src/bin/day06.rs) | - |
| [07](https://adventofcode.com/2024/day/7) |  |  |  |  |
| [08](https://adventofcode.com/2024/day/8) |  |  |  |  |
| [09](https://adventofcode.com/2024/day/9) |  |  |  |  |
| [10](https://adventofcode.com/2024/day/10) |  |  |  |  |
| [11](https://adventofcode.com/2024/day/11) |  |  |  |  |
| [12](https://adventofcode.com/2024/day/12) |  |  |  |  |
| [13](https://adventofcode.com/2024/day/13) |  |  |  |  |
| [14](https://adventofcode.com/2024/day/14) |  |  |  |  |
| [15](https://adventofcode.com/2024/day/15) |  |  |  |  |
| [16](https://adventofcode.com/2024/day/16) |  |  |  |  |
| [17](https://adventofcode.com/2024/day/17) |  |  |  |  |
| [18](https://adventofcode.com/2024/day/18) |  |  |  |  |
| [19](https://adventofcode.com/2024/day/19) |  |  |  |  |
| [20](https://adventofcode.com/2024/day/20) |  |  |  |  |
| [21](https://adventofcode.com/2024/day/21) |  |  |  |  |
| [22](https://adventofcode.com/2024/day/22) |  |  |  |  |
| [23](https://adventofcode.com/2024/day/23) |  |  |  |  |
| [24](https://adventofcode.com/2024/day/24) |  |  |  |  |
| [25](https://adventofcode.com/2024/day/25) |  |  |  |  |

**Total: 12/50 ⭐**

> This progress table is automatically updated when you submit correct solutions using `aoc submit`.
> You can also manually update it by running `aoc status -y 2024` from the repository root.

## Quick Commands

### Create a new day

```bash
# Using the custom CLI tool (from repository root)
aoc new day 1

# Manual approach:
cp src/bin/template.rs src/bin/day01.rs
# Then edit day01.rs and replace XX with 01
```

### Download input

```bash
# Using the custom CLI tool (from repository root)
aoc download 1
```

### Run solution

```bash
cargo run --bin day01              # Debug mode
cargo run --bin day01 --release    # Optimized mode
```

### Test solution

```bash
cargo test --bin day01
```

### Benchmark

First, set up benchmarks:

1. Add criterion to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

2. Expose day modules in `src/lib.rs`:

```rust
pub mod day01;
pub mod day02;
```

3. Create `benches/benchmarks.rs`:

```rust
use aoc_common::aoc_bench;
use criterion::{criterion_group, criterion_main};

aoc_bench!(day01);
aoc_bench!(day02);

criterion_group!(benches, day01, day02);
criterion_main!(benches);
```

Then run:

```bash
cargo bench
```

## Workflow

1. **Create new day file** (from repository root)

   ```bash
   aoc new day 1
   ```

2. **Update the file**
   - Add example input to `data/examples/01.txt`
   - Update test with expected result

3. **Download input** (from repository root)

   ```bash
   aoc download 1
   ```

4. **Implement solution** (from 2024 directory)

   ```bash
   cd 2024
   # Implement `part1()` function
   # Run tests: cargo test --bin day01
   # Run solution: cargo run --bin day01
   ```

5. **Submit answer** (from repository root)

   ```bash
   aoc submit 1 1 <answer>  # Submit part 1
   aoc submit 1 2 <answer>  # Submit part 2
   ```

6. **Optimize if needed**
   - Run benchmarks: `cargo bench`
   - Profile with `perf`, `flamegraph`, etc.
   - Optimize and iterate

## Project Structure

```
2024/
├── src/
│   ├── bin/
│   │   ├── template.rs    # Template for new days (removed after setup)
│   │   ├── day01.rs       # Day 1 solution
│   │   └── ...
│   └── lib.rs             # Minimal library for benchmarking
├── data/
│   ├── inputs/            # Puzzle inputs (gitignored)
│   └── examples/          # Example inputs for tests
└── Cargo.toml
```
