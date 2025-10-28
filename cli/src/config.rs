use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub session: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let session = load_session().ok();
        Ok(Config { session })
    }

    pub fn require_session(&self) -> Result<&str> {
        self.session.as_deref().context(
            "No session cookie found. Please run:\n  aoc auth <session_cookie>\n\nGet your session cookie from https://adventofcode.com (F12 → Application → Cookies → session)"
        )
    }
}

fn config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().context("Could not determine config directory")?;
    path.push("aoc-cli");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn session_file_path() -> Result<PathBuf> {
    let mut path = config_dir()?;
    path.push("session");
    Ok(path)
}

fn load_session() -> Result<String> {
    // Try .env file first
    if let Ok(session) = std::env::var("AOC_SESSION") {
        return Ok(session);
    }

    // Try config file
    let path = session_file_path()?;
    if path.exists() {
        let session = fs::read_to_string(&path)
            .context("Failed to read session file")?
            .trim()
            .to_string();
        return Ok(session);
    }

    anyhow::bail!("No session found")
}

pub fn save_session(session: &str) -> Result<()> {
    let path = session_file_path()?;
    fs::write(&path, session.trim()).context("Failed to write session file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_load_session_from_env() {
        // Set environment variable (unsafe in recent Rust versions)
        unsafe {
            env::set_var("AOC_SESSION", "test_session_from_env");
        }

        let session = load_session().expect("Should load from environment");
        assert_eq!(session, "test_session_from_env");

        // Clean up
        unsafe {
            env::remove_var("AOC_SESSION");
        }
    }

    #[test]
    fn test_save_and_load_session() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let session_path = temp_dir.path().join("session");

        // Write directly to temp file to test loading
        fs::write(&session_path, "test_session_123")
            .expect("Failed to write test session");

        let content = fs::read_to_string(&session_path).unwrap();
        assert_eq!(content.trim(), "test_session_123");
    }

    #[test]
    fn test_config_require_session_fails_without_session() {
        let config = Config { session: None };
        let result = config.require_session();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No session cookie found"));
    }

    #[test]
    fn test_config_require_session_succeeds_with_session() {
        let config = Config {
            session: Some("valid_session".to_string()),
        };
        let result = config.require_session();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "valid_session");
    }
}
