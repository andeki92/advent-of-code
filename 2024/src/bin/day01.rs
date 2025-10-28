use std::{collections::HashMap, str, time::Instant};

fn main() {
    let input = include_str!("../../data/inputs/01.txt");

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

fn format_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(char::is_whitespace).unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.trim_start().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let formatted_input = format_input(input);
    let (mut left, mut right): (Vec<_>, Vec<_>) = formatted_input.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part2(input: &str) -> usize {
    let formatted_input = format_input(input);
    let (left, right): (Vec<_>, Vec<_>) = formatted_input.into_iter().unzip();

    let mut counts = HashMap::new();
    for &num in &right {
        *counts.entry(num).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|num| num * counts.get(&num).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/01.txt");

    #[test]
    fn test_input_formatter() {
        let formatted_input = format_input(EXAMPLE);

        assert_eq!(formatted_input[0], (3, 4));
        assert_eq!(formatted_input[1], (4, 3));
        assert_eq!(formatted_input[2], (2, 5));
        assert_eq!(formatted_input[3], (1, 3));
        assert_eq!(formatted_input[4], (3, 9));
        assert_eq!(formatted_input[5], (3, 3));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}
