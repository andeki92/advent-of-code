use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use scraper::Selector;

/// User agent for all HTTP requests to Advent of Code
const USER_AGENT: &str = "github.com/andeki92/advent-of-code by anders@bacheklever.no";

/// Shared HTTP client with proper user agent
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to create HTTP client")
});

/// Compiled selector for extracting submission results
static ARTICLE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("article").expect("Failed to compile article selector"));

/// Compiled selector for calendar days
static CALENDAR_DAY_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("pre.calendar a").expect("Failed to compile calendar-day selector")
});

/// Compiled regex for extracting wait times from rate limit messages
static WAIT_TIME_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"(\d+m\s*\d+s|\d+s|\d+\s*minute)")
        .expect("Failed to compile wait time regex")
});

/// Get the shared HTTP client
pub fn http_client() -> &'static Client {
    &HTTP_CLIENT
}

/// Get the article selector for parsing submissions
pub fn article_selector() -> &'static Selector {
    &ARTICLE_SELECTOR
}

/// Get the calendar day selector for parsing progress
pub fn calendar_day_selector() -> &'static Selector {
    &CALENDAR_DAY_SELECTOR
}

/// Extract wait time from rate limit message
pub fn extract_wait_time(text: &str) -> Option<String> {
    WAIT_TIME_REGEX.find(text).map(|m| m.as_str().to_string())
}

/// Path utilities for consistent year/day handling
pub mod paths {
    use std::path::PathBuf;

    /// Get the year directory path
    pub fn year_dir(year: u16) -> PathBuf {
        PathBuf::from(year.to_string())
    }

    /// Get the path to a day's solution file
    pub fn day_solution(year: u16, day: u8) -> PathBuf {
        year_dir(year).join(format!("src/bin/day{:02}.rs", day))
    }

    /// Get the path to a day's input file
    pub fn day_input(year: u16, day: u8) -> PathBuf {
        year_dir(year).join(format!("data/inputs/{:02}.txt", day))
    }

    /// Get the path to a day's example file
    pub fn day_example(year: u16, day: u8) -> PathBuf {
        year_dir(year).join(format!("data/examples/{:02}.txt", day))
    }

    /// Get the path to the year's README
    pub fn year_readme(year: u16) -> PathBuf {
        year_dir(year).join("README.md")
    }

    /// Get the path to the source template file (in aoc-template directory)
    pub fn source_template_file() -> PathBuf {
        template_dir().join("src/bin/template.rs")
    }

    /// Get the template directory path
    pub fn template_dir() -> PathBuf {
        PathBuf::from("template")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_utilities() {
        assert_eq!(
            paths::day_solution(2024, 1).to_str().unwrap(),
            "2024/src/bin/day01.rs"
        );
        assert_eq!(
            paths::day_input(2024, 15).to_str().unwrap(),
            "2024/data/inputs/15.txt"
        );
        assert_eq!(paths::year_readme(2024).to_str().unwrap(), "2024/README.md");
    }

    #[test]
    fn test_extract_wait_time() {
        assert_eq!(
            extract_wait_time("You have 5m 30s left"),
            Some("5m 30s".to_string())
        );
        assert_eq!(extract_wait_time("Wait 45s"), Some("45s".to_string()));
        assert_eq!(extract_wait_time("No time here"), None);
    }

    #[test]
    fn test_http_client_is_created() {
        // Just verify we can get the client without panicking
        let _ = http_client();
    }
}
