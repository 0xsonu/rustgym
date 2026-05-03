# RustGym CLI

A full-screen interactive Terminal User Interface (TUI) for the RustGym platform. Browse quests, solve coding challenges, and submit solutions — all from your terminal.

## Features

- Full-screen TUI with vim-style navigation
- Login with persistent sessions (auto-login on relaunch)
- Browse quests, levels, and tasks with progress tracking
- Open challenges in your preferred editor (nvim/vim/vi or `$EDITOR`)
- Submit solutions and see test results inline
- XP tracking, level progression, and completion status
- Locked level indicators (sequential progression)

## Installation

```bash
cd cli
cargo install --path .
```

## Usage

Launch the TUI:

```bash
rustgym
```

### Options

```
--api-url <URL>    API base URL (default: https://rustgym.dev)
--help, -h         Print help information
--version, -V      Print version information
```

## Navigation

RustGym CLI uses vim-style keybindings throughout:

### Global

| Key    | Action              |
| ------ | ------------------- |
| Ctrl+C | Quit (always works) |

### Login Screen

| Key   | Action                      |
| ----- | --------------------------- |
| i / a | Enter editing mode          |
| Esc   | Exit editing / quit         |
| Tab   | Switch between fields       |
| j / k | Switch fields (normal mode) |
| Enter | Submit login                |
| q     | Quit (normal mode)          |

### Quest List

| Key   | Action           |
| ----- | ---------------- |
| j / k | Navigate up/down |
| Enter | Open quest       |
| L     | Logout           |
| q     | Quit             |

### Quest Detail (Levels)

| Key   | Action                         |
| ----- | ------------------------------ |
| j / k | Navigate (skips locked levels) |
| Enter | Open level (unlocked only)     |
| b     | Back to quest list             |

### Level Detail (Tasks)

| Key   | Action         |
| ----- | -------------- |
| j / k | Navigate       |
| Enter | Open task      |
| b     | Back to levels |

### Task Detail

| Key   | Action             |
| ----- | ------------------ |
| i / a | Open in editor     |
| s     | Submit solution    |
| ?     | Toggle hint        |
| j / k | Scroll description |
| b     | Back to level      |

### Submit Screen

| Key   | Action           |
| ----- | ---------------- |
| j / k | Scroll results   |
| r     | Retry submission |
| b     | Back to task     |

## Editor Integration

When you press `i` on a task, the CLI:

1. Downloads the starter code to `~/.rustgym/challenges/<slug>/src/lib.rs`
2. Opens the file in your editor
3. Returns to the TUI when the editor exits

Editor resolution order:

1. `$EDITOR` environment variable
2. `nvim` (if on PATH)
3. `vim` (if on PATH)
4. `vi` (if on PATH)

## Submitting Solutions

Press `s` on a task detail screen to submit your solution. The CLI reads your code from `~/.rustgym/challenges/<slug>/src/lib.rs` and sends it to the server for execution.

Results show:

- Pass/fail status with test count
- Execution duration
- XP awarded (first successful submission only)
- Compiler errors (filtered, no build noise)
- Failed test details with error output

## Configuration

Config is stored at `~/.rustgym/config.toml`:

```toml
token = "your-jwt-token"
api_url = "https://rustgym.dev"
```

- Token is saved automatically on login
- API URL can be set via `--api-url` flag (persisted to config)
- Token is cleared on logout or session expiry (401)

## Challenge Files

Challenge files are stored at:

```
~/.rustgym/challenges/
├── <task-slug>/
│   ├── src/
│   │   └── lib.rs          # Your solution (starter code on first open)
│   └── description.md      # Problem statement (reference)
```

- Code files are never overwritten (your edits persist)
- Description files are updated on each open
- Files persist between sessions for continued work

## Architecture

The CLI is built with:

- **Ratatui** + **crossterm** for the TUI rendering
- **Tokio** async runtime for non-blocking API calls
- **reqwest** for HTTP communication with the backend
- Elm-like architecture: App state → Event → Update → Render

## Development

```bash
# Check
cargo check

# Test (133 tests including property-based tests)
cargo test

# Build
cargo build --release

# Run locally against a dev server
cargo run -- --api-url http://localhost:3000
```
