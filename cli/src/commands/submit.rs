use anyhow::{Context, Result};
use colored::*;
use reqwest::header;
use scraper::Html;
use std::collections::HashMap;

use crate::commands::status;
use crate::config::Config;
use crate::utils;

pub fn run(config: &Config, day: u8, part: u8, answer: &str, year: u16) -> Result<()> {
    let session = config.require_session()?;

    println!(
        "{}",
        format!(
            "Submitting answer for day {}, part {} of year {}...",
            day, part, year
        )
        .cyan()
    );
    println!("  Answer: {}", answer.bold());

    // Submit answer
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);

    let mut form = HashMap::new();
    form.insert("level", part.to_string());
    form.insert("answer", answer.to_string());

    let response = utils::http_client()
        .post(&url)
        .header(header::COOKIE, format!("session={}", session))
        .form(&form)
        .send()
        .context("Failed to send submission to Advent of Code")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to submit answer: HTTP {} - {}",
            response.status(),
            response
                .status()
                .canonical_reason()
                .unwrap_or("Unknown error")
        );
    }

    let body = response.text().context("Failed to read response body")?;

    // Parse response to extract the result message
    let document = Html::parse_document(&body);

    if let Some(article) = document.select(utils::article_selector()).next() {
        let text = article.text().collect::<String>();

        // Analyze the response
        if text.contains("That's the right answer") {
            println!("\n{}", "✓ Correct!".green().bold());
            println!("{}", text.trim());

            if text.contains("one gold star") || text.contains("You complete") {
                println!(
                    "\n{} ⭐",
                    format!("You earned a star for day {}!", day)
                        .yellow()
                        .bold()
                );
            }

            // Update README with progress by fetching from AOC website (quietly)
            println!();
            if let Err(e) = status::run_quiet(config, year, true) {
                println!(
                    "{}",
                    format!("Warning: Could not update README: {}", e).yellow()
                );
            }
        } else if text.contains("That's not the right answer") {
            println!("\n{}", "✗ Incorrect".red().bold());
            println!("{}", text.trim());

            if text.contains("too high") {
                println!("\n{}", "Hint: Your answer is too high".yellow());
            } else if text.contains("too low") {
                println!("\n{}", "Hint: Your answer is too low".yellow());
            }
        } else if text.contains("You gave an answer too recently") {
            println!("\n{}", "⏱ Rate Limited".yellow().bold());
            println!("{}", text.trim());

            // Try to extract wait time
            if let Some(wait_str) = utils::extract_wait_time(&text) {
                println!(
                    "\n{}",
                    format!("Please wait {} before submitting again", wait_str).dimmed()
                );
            }
        } else if text.contains("Did you already complete it") {
            println!("\n{}", "✓ Already Completed".green().bold());
            println!("You've already completed this part!");
        } else {
            println!("\n{}", "Response:".bold());
            println!("{}", text.trim());
        }
    } else {
        println!("\n{}", "Warning: Could not parse response".yellow());
        println!("Response might contain useful information.");
    }

    Ok(())
}
