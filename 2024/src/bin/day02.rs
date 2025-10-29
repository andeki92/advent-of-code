use std::time::Instant;

fn main() {
    let input = include_str!("../../data/inputs/02.txt");

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

fn format_input(input: &str) -> Vec<Vec<usize>> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|chars| chars.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

// A report only counts as safe if both of the following are true:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn is_safe_report(report: &[usize]) -> bool {
    let all_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let all_decreasing = report.windows(2).all(|w| w[0] > w[1]);

    let valid_diffs = report.windows(2).all(|w| match w {
        [current, next] => {
            let diff = current.abs_diff(*next);
            diff >= 1 && diff <= 3
        }
        _ => unreachable!(),
    });

    (all_increasing || all_decreasing) && valid_diffs
}

// A report only counts as safe if both of the following are true:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
//
// The Problem Dampener is a reactor-mounted module that lets the reactor
// safety systems tolerate a single bad level in what would otherwise be
// a safe report. It's like the bad level never happened!
fn is_safe_with_dampening(report: &[usize]) -> bool {
    // First check if already safe without dampening
    if is_safe_report(report) {
        return true;
    }

    (0..report.len()).any(|skip_idx| {
        let dampened: Vec<_> = report[..skip_idx]
            .iter()
            .chain(&report[skip_idx + 1..])
            .copied()
            .collect::<Vec<_>>();

        is_safe_report(&dampened)
    })
}

fn part1(input: &str) -> usize {
    let formatted_input = format_input(input);
    formatted_input
        .into_iter()
        .filter(|r| is_safe_report(r))
        .count()
}

fn part2(input: &str) -> usize {
    let formatted_input = format_input(input);
    formatted_input
        .into_iter()
        .filter(|r| is_safe_with_dampening(r))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/02.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}
