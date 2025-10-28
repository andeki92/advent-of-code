use anyhow::Result;
use chrono::Datelike;
use clap::{Parser, Subcommand};

mod commands;
mod config;
mod utils;

use commands::{download, new, status, submit};
use config::Config;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new day or year
    New {
        #[command(subcommand)]
        command: NewCommands,
    },

    /// Download puzzle input
    Download {
        /// Day number (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Year (defaults to current year)
        #[arg(short, long)]
        year: Option<u16>,

        /// Force re-download even if file exists
        #[arg(short, long)]
        force: bool,
    },

    /// Submit solution
    Submit {
        /// Day number (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Part number (1 or 2)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
        part: u8,

        /// Answer to submit
        answer: String,

        /// Year (defaults to current year)
        #[arg(short, long)]
        year: Option<u16>,
    },

    /// Check progress and update README
    Status {
        /// Year (defaults to current year)
        #[arg(short, long)]
        year: Option<u16>,

        /// Update README with current progress
        #[arg(short, long, default_value_t = true)]
        update: bool,
    },

    /// Set session cookie
    Auth {
        /// Session cookie value
        session: String,
    },
}

#[derive(Subcommand)]
enum NewCommands {
    /// Create a new day solution from template
    Day {
        /// Day number (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Year (defaults to current year)
        #[arg(short, long)]
        year: Option<u16>,
    },

    /// Set up a new year directory from template
    Year {
        /// Year to create (e.g., 2024)
        year: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Commands::New { command } => match command {
            NewCommands::Day { day, year } => {
                let year = determine_year(year)?;
                new::day(day, year)?;
            }
            NewCommands::Year { year } => {
                new::year(year)?;
            }
        },
        Commands::Download { day, year, force } => {
            let year = determine_year(year)?;
            download::run(&config, day, year, force)?;
        }
        Commands::Submit {
            day,
            part,
            answer,
            year,
        } => {
            let year = determine_year(year)?;
            submit::run(&config, day, part, &answer, year)?;
        }
        Commands::Status { year, update } => {
            let year = determine_year(year)?;
            status::run(&config, year, update)?;
        }
        Commands::Auth { session } => {
            config::save_session(&session)?;
            println!("Session cookie saved successfully!");
        }
    }

    Ok(())
}

fn determine_year(year: Option<u16>) -> Result<u16> {
    if let Some(y) = year {
        return Ok(y);
    }

    // Default to current year
    let now = chrono::Utc::now();
    Ok(now.year() as u16)
}
