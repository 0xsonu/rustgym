/// Result of a single test case parsed from cargo test output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestResult {
    /// Fully qualified test name (e.g., "tests::test_add")
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Failure message (captured stdout), if the test failed
    pub message: Option<String>,
}

/// Summary of the overall test run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestSummary {
    pub total_passed: usize,
    pub total_failed: usize,
    pub total_ignored: usize,
}

/// Parse the stdout output from `cargo test --no-fail-fast`.
///
/// Extracts per-test pass/fail results and failure messages.
pub fn parse_cargo_test_output(stdout: &str) -> (Vec<TestResult>, Option<TestSummary>) {
    let mut results: Vec<TestResult> = Vec::new();
    let mut summary: Option<TestSummary> = None;

    // First pass: extract test results from lines like:
    //   test test_name ... ok
    //   test test_name ... FAILED
    for line in stdout.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("test ") {
            if let Some(name) = rest.strip_suffix(" ... ok") {
                results.push(TestResult {
                    name: name.trim().to_string(),
                    passed: true,
                    message: None,
                });
            } else if let Some(name) = rest.strip_suffix(" ... FAILED") {
                results.push(TestResult {
                    name: name.trim().to_string(),
                    passed: false,
                    message: None,
                });
            }
        }

        // Parse summary line: "test result: ok. X passed; Y failed; Z ignored; ..."
        if trimmed.starts_with("test result:") {
            summary = parse_summary_line(trimmed);
        }
    }

    // Second pass: extract failure messages from stdout sections
    let failure_messages = extract_failure_messages(stdout);
    for result in &mut results {
        if !result.passed {
            if let Some(msg) = failure_messages.get(result.name.as_str()) {
                result.message = Some(msg.clone());
            }
        }
    }

    (results, summary)
}

/// Parse the summary line from cargo test output.
/// Format: "test result: ok. X passed; Y failed; Z ignored; ..."
/// or:     "test result: FAILED. X passed; Y failed; Z ignored; ..."
fn parse_summary_line(line: &str) -> Option<TestSummary> {
    // Find the part after "test result: ok. " or "test result: FAILED. "
    let stats_part = line
        .strip_prefix("test result: ok. ")
        .or_else(|| line.strip_prefix("test result: FAILED. "))?;

    let mut passed = 0;
    let mut failed = 0;
    let mut ignored = 0;

    for segment in stats_part.split(';') {
        let segment = segment.trim();
        if let Some(num_str) = segment.strip_suffix(" passed") {
            passed = num_str.trim().parse().unwrap_or(0);
        } else if let Some(num_str) = segment.strip_suffix(" failed") {
            failed = num_str.trim().parse().unwrap_or(0);
        } else if let Some(num_str) = segment.strip_suffix(" ignored") {
            ignored = num_str.trim().parse().unwrap_or(0);
        }
    }

    Some(TestSummary {
        total_passed: passed,
        total_failed: failed,
        total_ignored: ignored,
    })
}

/// Extract failure messages from cargo test output.
///
/// Failure messages appear between:
///   ---- test_name stdout ----
/// and the next `----` separator or `failures:` section.
fn extract_failure_messages(stdout: &str) -> std::collections::HashMap<&str, String> {
    let mut messages: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Look for "---- test_name stdout ----"
        if trimmed.starts_with("---- ") && trimmed.ends_with(" stdout ----") {
            let test_name = trimmed
                .strip_prefix("---- ")
                .and_then(|s| s.strip_suffix(" stdout ----"))
                .unwrap_or("");

            if !test_name.is_empty() {
                let mut msg_lines: Vec<&str> = Vec::new();
                i += 1;

                // Collect lines until the next separator
                while i < lines.len() {
                    let next = lines[i].trim();
                    if next.starts_with("---- ") || next == "failures:" || next == "note:" {
                        break;
                    }
                    msg_lines.push(lines[i]);
                    i += 1;
                }

                let message = msg_lines.join("\n").trim().to_string();
                if !message.is_empty() {
                    messages.insert(test_name, message);
                }
                continue;
            }
        }

        i += 1;
    }

    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_passing() {
        let output = r#"
running 3 tests
test tests::test_add ... ok
test tests::test_sub ... ok
test tests::test_mul ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
"#;

        let (results, summary) = parse_cargo_test_output(output);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.passed));
        assert!(results.iter().all(|r| r.message.is_none()));

        let summary = summary.unwrap();
        assert_eq!(summary.total_passed, 3);
        assert_eq!(summary.total_failed, 0);
        assert_eq!(summary.total_ignored, 0);
    }

    #[test]
    fn test_parse_with_failures() {
        let output = r#"
running 2 tests
test tests::test_add ... ok
test tests::test_sub ... FAILED

---- tests::test_sub stdout ----
thread 'tests::test_sub' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `3`', src/lib.rs:10:5

failures:
    tests::test_sub

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
"#;

        let (results, summary) = parse_cargo_test_output(output);
        assert_eq!(results.len(), 2);

        let passed = results
            .iter()
            .find(|r| r.name == "tests::test_add")
            .unwrap();
        assert!(passed.passed);
        assert!(passed.message.is_none());

        let failed = results
            .iter()
            .find(|r| r.name == "tests::test_sub")
            .unwrap();
        assert!(!failed.passed);
        assert!(failed.message.is_some());
        assert!(failed
            .message
            .as_ref()
            .unwrap()
            .contains("assertion failed"));

        let summary = summary.unwrap();
        assert_eq!(summary.total_passed, 1);
        assert_eq!(summary.total_failed, 1);
    }

    #[test]
    fn test_parse_empty_output() {
        let (results, summary) = parse_cargo_test_output("");
        assert!(results.is_empty());
        assert!(summary.is_none());
    }

    #[test]
    fn test_parse_with_ignored() {
        let output = r#"
running 3 tests
test tests::test_add ... ok
test tests::test_ignored ... ok
test tests::test_sub ... ok

test result: ok. 3 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
"#;

        let (_, summary) = parse_cargo_test_output(output);
        let summary = summary.unwrap();
        assert_eq!(summary.total_passed, 3);
        assert_eq!(summary.total_ignored, 2);
    }
}
