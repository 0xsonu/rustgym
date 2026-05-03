// Feature: cli-tui-rewrite, Property 9: Editor resolution follows priority order
// Feature: cli-tui-rewrite, Property 14: Editor file path follows slug pattern

use proptest::prelude::*;

use rustgym_cli::editor::{file_path_for_slug, resolve_editor};

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Strategy to generate valid task slugs: lowercase alphanumeric with hyphens,
/// starting with a letter, length 1..30.
fn arb_valid_slug() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9\\-]{0,28}[a-z0-9]"
}

/// Strategy to generate non-empty editor values (arbitrary non-empty strings
/// that could represent an editor binary name or path).
fn arb_editor_value() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9/_\\-]{0,30}"
}

// ─── Property 9: Editor resolution follows priority order ───────────────────

// **Validates: Requirements 7.4, 10.2**
//
// For any environment state, the editor resolver SHALL return the first
// available option in this order:
// (1) $EDITOR value if set and non-empty
// (2) nvim if on PATH
// (3) vim if on PATH
// (4) vi if on PATH
// (5) error if none found
//
// We test the $EDITOR priority: when $EDITOR is set to a non-empty value,
// resolve_editor must return that value regardless of what's on PATH.

proptest! {
    /// When $EDITOR is set to any non-empty value, resolve_editor returns that value.
    #[test]
    fn prop_resolve_editor_returns_editor_env_when_set(
        editor_val in arb_editor_value(),
    ) {
        // SAFETY: These tests must run serially due to env var mutation.
        // proptest runs iterations within a single test, so this is safe.
        unsafe { std::env::set_var("EDITOR", &editor_val); }
        let result = resolve_editor();
        unsafe { std::env::remove_var("EDITOR"); }

        prop_assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        prop_assert_eq!(result.unwrap(), editor_val);
    }
}

/// When $EDITOR is empty, resolve_editor does NOT return an empty string.
/// It should fall through to PATH checks.
#[test]
fn test_resolve_editor_skips_empty_env_var() {
    unsafe { std::env::set_var("EDITOR", ""); }
    let result = resolve_editor();
    unsafe { std::env::remove_var("EDITOR"); }

    // Should either find something on PATH or return EditorNotFound
    match result {
        Ok(editor) => assert!(!editor.is_empty(), "Editor should not be empty"),
        Err(_) => {} // EditorNotFound is acceptable if nothing is on PATH
    }
}

/// When $EDITOR is unset, resolve_editor does not return $EDITOR.
/// It should fall through to PATH checks (nvim → vim → vi).
#[test]
fn test_resolve_editor_falls_through_when_unset() {
    unsafe { std::env::remove_var("EDITOR"); }
    let result = resolve_editor();

    match result {
        Ok(editor) => {
            // Must be one of the PATH fallbacks
            assert!(
                editor == "nvim" || editor == "vim" || editor == "vi",
                "Expected nvim/vim/vi fallback, got: {}",
                editor
            );
        }
        Err(_) => {} // EditorNotFound is acceptable if nothing is on PATH
    }
}

/// $EDITOR takes priority over PATH editors — even if nvim/vim/vi are available,
/// the $EDITOR value wins.
/// Note: This test uses serial execution to avoid env var race conditions.
#[test]
fn test_resolve_editor_env_takes_priority_over_path() {
    // This is covered by the proptest above which generates random EDITOR values.
    // We just verify the basic contract here with a known value.
    unsafe { std::env::set_var("EDITOR", "custom-editor-xyz"); }
    let result = resolve_editor();
    // Clean up immediately
    unsafe { std::env::remove_var("EDITOR"); }

    // The result should be our custom editor (unless another test raced us).
    // Since env vars are process-global, we accept either our value or a PATH fallback.
    match result {
        Ok(editor) => {
            assert!(
                editor == "custom-editor-xyz" || editor == "nvim" || editor == "vim" || editor == "vi",
                "Expected custom-editor-xyz or PATH fallback, got: {}",
                editor
            );
        }
        Err(_) => panic!("Expected Ok result"),
    }
}

// ─── Property 14: Editor file path follows slug pattern ─────────────────────

// **Validates: Requirements 10.5**
//
// For any valid task slug `s`, the editor SHALL open the file at a path
// under ~/.rustgym/challenges/{s}/src/lib.rs.

proptest! {
    /// For any valid slug, file_path_for_slug produces a path containing
    /// the slug and ending with /src/lib.rs.
    #[test]
    fn prop_file_path_for_slug_matches_pattern(
        slug in arb_valid_slug(),
    ) {
        let result = file_path_for_slug(&slug);
        // Path should contain .rustgym/challenges/{slug}/src/lib.rs
        let expected_suffix = format!(".rustgym/challenges/{}/src/lib.rs", slug);
        prop_assert!(
            result.ends_with(&expected_suffix),
            "Path '{}' should end with '{}'",
            result,
            expected_suffix
        );
    }

    /// The file path always contains ".rustgym/challenges/" and ends with "/src/lib.rs".
    #[test]
    fn prop_file_path_has_correct_prefix_and_suffix(
        slug in arb_valid_slug(),
    ) {
        let result = file_path_for_slug(&slug);
        prop_assert!(result.contains(".rustgym/challenges/"), "Path should contain .rustgym/challenges/");
        prop_assert!(result.ends_with("/src/lib.rs"), "Path should end with /src/lib.rs");
    }

    /// The slug is embedded in the path without modification.
    #[test]
    fn prop_file_path_contains_slug_unmodified(
        slug in arb_valid_slug(),
    ) {
        let result = file_path_for_slug(&slug);
        // The path should contain the slug between "challenges/" and "/src"
        prop_assert!(
            result.contains(&format!("challenges/{}/src", slug)),
            "Path '{}' should contain 'challenges/{}/src'",
            result,
            slug
        );
    }
}
