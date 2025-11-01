use std::time::Instant;

fn main() {
    let input = include_str!("../../data/inputs/XX.txt");

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

fn parse_input(input: &str) -> Vec<i64> {
    let _ = input; // TODO: implement
    vec![]
}

fn part1(input: &str) -> i64 {
    let formatted_input = parse_input(input);
    let _ = formatted_input; // TODO: implement
    0
}

fn part2(input: &str) -> i64 {
    let formatted_input = parse_input(input);
    let _ = formatted_input; // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/XX.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
