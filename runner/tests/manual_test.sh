#!/usr/bin/env bash
# Manual integration test for the RustGym runner service.
#
# Prerequisites:
#   - Docker daemon running
#   - Runner service running on localhost:3001
#
# Usage:
#   1. Build the sandbox image:
#      docker build -f runner/sandbox.Dockerfile -t rustgym-sandbox ./runner
#
#   2. Start the runner service:
#      cargo run --release (from the runner/ directory)
#      OR: docker-compose up runner
#
#   3. Run this script:
#      bash runner/tests/manual_test.sh

set -euo pipefail

RUNNER_URL="${RUNNER_URL:-http://localhost:3001}"
PASS=0
FAIL=0

# Helper: encode string to base64
b64() {
    echo -n "$1" | base64 -w0 2>/dev/null || echo -n "$1" | base64
}

# Helper: make a request and check the response
check_response() {
    local test_name="$1"
    local expected_status="$2"
    local response="$3"

    actual_status=$(echo "$response" | jq -r '.status // empty')
    if [ "$actual_status" = "$expected_status" ]; then
        echo "  ✓ $test_name (status: $actual_status)"
        PASS=$((PASS + 1))
    else
        echo "  ✗ $test_name (expected: $expected_status, got: $actual_status)"
        echo "    Response: $response"
        FAIL=$((FAIL + 1))
    fi
}

echo "=== RustGym Runner Manual Integration Tests ==="
echo "Runner URL: $RUNNER_URL"
echo ""

# --- Health Check ---
echo "1. Health check"
health=$(curl -s "$RUNNER_URL/health")
if echo "$health" | jq -e '.status == "ok"' > /dev/null 2>&1; then
    echo "  ✓ Health check passed"
    PASS=$((PASS + 1))
else
    echo "  ✗ Health check failed: $health"
    FAIL=$((FAIL + 1))
fi
echo ""

# --- Test: Passing code ---
echo "2. Passing test submission"
CODE=$(b64 'pub fn add(a: i32, b: i32) -> i32 { a + b }')
TEST=$(b64 '#[cfg(test)] mod tests { use super::*; #[test] fn test_add() { assert_eq!(add(2, 3), 5); } }')
CARGO=$(b64 '[package]
name = "challenge"
version = "0.1.0"
edition = "2021"')

response=$(curl -s -X POST "$RUNNER_URL/run/test" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\", \"test_b64\": \"$TEST\", \"cargo_toml_b64\": \"$CARGO\"}")
check_response "Passing submission" "passed" "$response"
echo ""

# --- Test: Failing code ---
echo "3. Failing test submission"
CODE=$(b64 'pub fn add(a: i32, b: i32) -> i32 { a - b }')

response=$(curl -s -X POST "$RUNNER_URL/run/test" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\", \"test_b64\": \"$TEST\", \"cargo_toml_b64\": \"$CARGO\"}")
check_response "Failing submission" "failed" "$response"
echo ""

# --- Test: Compile error ---
echo "4. Compile error submission"
CODE=$(b64 'pub fn add(a: i32, b: i32) -> i32 { a + }')

response=$(curl -s -X POST "$RUNNER_URL/run/test" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\", \"test_b64\": \"$TEST\", \"cargo_toml_b64\": \"$CARGO\"}")
# Compile errors may show as "compile_error" or "runtime_error" depending on exit code
actual_status=$(echo "$response" | jq -r '.status // empty')
if [ "$actual_status" = "compile_error" ] || [ "$actual_status" = "runtime_error" ]; then
    echo "  ✓ Compile error submission (status: $actual_status)"
    PASS=$((PASS + 1))
else
    echo "  ✗ Compile error submission (expected: compile_error, got: $actual_status)"
    FAIL=$((FAIL + 1))
fi
echo ""

# --- Test: Timeout ---
echo "5. Timeout submission"
CODE=$(b64 'pub fn infinite() { loop {} }')
TEST=$(b64 '#[cfg(test)] mod tests { use super::*; #[test] fn test_timeout() { infinite(); } }')

response=$(curl -s -X POST "$RUNNER_URL/run/test" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\", \"test_b64\": \"$TEST\", \"cargo_toml_b64\": \"$CARGO\", \"timeout_secs\": 5}")
check_response "Timeout submission" "timeout" "$response"
echo ""

# --- Test: Syntest violation ---
echo "6. Syntest violation (no_unwrap)"
CODE=$(b64 'pub fn risky() -> i32 { Some(42).unwrap() }')
TEST=$(b64 '#[cfg(test)] mod tests { use super::*; #[test] fn test_risky() { assert_eq!(risky(), 42); } }')

response=$(curl -s -X POST "$RUNNER_URL/run/test" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\", \"test_b64\": \"$TEST\", \"cargo_toml_b64\": \"$CARGO\", \"syntest_rules\": [\"no_unwrap\"]}")
check_response "Syntest violation" "syntest_failed" "$response"
echo ""

# --- Test: Playground ---
echo "7. Playground execution"
CODE=$(b64 'fn main() { println!("Hello from RustGym!"); }')

response=$(curl -s -X POST "$RUNNER_URL/run/playground" \
    -H "Content-Type: application/json" \
    -d "{\"code_b64\": \"$CODE\"}")
check_response "Playground execution" "success" "$response"
echo ""

# --- Summary ---
echo "=== Results ==="
echo "  Passed: $PASS"
echo "  Failed: $FAIL"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo "SOME TESTS FAILED"
    exit 1
else
    echo "ALL TESTS PASSED"
    exit 0
fi
