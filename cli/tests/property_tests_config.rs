// Feature: cli-tui-rewrite, Property 16: API URL resolution with default

use proptest::prelude::*;

use rustgym_cli::config::CliConfig;

// ─── Helpers ────────────────────────────────────────────────────────────────

const DEFAULT_API_URL: &str = "https://rustgym.dev";

/// Strategy to generate non-empty URL strings (simulating valid API URLs).
fn arb_non_empty_url() -> impl Strategy<Value = String> {
    "https?://[a-z][a-z0-9\\-]{1,20}\\.[a-z]{2,5}(:[0-9]{2,5})?"
}

/// Strategy to generate an optional non-empty URL (Some(url) or None).
fn arb_optional_url() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some(String::new())),
        arb_non_empty_url().prop_map(Some),
    ]
}

/// Strategy to generate an optional CLI override (Some(url), Some(""), or None).
fn arb_cli_override() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some(String::new())),
        arb_non_empty_url().prop_map(Some),
    ]
}

// ─── Property 16: API URL resolution with default ───────────────────────────

// **Validates: Requirements 12.1, 12.2**
//
// For any config file state, if `api_url` is set and non-empty the resolved
// URL SHALL equal that value; if `api_url` is `None` or empty, the resolved
// URL SHALL equal "https://rustgym.dev". A `--api-url` CLI argument SHALL
// take precedence over both.

proptest! {
    /// Test case 1: CLI override (non-empty) always wins over config value.
    /// When a non-empty CLI override is provided, it takes precedence regardless
    /// of what the config contains.
    #[test]
    fn prop_cli_override_wins_over_config(
        cli_url in arb_non_empty_url(),
        config_url in arb_optional_url(),
    ) {
        let config = CliConfig {
            token: None,
            api_url: config_url,
        };
        let result = config.resolve_api_url(Some(&cli_url));
        prop_assert_eq!(result, cli_url);
    }

    /// Test case 2: Config value (non-empty) wins when no CLI override.
    /// When no CLI override is provided and config has a non-empty api_url,
    /// the config value is used.
    #[test]
    fn prop_config_value_used_when_no_cli_override(
        config_url in arb_non_empty_url(),
    ) {
        let config = CliConfig {
            token: None,
            api_url: Some(config_url.clone()),
        };
        let result = config.resolve_api_url(None);
        prop_assert_eq!(result, config_url);
    }

    /// Test case 3: Default "https://rustgym.dev" used when both are None/empty.
    /// When CLI override is None and config api_url is None or empty,
    /// the default URL is returned.
    #[test]
    fn prop_default_used_when_both_none_or_empty(
        config_url in prop_oneof![Just(None), Just(Some(String::new()))],
        cli_override in prop_oneof![Just(None), Just(Some(String::new()))],
    ) {
        let config = CliConfig {
            token: None,
            api_url: config_url,
        };
        let result = config.resolve_api_url(cli_override.as_deref());
        prop_assert_eq!(result, DEFAULT_API_URL);
    }

    /// Test case 4: Empty CLI override falls through to config value.
    /// When CLI override is an empty string, it should not take precedence;
    /// the config value should be used instead.
    #[test]
    fn prop_empty_cli_override_falls_through_to_config(
        config_url in arb_non_empty_url(),
    ) {
        let config = CliConfig {
            token: None,
            api_url: Some(config_url.clone()),
        };
        let result = config.resolve_api_url(Some(""));
        prop_assert_eq!(result, config_url);
    }

    /// Test case 5: Empty config value falls through to default.
    /// When config api_url is an empty string and no CLI override is provided,
    /// the default URL is returned.
    #[test]
    fn prop_empty_config_falls_through_to_default(
        // Generate various empty/None combinations for CLI override
        cli_override in prop_oneof![Just(None), Just(Some(String::new()))],
    ) {
        let config = CliConfig {
            token: None,
            api_url: Some(String::new()),
        };
        let result = config.resolve_api_url(cli_override.as_deref());
        prop_assert_eq!(result, DEFAULT_API_URL);
    }

    /// Combined property: the resolution priority is always
    /// non-empty CLI > non-empty config > default.
    #[test]
    fn prop_resolution_priority_holds(
        cli_override in arb_cli_override(),
        config_url in arb_optional_url(),
    ) {
        let config = CliConfig {
            token: None,
            api_url: config_url.clone(),
        };
        let result = config.resolve_api_url(cli_override.as_deref());

        // Determine expected value based on priority
        let expected = if let Some(ref cli) = cli_override {
            if !cli.is_empty() {
                cli.clone()
            } else if let Some(ref cfg) = config_url {
                if !cfg.is_empty() {
                    cfg.clone()
                } else {
                    DEFAULT_API_URL.to_string()
                }
            } else {
                DEFAULT_API_URL.to_string()
            }
        } else if let Some(ref cfg) = config_url {
            if !cfg.is_empty() {
                cfg.clone()
            } else {
                DEFAULT_API_URL.to_string()
            }
        } else {
            DEFAULT_API_URL.to_string()
        };

        prop_assert_eq!(result, expected);
    }
}
