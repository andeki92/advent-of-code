use anyhow::{Context, Result};
use colored::*;
use reqwest::header;
use std::fs;

use crate::config::Config;
use crate::utils;

pub fn run(config: &Config, day: u8, year: u16, force: bool) -> Result<()> {
    let session = config.require_session()?;

    println!(
        "{}",
        format!("Downloading input for day {} of year {}...", day, year).cyan()
    );

    // Determine input file path
    let input_path = utils::paths::day_input(year, day);

    // Check if file already exists
    if input_path.exists() && !force {
        let existing = fs::read_to_string(&input_path)?;
        if !existing.trim().is_empty() {
            println!(
                "  {} Input already exists at {}",
                "Skipped".yellow(),
                input_path.display()
            );
            println!("  Use --force to re-download");
            return Ok(());
        }
    }

    // Download input
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = utils::http_client()
        .get(&url)
        .header(header::COOKIE, format!("session={}", session))
        .send()
        .context("Failed to send request to Advent of Code")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download input: HTTP {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown error")
        );
    }

    let input = response
        .text()
        .context("Failed to read response body")?;

    // Check if we got an error page instead of input
    if input.contains("Please don't repeatedly request this endpoint") {
        anyhow::bail!("Rate limited by Advent of Code. Please wait a moment and try again.");
    }

    if input.contains("Please log in") {
        anyhow::bail!("Session cookie is invalid or expired. Please run: aoc auth <new_session>");
    }

    if input.contains("before it unlocks!") || input.contains("Not Found") {
        anyhow::bail!("Day {} is not yet available for year {}", day, year);
    }

    // Ensure parent directory exists
    if let Some(parent) = input_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Save input
    fs::write(&input_path, input)
        .context(format!("Failed to write input to {}", input_path.display()))?;

    println!("  {} {}", "Downloaded".green(), input_path.display());
    println!(
        "  {} {} bytes",
        "Size:".dimmed(),
        fs::metadata(&input_path)?.len()
    );

    Ok(())
}
