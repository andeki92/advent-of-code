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
aoc new year 2024

# Create day solution
aoc new day 1 -y 2024

# Download input
aoc download 1 -y 2024

# Test on example input
cargo test -p aoc-2024 --bin day01

# Run on real input
cargo run -p aoc-2024 --bin day01 --release

# Submit answers
aoc submit 1 1 "answer" -y 2024
aoc submit 1 2 "answer" -y 2024
```

## Commands

| Command                        | Description                   |
| ------------------------------ | ----------------------------- |
| `aoc new year 2024`            | Create year from template     |
| `aoc new day 1 -y 2024`        | Create day01.rs from template |
| `aoc download 1 -y 2024`       | Download puzzle input         |
| `aoc submit 1 1 "ans" -y 2024` | Submit part 1 answer          |
| `aoc auth "cookie"`            | Save session cookie           |

## Tips

- `common/` has utilities (gcd, lcm, parsing) - use if helpful
- Inputs are personal - don't commit them (already gitignored)
