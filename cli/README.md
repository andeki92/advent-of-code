# aoc-cli

Custom CLI tool for Advent of Code workflow automation.

## Features

- **Scaffold**: Create new day solutions from template
- **Download**: Fetch puzzle inputs from adventofcode.com
- **Submit**: Submit solutions and get instant feedback
- **Session Management**: Secure cookie storage

## Installation

From the repository root:

```bash
cargo build --release -p aoc-cli
```

The binary will be at `target/release/aoc`.

Optionally, add it to your PATH:

```bash
# Linux/macOS
sudo cp target/release/aoc /usr/local/bin/

# Or add to your shell profile
export PATH="$PATH:/path/to/advent-of-code/target/release"
```

## Setup

### Get Session Cookie

1. Go to https://adventofcode.com and log in
2. Open Developer Tools (F12)
3. Navigate to Application/Storage → Cookies
4. Find the `session` cookie and copy its value

### Save Session Cookie

```bash
aoc auth "your_session_cookie_here"
```

This stores the cookie in `~/.config/aoc-cli/session`.

Alternatively, set it as an environment variable:

```bash
export AOC_SESSION="your_session_cookie_here"
```

## Usage

### Create a New Year

```bash
aoc new year 2025  # Set up directory structure for 2025
```

Creates:
- Year directory with template structure
- Cargo.toml and configuration files
- Empty data directories for inputs and examples
- Adds year to workspace

### Create a New Day

```bash
aoc new day 1           # Day 1 of current year
aoc new day 15 -y 2024  # Day 15 of 2024
```

Creates:
- Solution file from template
- Empty example input file
- Empty input file

### Download Puzzle Input

```bash
aoc download 1         # Day 1 of current year
aoc download 5 -y 2023  # Day 5 of 2023
aoc download 1 --force  # Re-download even if exists
```

### Submit Solution

```bash
aoc submit 1 1 "42"           # Day 1, part 1
aoc submit 1 2 "100" -y 2024  # Day 1, part 2 of 2024
```

The tool will:
- Show if your answer is correct/incorrect
- Display hints (too high/low)
- Handle rate limiting
- Detect already completed parts
- Automatically update the year's README progress table on success

### Check Progress

```bash
aoc status           # Check current year
aoc status -y 2024   # Check specific year
```

The tool will:
- Fetch your completion status from adventofcode.com
- Check which solutions exist locally
- Update the README.md progress table
- Display a summary of your progress

## Notes

### Year Detection

The tool determines the year in this order:
1. `--year` / `-y` flag
2. Current year

### User Agent

The tool identifies itself to Advent of Code servers with:
```
github.com/yourusername/advent-of-code by your@email.com
```

Please update this in the source code before use to comply with AoC's automation guidelines.

### Rate Limiting

Advent of Code has rate limits:
- Don't spam download requests
- Wait before resubmitting wrong answers (enforced by the site)
- The CLI will show wait times if you're rate limited

### Session Cookie Security

- Session cookies are stored in your config directory
- Never commit `.env` or share your session cookie
- Treat it like a password

## Development

### Project Structure

```
aoc-cli/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # Session management (with tests)
│   ├── utils.rs         # Shared utilities (with tests)
│   └── commands/
│       ├── new.rs       # New day/year commands
│       ├── download.rs  # Download command
│       ├── submit.rs    # Submit command
│       └── status.rs    # Status/progress command
├── template/            # REMOVED - now at ../aoc-template/
└── Cargo.toml
```

The template is now a separate workspace member at `aoc-template/`.
This allows it to:
- Be tested independently
- Inherit workspace settings
- Be versioned and improved separately

### Dependencies

- `clap`: CLI argument parsing
- `reqwest`: HTTP client
- `scraper`: HTML parsing for responses
- `colored`: Terminal colors
- `anyhow`: Error handling

## Future Enhancements

- GitHub OAuth authentication
- Download puzzle descriptions
- Cache puzzle metadata
- Visualization support
- Benchmark tracking in progress table
