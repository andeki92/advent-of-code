use std::time::Instant;

use aoc_common::nums::concat_numbers;

fn main() {
    let input = include_str!("../../data/inputs/07.txt");

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

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let test_value = left.parse::<u64>().unwrap();
            let numbers = right
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn can_solve(
    target: u64,
    current_value: u64,
    remaining_numbers: &[u64],
    with_concat: bool,
) -> bool {
    if current_value > target {
        return false;
    }

    match remaining_numbers {
        [] => return target == current_value,
        [next_num, rest @ ..] => {
            can_solve(target, current_value + next_num, rest, with_concat)
                || can_solve(target, current_value * next_num, rest, with_concat)
                || (with_concat && {
                    let concat = concat_numbers(current_value, *next_num);
                    can_solve(target, concat, rest, with_concat)
                })
        }
    }
}

fn part1(input: &str) -> u64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter(|e| can_solve(e.test_value, e.numbers[0], &e.numbers[1..], false))
        .map(|e| e.test_value)
        .sum()
}

fn part2(input: &str) -> u64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter(|e| can_solve(e.test_value, e.numbers[0], &e.numbers[1..], true))
        .map(|e| e.test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/07.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 11387);
    }
}
