use std::time::Instant;

fn main() {
    let input = include_str!("../../data/inputs/04.txt");

    let start = Instant::now();
    let part1_result = part1(input);
    let part1_time = start.elapsed();
    println!("Part 1: {} ({:?})", part1_result, part1_time);

    let start = Instant::now();
    let part2_result = part2(input);
    let part2_time = start.elapsed();
    println!("Part 2: {} ({:?})", part2_result, part2_time);

    println!("\nTotal time: {:?}", part1_time + part2_time);
}

/// Patterns to search for "XMAS" in all four directions (horizontal, vertical, both diagonals)
const XMAS_PATTERNS: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], // Horizontal →
    [(0, 0), (0, 1), (0, 2), (0, 3)], // Vertical ↓
    [(0, 0), (1, 1), (2, 2), (3, 3)], // Diagonal ↘
    [(3, 0), (2, 1), (1, 2), (0, 3)], // Diagonal ↙
];

/// Two diagonals forming an X pattern for "MAS" search
const X_PATTERNS: [[(usize, usize); 3]; 2] = [
    [(0, 0), (1, 1), (2, 2)], // Diagonal ↘
    [(2, 0), (1, 1), (0, 2)], // Diagonal ↙
];

fn part1(input: &str) -> usize {
    let grid = aoc_common::parse_char_grid(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            for pattern in XMAS_PATTERNS {
                if let Some(word) = extract_pattern(&grid, x, y, &pattern) {
                    if word == "XMAS" || word == "SAMX" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let grid = aoc_common::parse_char_grid(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let all_diagonals_match = X_PATTERNS.iter().all(|pattern| {
                if let Some(word) = extract_pattern(&grid, x, y, pattern) {
                    word == "MAS" || word == "SAM"
                } else {
                    false
                }
            });

            if all_diagonals_match {
                count += 1;
            }
        }
    }

    count
}

/// Extract characters from the grid following a pattern of offsets
fn extract_pattern(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    pattern: &[(usize, usize)],
) -> Option<String> {
    pattern
        .iter()
        .map(|&(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            grid.get(ny).and_then(|row| row.get(nx)).copied()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/04.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 9);
    }
}
