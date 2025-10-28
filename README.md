# 🎄 Advent of Code in Rust 🎄

Multi-year Advent of Code workspace with custom tooling, shared utilities, and centralized benchmarks.

## Setup

**1. Build CLI:**

```bash
cargo build --release -p aoc-cli
```

**2. Get session cookie:**

- Go to https://adventofcode.com (logged in)
- F12 → Application → Cookies → copy `session` value

**3. Save credentials:**

```bash
./target/release/aoc auth "your_session_cookie"
```

**4. Add to PATH (optional):**

```bash
# macOS/Linux (requires password)
sudo ln -sf "$(pwd)/target/release/aoc" /usr/local/bin/aoc

# Then use 'aoc' instead of './target/release/aoc'
```

## Daily Workflow

```bash
# Create year (first time only)
aoc new-year 2024

# Create day solution
aoc scaffold 1 -y 2024

# Download input
aoc download 1 -y 2024

# Implement solution in 2024/src/bin/day01.rs
cd 2024
cargo test --bin day01           # Test with example
cargo run --bin day01 --release  # Run with real input

# Submit
cd ..
aoc submit 1 1 "answer" -y 2024
aoc submit 1 2 "answer" -y 2024
```

## Commands

| Command                        | Description                   |
| ------------------------------ | ----------------------------- |
| `aoc new-year 2024`            | Create year from template     |
| `aoc scaffold 1 -y 2024`       | Create day01.rs from template |
| `aoc download 1 -y 2024`       | Download puzzle input         |
| `aoc submit 1 1 "ans" -y 2024` | Submit part 1 answer          |
| `aoc auth "cookie"`            | Save session cookie           |

## Structure

```
advent-of-code/
├── common/      # Shared utilities (gcd, lcm, parsing)
├── aoc-cli/     # Custom CLI tool
├── template/    # Year template (YEAR placeholder)
├── 2024/        # Year-specific solutions
├── benches/     # Centralized benchmarks (all years)
├── viz/         # Visualization framework (optional)
└── Cargo.toml   # Workspace config
```

Each year contains:

- `src/bin/` - Daily solutions (day01.rs, day02.rs, ...)
- `src/lib.rs` - Public modules for benchmarking
- `data/inputs/` - Your puzzle inputs (gitignored)
- `data/examples/` - Example inputs for testing

## Benchmarking

Benchmarks run from root and cover all years!

**1. Expose solution in year's `src/lib.rs`:**

```rust
pub mod day01 {
    pub fn part1(input: &str) -> usize { /* ... */ }
    pub fn part2(input: &str) -> usize { /* ... */ }
}
```

## Tips

- `common/` has utilities (gcd, lcm, parsing) - use if helpful
- Each day can still be self-contained if preferred
- Add dependencies per-day in year's `Cargo.toml` if needed
- Inputs are personal - don't commit them (already gitignored)
