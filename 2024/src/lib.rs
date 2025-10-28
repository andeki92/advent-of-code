// Minimal library file for Advent of Code YEAR
//
// This is intentionally kept minimal to avoid creating shared utilities.
// Each day's solution should be self-contained in its own binary.
//
// However, to enable benchmarking, you can optionally expose day modules here:
//
// pub mod day01;
// pub mod day02;
// ...
//
// Each module should contain public part1() and part2() functions.
// The corresponding bin file can then re-export and use these functions.

#[cfg(test)]
mod test_utils {
    // Add any absolutely essential test utilities here
    // Example: common assertion helpers, test input loaders, etc.
}
