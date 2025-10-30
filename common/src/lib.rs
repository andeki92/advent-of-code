// Common utilities shared across all Advent of Code years
//
// Add utilities here that are genuinely useful across multiple years.
// Keep this minimal - most logic should stay in individual days.

pub mod benchmark;

// Example utilities (add as needed):

/// Parse a grid of characters from input
pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Transpose a grid of chars
pub fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    if grid.is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut transposed = vec![vec![' '; rows]; cols];

    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            transposed[x][y] = ch;
        }
    }

    transposed
}

/// Parse a list of numbers (one per line)
pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

/// Greatest Common Divisor
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

/// Least Common Multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
    }
}
