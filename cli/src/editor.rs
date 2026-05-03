use std::fs;
use std::io::Stdout;
use std::path::PathBuf;
use std::process::Command;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::api::error::AppError;

/// Resolve the editor binary to use.
///
/// Checks in order: $EDITOR env var → nvim → vim → vi on PATH.
/// Returns the editor command string or `AppError::EditorNotFound`.
pub fn resolve_editor() -> Result<String, AppError> {
    // 1. Check $EDITOR environment variable
    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }

    // 2. Check nvim, vim, vi on PATH
    for candidate in &["nvim", "vim", "vi"] {
        if is_on_path(candidate) {
            return Ok(candidate.to_string());
        }
    }

    Err(AppError::EditorNotFound)
}

/// Get the challenges directory: ~/.rustgym/challenges/
fn challenges_dir() -> Result<PathBuf, AppError> {
    let home = dirs::home_dir().ok_or_else(|| {
        AppError::Other("Could not determine home directory".to_string())
    })?;
    Ok(home.join(".rustgym").join("challenges"))
}

/// Build the file path for a given task slug under ~/.rustgym/challenges/{slug}/src/lib.rs.
pub fn file_path_for_slug(slug: &str) -> String {
    match challenges_dir() {
        Ok(dir) => dir
            .join(slug)
            .join("src")
            .join("lib.rs")
            .to_string_lossy()
            .to_string(),
        // Fallback if home dir can't be determined
        Err(_) => format!("/tmp/rustgym/challenges/{}/src/lib.rs", slug),
    }
}

/// Ensure the challenge file exists on disk with the starter code.
/// If the file already exists (user has edited it), don't overwrite.
/// If it doesn't exist, create it with the starter code.
/// Also writes/updates the description.md file alongside it.
pub fn ensure_challenge_file(
    slug: &str,
    starter_code: &str,
    description_md: &str,
) -> Result<String, AppError> {
    let file_path = file_path_for_slug(slug);
    let path = PathBuf::from(&file_path);

    // Create parent directories.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::FileError {
            path: parent.to_string_lossy().to_string(),
            message: e.to_string(),
        })?;
    }

    // If code file doesn't exist, write starter code.
    if !path.exists() {
        fs::write(&path, starter_code).map_err(|e| AppError::FileError {
            path: file_path.clone(),
            message: e.to_string(),
        })?;
    }

    // Always write/update the description file (it's read-only reference material).
    if !description_md.is_empty() {
        let desc_path = description_path_for_slug(slug);
        let desc_p = PathBuf::from(&desc_path);
        if let Some(parent) = desc_p.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&desc_p, description_md);
    }

    Ok(file_path)
}

/// Build the description file path: ~/.rustgym/challenges/{slug}/description.md
pub fn description_path_for_slug(slug: &str) -> String {
    match challenges_dir() {
        Ok(dir) => dir
            .join(slug)
            .join("description.md")
            .to_string_lossy()
            .to_string(),
        Err(_) => format!("/tmp/rustgym/challenges/{}/description.md", slug),
    }
}

/// Suspend the TUI, launch the editor on the given file, then restore the TUI.
pub fn open(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    file_path: &str,
    _slug: &str,
) -> Result<(), AppError> {
    let editor = resolve_editor()?;

    // Leave alternate screen
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .map_err(|e| AppError::Other(format!("Failed to leave alternate screen: {}", e)))?;

    // Disable raw mode
    disable_raw_mode()
        .map_err(|e| AppError::Other(format!("Failed to disable raw mode: {}", e)))?;

    // Open only the code file. The description is viewable in the TUI.
    let status = Command::new(&editor)
        .arg(file_path)
        .status()
        .map_err(|e| AppError::Other(format!("Failed to launch editor '{}': {}", editor, e)))?;

    if !status.success() {
        let _ = enable_raw_mode();
        let _ = execute!(terminal.backend_mut(), EnterAlternateScreen);
        let _ = terminal.clear();
        return Err(AppError::Other(format!(
            "Editor '{}' exited with status: {}",
            editor,
            status.code().unwrap_or(-1)
        )));
    }

    // Re-enable raw mode
    enable_raw_mode()
        .map_err(|e| AppError::Other(format!("Failed to re-enable raw mode: {}", e)))?;

    // Re-enter alternate screen
    execute!(terminal.backend_mut(), EnterAlternateScreen)
        .map_err(|e| AppError::Other(format!("Failed to enter alternate screen: {}", e)))?;

    // Clear terminal to force full redraw
    terminal
        .clear()
        .map_err(|e| AppError::Other(format!("Failed to clear terminal: {}", e)))?;

    Ok(())
}

/// Check if a binary is available on PATH using the `which` command.
fn is_on_path(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_path_for_slug_uses_home_dir() {
        let path = file_path_for_slug("two-sum");
        assert!(path.contains("two-sum"));
        assert!(path.ends_with("src/lib.rs"));
        assert!(path.contains(".rustgym/challenges"));
    }

    #[test]
    fn file_path_for_slug_handles_nested_slug() {
        let path = file_path_for_slug("group-by");
        assert!(path.contains("group-by"));
        assert!(path.ends_with("src/lib.rs"));
    }

    #[test]
    fn resolve_editor_respects_env_var() {
        unsafe { std::env::set_var("EDITOR", "nano"); }
        let result = resolve_editor();
        assert_eq!(result.unwrap(), "nano");
        unsafe { std::env::remove_var("EDITOR"); }
    }

    #[test]
    fn resolve_editor_skips_empty_env_var() {
        unsafe { std::env::set_var("EDITOR", ""); }
        let result = resolve_editor();
        if let Ok(editor) = result {
            assert!(!editor.is_empty());
        }
        unsafe { std::env::remove_var("EDITOR"); }
    }

    #[test]
    fn ensure_challenge_file_creates_file() {
        let slug = "test-ensure-challenge";
        let starter = "// starter code\nfn solution() {}";
        let description = "# Test Challenge\n\nSolve this.";

        // Clean up first
        let path = file_path_for_slug(slug);
        let _ = std::fs::remove_file(&path);
        let desc_path = description_path_for_slug(slug);
        let _ = std::fs::remove_file(&desc_path);

        let result = ensure_challenge_file(slug, starter, description);
        assert!(result.is_ok());

        let written = std::fs::read_to_string(&path).unwrap();
        assert_eq!(written, starter);

        // Description should also be written
        let desc_written = std::fs::read_to_string(&desc_path).unwrap();
        assert_eq!(desc_written, description);

        // Clean up
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&desc_path);
        let p = PathBuf::from(&path);
        let _ = std::fs::remove_dir(p.parent().unwrap());
        let _ = std::fs::remove_dir(p.parent().unwrap().parent().unwrap());
    }

    #[test]
    fn ensure_challenge_file_does_not_overwrite() {
        let slug = "test-no-overwrite";
        let starter = "// original starter";
        let user_code = "// user modified code";
        let description = "# Test\n\nDescription.";

        // Write the file first with user code
        let path = file_path_for_slug(slug);
        let p = PathBuf::from(&path);
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&path, user_code).unwrap();

        // ensure_challenge_file should NOT overwrite code
        let result = ensure_challenge_file(slug, starter, description);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, user_code); // Should still be user's code

        // Clean up
        let _ = std::fs::remove_file(&path);
        let desc_path = description_path_for_slug(slug);
        let _ = std::fs::remove_file(&desc_path);
        let _ = std::fs::remove_dir(p.parent().unwrap());
        let _ = std::fs::remove_dir(p.parent().unwrap().parent().unwrap());
    }
}
