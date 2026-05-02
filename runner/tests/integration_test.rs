//! Integration tests for the RustGym runner service.
//!
//! These tests exercise the output parser and syntest modules without requiring
//! a running Docker daemon. They simulate the various scenarios a user submission
//! can produce: passing tests, failing tests, compile errors, and syntest violations.
//!
//! For full end-to-end Docker integration tests, see `tests/manual_test.sh` which
//! requires the sandbox image to be built and Docker to be available.

use rustgym_runner::output::parse_cargo_test_output;
use rustgym_runner::syntest::validate;

// =============================================================================
// Output Parser Tests — Passing Scenarios
// =============================================================================

#[test]
fn test_parse_passing_test_run() {
    let output = r#"
   Compiling challenge v0.1.0 (/workspace)
    Finished test [unoptimized + debuginfo] target(s) in 1.23s
     Running unittests src/lib.rs (target/debug/deps/challenge-abc123)

running 3 tests
test tests::test_add ... ok
test tests::test_subtract ... ok
test tests::test_multiply ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.passed));
    assert!(results.iter().all(|r| r.message.is_none()));

    let summary = summary.expect("should have a summary");
    assert_eq!(summary.total_passed, 3);
    assert_eq!(summary.total_failed, 0);
    assert_eq!(summary.total_ignored, 0);
}

#[test]
fn test_parse_single_passing_test() {
    let output = r#"
running 1 test
test tests::test_hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 1);
    assert!(results[0].passed);
    assert_eq!(results[0].name, "tests::test_hello");

    let summary = summary.unwrap();
    assert_eq!(summary.total_passed, 1);
}

// =============================================================================
// Output Parser Tests — Failing Scenarios
// =============================================================================

#[test]
fn test_parse_failing_test_run() {
    let output = r#"
   Compiling challenge v0.1.0 (/workspace)
    Finished test [unoptimized + debuginfo] target(s) in 0.89s
     Running unittests src/lib.rs (target/debug/deps/challenge-abc123)

running 3 tests
test tests::test_add ... ok
test tests::test_subtract ... FAILED
test tests::test_multiply ... ok

---- tests::test_subtract stdout ----
thread 'tests::test_subtract' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `3`', tests/tests.rs:15:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::test_subtract

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 3);

    let passing: Vec<_> = results.iter().filter(|r| r.passed).collect();
    assert_eq!(passing.len(), 2);

    let failing: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    assert_eq!(failing.len(), 1);
    assert_eq!(failing[0].name, "tests::test_subtract");
    assert!(failing[0].message.is_some());
    assert!(failing[0].message.as_ref().unwrap().contains("left: `5`"));

    let summary = summary.unwrap();
    assert_eq!(summary.total_passed, 2);
    assert_eq!(summary.total_failed, 1);
}

#[test]
fn test_parse_all_failing() {
    let output = r#"
running 2 tests
test tests::test_a ... FAILED
test tests::test_b ... FAILED

---- tests::test_a stdout ----
thread 'tests::test_a' panicked at 'not yet implemented', src/lib.rs:2:5

---- tests::test_b stdout ----
thread 'tests::test_b' panicked at 'not yet implemented', src/lib.rs:6:5

failures:
    tests::test_a
    tests::test_b

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| !r.passed));
    assert!(results.iter().all(|r| r.message.is_some()));

    let summary = summary.unwrap();
    assert_eq!(summary.total_passed, 0);
    assert_eq!(summary.total_failed, 2);
}

// =============================================================================
// Output Parser Tests — Compile Error Scenarios
// =============================================================================

#[test]
fn test_parse_compile_error_no_test_results() {
    // When code fails to compile, cargo test produces no test result lines.
    // The stderr would contain the error, but stdout is mostly empty.
    let stdout = "";
    let stderr = r#"error[E0308]: mismatched types
 --> src/lib.rs:3:5
  |
3 |     "hello"
  |     ^^^^^^^ expected `i32`, found `&str`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `challenge` due to previous error
"#;

    // The output parser only looks at stdout for test results
    let (results, summary) = parse_cargo_test_output(stdout);

    assert!(results.is_empty(), "compile errors produce no test results");
    assert!(summary.is_none(), "compile errors produce no summary");

    // Verify the stderr contains the compile error indicator
    assert!(stderr.contains("error[E"));
}

#[test]
fn test_parse_compile_error_with_partial_output() {
    // Sometimes cargo outputs some lines before the compile error
    let output = r#"
   Compiling challenge v0.1.0 (/workspace)
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert!(results.is_empty());
    assert!(summary.is_none());
}

// =============================================================================
// Syntest Validation Tests — Various Rule Combinations
// =============================================================================

#[test]
fn test_syntest_no_unwrap_rule_passes() {
    let code = r#"
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
"#;

    let violations = validate(code, &["no_unwrap".to_string()]).unwrap();
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_no_unwrap_rule_fails() {
    let code = r#"
pub fn risky_divide(a: i32, b: i32) -> i32 {
    Some(a / b).unwrap()
}
"#;

    let violations = validate(code, &["no_unwrap".to_string()]).unwrap();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule, "no_unwrap");
}

#[test]
fn test_syntest_no_clone_rule_passes() {
    let code = r#"
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
"#;

    let violations = validate(code, &["no_clone".to_string()]).unwrap();
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_no_clone_rule_fails() {
    let code = r#"
pub fn duplicate(s: &String) -> String {
    s.clone()
}
"#;

    let violations = validate(code, &["no_clone".to_string()]).unwrap();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule, "no_clone");
}

#[test]
fn test_syntest_must_use_match_passes() {
    let code = r#"
pub fn describe(opt: Option<i32>) -> &'static str {
    match opt {
        Some(_) => "has value",
        None => "empty",
    }
}
"#;

    let violations = validate(code, &["must_use_match".to_string()]).unwrap();
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_must_use_match_fails() {
    let code = r#"
pub fn describe(opt: Option<i32>) -> &'static str {
    if opt.is_some() { "has value" } else { "empty" }
}
"#;

    let violations = validate(code, &["must_use_match".to_string()]).unwrap();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule, "must_use_match");
}

#[test]
fn test_syntest_must_use_lifetime_passes() {
    let code = r#"
pub fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
"#;

    let violations = validate(code, &["must_use_lifetime".to_string()]).unwrap();
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_must_use_lifetime_fails() {
    let code = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

    let violations = validate(code, &["must_use_lifetime".to_string()]).unwrap();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].rule, "must_use_lifetime");
}

#[test]
fn test_syntest_multiple_rules_all_violated() {
    let code = r#"
pub fn bad_code(s: &String) -> String {
    let cloned = s.clone();
    let opt: Option<&str> = Some(&cloned);
    opt.unwrap().to_string()
}
"#;

    let rules = vec![
        "no_unwrap".to_string(),
        "no_clone".to_string(),
        "must_use_match".to_string(),
        "must_use_lifetime".to_string(),
    ];

    let violations = validate(code, &rules).unwrap();
    assert_eq!(violations.len(), 4);

    let rule_names: Vec<&str> = violations.iter().map(|v| v.rule.as_str()).collect();
    assert!(rule_names.contains(&"no_unwrap"));
    assert!(rule_names.contains(&"no_clone"));
    assert!(rule_names.contains(&"must_use_match"));
    assert!(rule_names.contains(&"must_use_lifetime"));
}

#[test]
fn test_syntest_multiple_rules_partial_violation() {
    let code = r#"
pub fn process<'a>(s: &'a str) -> &'a str {
    match s.len() {
        0 => "empty",
        _ => s,
    }
}
"#;

    let rules = vec![
        "no_unwrap".to_string(),
        "no_clone".to_string(),
        "must_use_match".to_string(),
        "must_use_lifetime".to_string(),
    ];

    let violations = validate(code, &rules).unwrap();
    // This code uses match and lifetime, doesn't use unwrap or clone
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_unknown_rule() {
    let code = r#"
pub fn hello() -> &'static str {
    "hello"
}
"#;

    let violations = validate(code, &["nonexistent_rule".to_string()]).unwrap();
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("Unknown syntest rule"));
}

#[test]
fn test_syntest_empty_rules() {
    let code = r#"
pub fn hello() -> &'static str {
    "hello"
}
"#;

    let violations = validate(code, &[]).unwrap();
    assert!(violations.is_empty());
}

#[test]
fn test_syntest_invalid_syntax() {
    let code = "fn broken( {";

    let result = validate(code, &["no_unwrap".to_string()]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse code"));
}

// =============================================================================
// Output Parser — Edge Cases
// =============================================================================

#[test]
fn test_parse_output_with_warnings_but_passing() {
    let output = r#"
   Compiling challenge v0.1.0 (/workspace)
warning: unused variable: `x`
 --> src/lib.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `challenge` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running unittests src/lib.rs (target/debug/deps/challenge-abc123)

running 1 test
test tests::test_it ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
"#;

    let (results, summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 1);
    assert!(results[0].passed);

    let summary = summary.unwrap();
    assert_eq!(summary.total_passed, 1);
    assert_eq!(summary.total_failed, 0);
}

#[test]
fn test_parse_output_with_multiple_test_modules() {
    let output = r#"
running 2 tests
test unit::test_a ... ok
test unit::test_b ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 2 tests
test integration::test_c ... ok
test integration::test_d ... FAILED

---- integration::test_d stdout ----
thread 'integration::test_d' panicked at 'assertion failed', tests/tests.rs:10:5

failures:
    integration::test_d

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
"#;

    let (results, _summary) = parse_cargo_test_output(output);

    assert_eq!(results.len(), 4);

    let passing: Vec<_> = results.iter().filter(|r| r.passed).collect();
    assert_eq!(passing.len(), 3);

    let failing: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    assert_eq!(failing.len(), 1);
    assert_eq!(failing[0].name, "integration::test_d");
}
