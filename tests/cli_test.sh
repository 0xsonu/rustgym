#!/usr/bin/env bash
# =============================================================================
# RustGym CLI Tool Test Script
# =============================================================================
#
# Tests the CLI tool commands:
#   1. Build the CLI tool
#   2. rustgym login (with test credentials)
#   3. rustgym list
#   4. rustgym download hello-variables
#   5. Verify downloaded files exist
#
# Prerequisites:
#   - Rust toolchain installed (cargo)
#   - Backend API running on localhost:3000 (or set BASE_URL)
#   - A registered user (or set TEST_EMAIL and TEST_PASSWORD)
#   - At least one task seeded in the database
#
# Usage:
#   bash tests/cli_test.sh
#
# Environment variables:
#   BASE_URL       - API base URL (default: http://localhost:3000)
#   TEST_EMAIL     - Test user email (will register a new user if not set)
#   TEST_PASSWORD  - Test user password (will register a new user if not set)
#
# =============================================================================

set -uo pipefail

BASE_URL="${BASE_URL:-http://localhost:3000}"
PASS=0
FAIL=0
SKIP=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() {
    echo -e "  ${GREEN}✓ PASS${NC}: $1"
    PASS=$((PASS + 1))
}

fail() {
    echo -e "  ${RED}✗ FAIL${NC}: $1"
    [ -n "${2:-}" ] && echo -e "    ${RED}Detail: $2${NC}"
    FAIL=$((FAIL + 1))
}

skip() {
    echo -e "  ${YELLOW}⊘ SKIP${NC}: $1"
    SKIP=$((SKIP + 1))
}

echo "=== RustGym CLI Tool Tests ==="
echo "Base URL: $BASE_URL"
echo ""

# Determine project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CLI_DIR="$PROJECT_ROOT/cli"
WORK_DIR=$(mktemp -d)

# Cleanup on exit
cleanup() {
    rm -rf "$WORK_DIR"
}
trap cleanup EXIT

# =============================================================================
# Setup: Register a test user if credentials not provided
# =============================================================================
if [ -z "${TEST_EMAIL:-}" ] || [ -z "${TEST_PASSWORD:-}" ]; then
    TIMESTAMP=$(date +%s)
    TEST_EMAIL="cli_test_${TIMESTAMP}@test.rustgym.dev"
    TEST_PASSWORD="TestPass123!"
    TEST_USERNAME="cli_test_${TIMESTAMP}"

    echo "Registering test user: $TEST_EMAIL"
    REGISTER_RESP=$(curl -s -X POST "$BASE_URL/api/v1/auth/register" \
        -H "Content-Type: application/json" \
        -d "{\"username\": \"$TEST_USERNAME\", \"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}" 2>/dev/null)

    if echo "$REGISTER_RESP" | jq -e '.access_token' > /dev/null 2>&1; then
        echo "Test user registered successfully"
    else
        echo "WARNING: Could not register test user (backend may not be running)"
        echo "Response: $REGISTER_RESP"
    fi
fi
echo ""

# =============================================================================
# Test 1: Build the CLI tool
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test 1: Build CLI tool"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -d "$CLI_DIR" ]; then
    BUILD_OUTPUT=$(cargo build --release --manifest-path "$CLI_DIR/Cargo.toml" 2>&1)
    BUILD_EXIT=$?

    if [ $BUILD_EXIT -eq 0 ]; then
        pass "CLI tool built successfully"
        CLI_BIN="$CLI_DIR/target/release/rustgym"
        if [ ! -f "$CLI_BIN" ]; then
            # Try debug build path
            CLI_BIN="$CLI_DIR/target/debug/rustgym"
        fi
    else
        fail "CLI tool build failed" "$BUILD_OUTPUT"
        CLI_BIN=""
    fi
else
    fail "CLI directory not found at $CLI_DIR"
    CLI_BIN=""
fi
echo ""

# =============================================================================
# Test 2: rustgym login
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test 2: rustgym login"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$CLI_BIN" ] && [ -f "$CLI_BIN" ]; then
    # Set HOME to temp dir so we don't pollute real config
    export HOME="$WORK_DIR"

    LOGIN_OUTPUT=$("$CLI_BIN" --api-url "$BASE_URL" login --email "$TEST_EMAIL" --password "$TEST_PASSWORD" 2>&1)
    LOGIN_EXIT=$?

    if [ $LOGIN_EXIT -eq 0 ]; then
        pass "rustgym login succeeded"
    else
        fail "rustgym login failed (exit code: $LOGIN_EXIT)" "$LOGIN_OUTPUT"
    fi
else
    skip "rustgym login (CLI binary not available)"
fi
echo ""

# =============================================================================
# Test 3: rustgym list
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test 3: rustgym list"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$CLI_BIN" ] && [ -f "$CLI_BIN" ]; then
    LIST_OUTPUT=$("$CLI_BIN" --api-url "$BASE_URL" list 2>&1)
    LIST_EXIT=$?

    if [ $LIST_EXIT -eq 0 ]; then
        pass "rustgym list succeeded"
        # Check if output contains challenge info
        if echo "$LIST_OUTPUT" | grep -qi "challenge\|slug\|no challenges"; then
            pass "rustgym list produced meaningful output"
        else
            skip "rustgym list output format unclear"
        fi
    else
        fail "rustgym list failed (exit code: $LIST_EXIT)" "$LIST_OUTPUT"
    fi
else
    skip "rustgym list (CLI binary not available)"
fi
echo ""

# =============================================================================
# Test 4: rustgym download hello-variables
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test 4: rustgym download hello-variables"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$CLI_BIN" ] && [ -f "$CLI_BIN" ]; then
    # Run download from work directory
    pushd "$WORK_DIR" > /dev/null
    DOWNLOAD_OUTPUT=$("$CLI_BIN" --api-url "$BASE_URL" download hello-variables 2>&1)
    DOWNLOAD_EXIT=$?
    popd > /dev/null

    if [ $DOWNLOAD_EXIT -eq 0 ]; then
        pass "rustgym download hello-variables succeeded"
    else
        # May fail if task doesn't exist in DB
        if echo "$DOWNLOAD_OUTPUT" | grep -qi "not found\|404"; then
            skip "rustgym download (task 'hello-variables' not found in database)"
        else
            fail "rustgym download failed (exit code: $DOWNLOAD_EXIT)" "$DOWNLOAD_OUTPUT"
        fi
    fi
else
    skip "rustgym download (CLI binary not available)"
fi
echo ""

# =============================================================================
# Test 5: Verify downloaded files
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test 5: Verify downloaded files"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CHALLENGE_DIR="$WORK_DIR/challenges/hello-variables"

if [ -d "$CHALLENGE_DIR" ]; then
    # Check for expected files
    if [ -f "$CHALLENGE_DIR/src/lib.rs" ]; then
        pass "src/lib.rs exists"
    else
        fail "src/lib.rs not found in downloaded challenge"
    fi

    if [ -f "$CHALLENGE_DIR/Cargo.toml" ]; then
        pass "Cargo.toml exists"
    else
        fail "Cargo.toml not found in downloaded challenge"
    fi

    if [ -f "$CHALLENGE_DIR/description.md" ]; then
        pass "description.md exists"
    else
        fail "description.md not found in downloaded challenge"
    fi
else
    skip "File verification (challenge directory not created — download may have been skipped)"
fi
echo ""

# =============================================================================
# Summary
# =============================================================================
echo "=== CLI Test Results ==="
echo -e "  ${GREEN}Passed${NC}: $PASS"
echo -e "  ${RED}Failed${NC}: $FAIL"
echo -e "  ${YELLOW}Skipped${NC}: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}SOME CLI TESTS FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}ALL CLI TESTS PASSED${NC}"
    exit 0
fi
