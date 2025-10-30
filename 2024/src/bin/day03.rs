use std::time::Instant;

use regex::Regex;

fn main() {
    let input = include_str!("../../data/inputs/03.txt");

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

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"mul\((?P<x>\d+),(?P<y>\d+)\)|(?P<op>don\'t\(\)|do\(\))").unwrap();
    let mut is_enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        if let Some(op) = cap.name("op") {
            match op.as_str() {
                "don't()" => is_enabled = false,
                "do()" => is_enabled = true,
                _ => panic!("Not implemented: {:?}", op),
            }
        } else if let (Some(x), Some(y)) = (cap.name("x"), cap.name("y")) {
            if !is_enabled {
                continue;
            }

            let x: usize = x.as_str().parse().unwrap();
            let y: usize = y.as_str().parse().unwrap();
            sum += x * y;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/03.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 48);
    }
}
