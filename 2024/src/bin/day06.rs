use std::{collections::HashSet, time::Instant};

use aoc_common::grid::{Direction, Position, cell_at};

fn main() {
    let input = include_str!("../../data/inputs/06.txt");

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

fn get_start_pos(grid: &[Vec<char>]) -> Position {
    for (y, row) in grid.into_iter().enumerate() {
        for (x, &ch) in row.into_iter().enumerate() {
            if ch == '^' {
                return Position::new(x as i64, y as i64);
            }
        }
    }
    unreachable!("Failed to find starting position")
}

fn get_path(grid: &[Vec<char>]) -> HashSet<Position> {
    let mut pos = get_start_pos(grid);
    let mut direction = Direction::NORTH;
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);

        let next_pos = pos.step(direction);
        match cell_at(grid, next_pos) {
            Some('#') => direction = direction.turn_right(),
            Some(_) => pos = next_pos,
            None => break,
        }
    }

    visited
}

fn has_loop(grid: &[Vec<char>], obstacle: Position, mut pos: Position) -> bool {
    let mut visited = vec![vec![[false; 4]; grid[0].len()]; grid.len()];

    let mut direction = Direction::NORTH;

    loop {
        let (row, col) = pos.row_col().expect("position out of bounds");

        if visited[row][col][direction.index()] {
            return true;
        }
        visited[row][col][direction.index()] = true;

        let next_pos = pos.step(direction);

        if next_pos == obstacle {
            direction = direction.turn_right();
        } else {
            match cell_at(&grid, next_pos) {
                Some('#') => direction = direction.turn_right(),
                Some(_) => pos = next_pos,
                None => break,
            }
        }
    }

    false
}

// Part 1: 5080 (3.933125ms)
fn part1(input: &str) -> usize {
    let grid = aoc_common::parse_char_grid(input);
    get_path(&grid).len()
}

// Part 2: 1919 (56.993341375s)
fn part2(input: &str) -> usize {
    let grid = aoc_common::parse_char_grid(input);
    let start = get_start_pos(&grid);
    let path = get_path(&grid);

    path.into_iter()
        .filter(|&pos| pos != start)
        .filter(|&pos| has_loop(&grid, pos, start))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../data/examples/06.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
