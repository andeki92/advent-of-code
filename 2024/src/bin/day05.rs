use std::{cmp::Ordering, collections::HashSet, time::Instant};

fn main() {
    let input = include_str!("../../data/inputs/05.txt");

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

type Rules = HashSet<(usize, usize)>;
type Pages = Vec<Vec<usize>>;

fn format_input(input: &str) -> (Rules, Pages) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let rules: Rules = left
        .lines()
        .map(|line| {
            let (first, second) = line.split_once('|').unwrap();
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let pages: Pages = right
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (rules, pages)
}

fn is_valid_order(rules: &Rules, pages: &[usize]) -> bool {
    for (idx, &x) in pages.iter().enumerate() {
        for &y in &pages[idx + 1..] {
            if rules.contains(&(y, x)) {
                return false;
            }
        }
    }
    true
}

fn reorder(rules: &Rules, pages: &mut [usize]) {
    pages.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn middle_element(slice: &Vec<usize>) -> usize {
    slice[slice.len() / 2]
}

fn part1(input: &str) -> usize {
    let (rules, pages) = format_input(input);

    pages
        .iter()
        .filter(|page_list| is_valid_order(&rules, page_list))
        .map(middle_element)
        .sum()
}

fn part2(input: &str) -> usize {
    let (rules, pages) = format_input(input);

    pages
        .into_iter()
        .filter(|page_list| !is_valid_order(&rules, page_list))
        .map(|mut page_list| {
            reorder(&rules, &mut page_list);
            middle_element(&page_list)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/05.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}
