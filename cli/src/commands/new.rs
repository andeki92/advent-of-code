use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use crate::commands::download;
use crate::config::Config;
use crate::utils;

pub fn day(config: &Config, day: u8, year: u16) -> Result<()> {
    println!(
        "{}",
        format!("Creating day {} for year {}...", day, year).cyan()
    );

    let year_dir = utils::paths::year_dir(year);
    if !year_dir.exists() {
        anyhow::bail!(
            "Year directory '{}' does not exist. Run 'aoc new year {}' first.",
            year_dir.display(),
            year
        );
    }

    // Paths
    let template_path = utils::paths::source_template_file();
    let solution_path = utils::paths::day_solution(year, day);
    let example_path = utils::paths::day_example(year, day);
    let input_path = utils::paths::day_input(year, day);

    // Check if solution already exists
    if solution_path.exists() {
        anyhow::bail!("Solution file already exists: {}", solution_path.display());
    }

    // Read template
    let template = fs::read_to_string(&template_path).context(format!(
        "Failed to read template from {}",
        template_path.display()
    ))?;

    // Replace XX with day number
    let solution = template.replace("XX", &format!("{:02}", day));

    // Write solution file
    fs::write(&solution_path, solution).context(format!(
        "Failed to write solution to {}",
        solution_path.display()
    ))?;
    println!("  {} {}", "Created".green(), solution_path.display());

    // Create empty example file
    fs::write(&example_path, "").context(format!(
        "Failed to create example file at {}",
        example_path.display()
    ))?;
    println!("  {} {}", "Created".green(), example_path.display());

    // Download input automatically
    println!();
    match download::run(config, day, year, false) {
        Ok(_) => {
            // Input downloaded successfully
        }
        Err(e) => {
            // If download fails, create empty file and show warning
            fs::write(&input_path, "").context(format!(
                "Failed to create input file at {}",
                input_path.display()
            ))?;
            println!("  {} Could not download input: {}", "⚠".yellow(), e);
            println!(
                "  {} Empty input file created at {}",
                "Created".green(),
                input_path.display()
            );
            println!(
                "  {} You can download it later with: aoc download {} -y {}",
                "ℹ".blue(),
                day,
                year
            );
        }
    }

    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Add example input to {}", example_path.display());
    println!("  2. Implement solution in {}", solution_path.display());
    let day_padded = format!("{:02}", day);
    println!("  3. Run tests:");
    println!(
        "     {} cd {} && cargo test --bin day{}",
        "$".dimmed(),
        year,
        day_padded
    );
    println!("  4. Run solution:");
    println!(
        "     {} cd {} && cargo run --bin day{} --release",
        "$".dimmed(),
        year,
        day_padded
    );

    Ok(())
}

pub fn year(year: u16) -> Result<()> {
    println!(
        "{}",
        format!("Setting up Advent of Code {}...", year).cyan()
    );

    let year_dir = utils::paths::year_dir(year);

    // Look for template directory in current working directory
    let template_dir = utils::paths::template_dir();

    // Validation
    if year_dir.exists() {
        anyhow::bail!("Year directory '{}' already exists", year);
    }

    if !template_dir.exists() {
        anyhow::bail!(
            "Template directory not found at '{}'. Are you in the repository root?",
            template_dir.display()
        );
    }

    // Copy the year structure from template
    println!("  {} Copying structure from template...", "→".dimmed());
    copy_dir_all(&template_dir, &year_dir).context("Failed to copy from template")?;

    // Update files with year placeholders
    println!("  {} Updating year placeholders...", "→".dimmed());
    update_year_placeholders(&year_dir, year)?;

    // Remove placeholder files (only needed in template dir)
    println!("  {} Removing placeholder files...", "→".dimmed());
    let _ = fs::remove_file(year_dir.join("data/inputs/XX.txt"));
    let _ = fs::remove_file(year_dir.join("data/examples/XX.txt"));

    // Remove template.rs (only needed in template dir)
    println!("  {} Removing template binary...", "→".dimmed());
    let _ = fs::remove_file(year_dir.join("src/bin/template.rs"));

    // Clean any existing day solutions
    if let Ok(entries) = fs::read_dir(year_dir.join("src/bin")) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("day") && name.ends_with(".rs") {
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }

    // Add to workspace
    println!("  {} Adding to workspace...", "→".dimmed());
    add_to_workspace(year)?;

    println!();
    println!(
        "{}",
        format!("✓ Year {} setup complete!", year).green().bold()
    );
    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Verify workspace:");
    println!("     {} cargo check --workspace", "$".dimmed());
    println!("  2. Create first day:");
    println!("     {} aoc new day 1 -y {}", "$".dimmed(), year);
    println!("  3. Download input:");
    println!("     {} aoc download 1 -y {}", "$".dimmed(), year);
    println!("  4. Start solving!");

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            // Skip target directory
            if entry.file_name() == "target" {
                continue;
            }
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn update_year_placeholders(year_dir: &Path, year: u16) -> Result<()> {
    // Update README
    let readme_path = year_dir.join("README.md");
    if readme_path.exists() {
        let content = fs::read_to_string(&readme_path)?;
        let updated = content.replace("YEAR", &year.to_string());
        fs::write(&readme_path, updated)?;
    }

    // Update Cargo.toml and remove template binary section
    let cargo_path = year_dir.join("Cargo.toml");
    if cargo_path.exists() {
        let content = fs::read_to_string(&cargo_path)?;
        let updated = content
            .replace("YEAR", &year.to_string())
            // Remove the template binary section (multi-line)
            .lines()
            .filter(|line| {
                !line.contains("# Template binary")
                    && !line.contains("[[bin]]")
                    && !line.contains("name = \"template\"")
                    && !line.contains("path = \"src/bin/template.rs\"")
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&cargo_path, updated)?;
    }

    Ok(())
}

fn add_to_workspace(year: u16) -> Result<()> {
    let workspace_toml = std::path::PathBuf::from("Cargo.toml");

    if !workspace_toml.exists() {
        anyhow::bail!("Workspace Cargo.toml not found. Are you in the repository root?");
    }

    let content = fs::read_to_string(&workspace_toml)?;

    // Check if already in workspace
    if content.contains(&format!("\"{}\"", year)) {
        println!("    {} Already in workspace", "ℹ".blue());
        return Ok(());
    }

    // Find the members = [...] line and add the new year inline
    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut found_members = false;

    for line in lines {
        if line.trim().starts_with("members = [") {
            found_members = true;
            // Parse the existing members and add new year inline
            if line.trim().ends_with("]") {
                // Single-line format: members = ["foo", "bar"]
                let insert_pos = line.rfind(']').unwrap();
                if line.contains("[]") {
                    // Empty array
                    new_lines.push(format!("members = [\"{}\"]", year));
                } else {
                    // Has members - add to end
                    new_lines.push(format!("{}, \"{}\"]", &line[..insert_pos], year));
                }
            } else {
                // Multi-line format - keep as is (shouldn't happen with our setup)
                new_lines.push(line.to_string());
            }
        } else {
            new_lines.push(line.to_string());
        }
    }

    if !found_members {
        anyhow::bail!("Could not find 'members = [' in workspace Cargo.toml");
    }

    fs::write(&workspace_toml, new_lines.join("\n"))?;
    println!("    {} Added to workspace", "✓".green());

    Ok(())
}
