use anyhow::{Context, Result};
use colored::*;
use reqwest::header;
use scraper::Html;
use std::collections::HashSet;
use std::fs;

use crate::config::Config;
use crate::utils;

#[derive(Debug)]
struct DayStatus {
    day: u8,
    part1: bool,
    part2: bool,
    has_solution: bool,
}

pub fn run(config: &Config, year: u16, update_readme: bool) -> Result<()> {
    run_with_options(config, year, update_readme, false)
}

pub fn run_quiet(config: &Config, year: u16, update_readme: bool) -> Result<()> {
    run_with_options(config, year, update_readme, true)
}

fn run_with_options(config: &Config, year: u16, update_readme: bool, quiet: bool) -> Result<()> {
    let session = config.require_session()?;

    if !quiet {
        println!(
            "{}",
            format!("Fetching progress for Advent of Code {}...", year).cyan()
        );
    }

    // Fetch completion status from AOC website
    let completed = fetch_completed_stars(&session, year)?;

    // Check which solutions exist locally
    let local_solutions = find_local_solutions(year)?;

    // Build status for all 25 days
    let mut statuses = Vec::new();
    for day in 1..=25 {
        let (part1, part2) = completed.get(&day).copied().unwrap_or((false, false));
        let has_solution = local_solutions.contains(&day);

        statuses.push(DayStatus {
            day,
            part1,
            part2,
            has_solution,
        });
    }

    // Calculate total stars
    let total_stars: usize = statuses
        .iter()
        .map(|s| s.part1 as usize + s.part2 as usize)
        .sum();

    // Display summary
    if !quiet {
        println!("\n{}", "Progress Summary:".bold());
        println!("  Total Stars: {}/50 ⭐", total_stars);
        println!("  Local Solutions: {}/25", local_solutions.len());
    }

    // Update README if requested
    if update_readme {
        let readme_path = utils::paths::year_readme(year);
        if readme_path.exists() {
            update_readme_table(&readme_path, &statuses, total_stars, year)?;
            if !quiet {
                println!("\n{}", "✓ README.md updated".green());
            } else {
                println!("{}", "✓ README.md updated".green());
            }
        } else if !quiet {
            println!(
                "\n{}",
                format!("Warning: README.md not found at {}", readme_path.display()).yellow()
            );
        }
    }

    Ok(())
}

fn fetch_completed_stars(
    session: &str,
    year: u16,
) -> Result<std::collections::HashMap<u8, (bool, bool)>> {
    let url = format!("https://adventofcode.com/{}", year);
    let response = utils::http_client()
        .get(&url)
        .header(header::COOKIE, format!("session={}", session))
        .send()
        .context("Failed to fetch Advent of Code calendar page")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch calendar: HTTP {}", response.status());
    }

    let body = response.text()?;
    let document = Html::parse_document(&body);

    // Parse the calendar to find completed days
    // Days are marked with CSS classes: calendar-complete (1 star) or calendar-verycomplete (2 stars)
    let mut completed = std::collections::HashMap::new();

    for day_element in document.select(utils::calendar_day_selector()) {
        // Extract day number from the aria-label attribute (e.g., "Day 1, two stars")
        if let Some(aria_label) = day_element.value().attr("aria-label") {
            // Parse "Day X" or "Day X, one star" or "Day X, two stars"
            let day_str = aria_label
                .split(',')
                .next()
                .and_then(|s| s.strip_prefix("Day "))
                .unwrap_or("");

            if let Ok(day) = day_str.trim().parse::<u8>() {
                // Check CSS classes to determine completion status
                let class_attr = day_element.value().attr("class").unwrap_or("");
                let (part1, part2) = if class_attr.contains("calendar-verycomplete") {
                    (true, true) // Both parts completed
                } else if class_attr.contains("calendar-complete") {
                    (true, false) // Only part 1 completed
                } else {
                    (false, false) // Not started
                };

                completed.insert(day, (part1, part2));
            }
        }
    }

    Ok(completed)
}

fn find_local_solutions(year: u16) -> Result<HashSet<u8>> {
    let bin_dir = utils::paths::year_dir(year).join("src/bin");
    let mut solutions = HashSet::new();

    if !bin_dir.exists() {
        return Ok(solutions);
    }

    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Match files like "day01.rs", "day02.rs", etc.
            if name.starts_with("day") && name.ends_with(".rs") {
                if let Some(day_str) = name.strip_prefix("day").and_then(|s| s.strip_suffix(".rs"))
                {
                    if let Ok(day) = day_str.parse::<u8>() {
                        solutions.insert(day);
                    }
                }
            }
        }
    }

    Ok(solutions)
}

fn update_readme_table(
    readme_path: &std::path::PathBuf,
    statuses: &[DayStatus],
    total_stars: usize,
    year: u16,
) -> Result<()> {
    let content = fs::read_to_string(readme_path)?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    // Find the progress table section
    let mut table_start = None;
    let mut table_end = None;

    for (i, line) in lines.iter().enumerate() {
        // Match table header with flexible whitespace (handles padded columns)
        if line.starts_with("| Day") && line.contains("Part 1") && line.contains("Part 2") {
            table_start = Some(i);
        } else if table_start.is_some() && table_end.is_none() {
            // Look for end of table (empty line or next section)
            if line.trim().is_empty() || (line.starts_with('#') || line.starts_with("**Total:")) {
                table_end = Some(i);
                break;
            }
        }
    }

    if let (Some(start), Some(end)) = (table_start, table_end) {
        // Remove old table content (keep header)
        lines.drain(start + 2..end); // Keep header and separator line

        // Generate new table rows
        let mut new_rows = Vec::new();
        for status in statuses {
            let day_padded = format!("{:02}", status.day);
            let part1 = if status.part1 { "⭐" } else { "" };
            let part2 = if status.part2 { "⭐" } else { "" };
            let solution = if status.has_solution {
                format!("[day{}.rs](src/bin/day{}.rs)", day_padded, day_padded)
            } else {
                String::new()
            };
            let benchmark = if status.has_solution { "-" } else { "" };

            new_rows.push(format!(
                "| [{}](https://adventofcode.com/{}/day/{}) | {} | {} | {} | {} |",
                day_padded, year, status.day, part1, part2, solution, benchmark
            ));
        }

        // Insert new rows
        for (i, row) in new_rows.iter().enumerate() {
            lines.insert(start + 2 + i, row.clone());
        }

        // Update total stars line
        for line in lines.iter_mut() {
            if line.starts_with("**Total:") {
                *line = format!("**Total: {}/50 ⭐**", total_stars);
                break;
            }
        }
    }

    // Write back to file
    let updated_content = lines.join("\n") + "\n";
    fs::write(readme_path, updated_content)?;

    Ok(())
}
