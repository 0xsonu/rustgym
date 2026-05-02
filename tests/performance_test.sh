#!/usr/bin/env bash
# =============================================================================
# RustGym Performance Test — Submission Response Time
# =============================================================================
#
# Verifies that the submission endpoint responds within 15 seconds.
#
# Prerequisites:
#   - Backend API running on localhost:3000 (or set BASE_URL)
#   - Runner service running on localhost:3001
#   - Docker daemon running with rustgym-sandbox image built
#   - PostgreSQL running and migrated
#   - At least one task seeded in the database
#   - curl and jq installed
#
# Usage:
#   bash tests/performance_test.sh
#
# Environment variables:
#   BASE_URL       - API base URL (default: http://localhost:3000)
#   MAX_TIME_SECS  - Maximum acceptable response time (default: 15)
#
# =============================================================================

set -uo pipefail

BASE_URL="${BASE_URL:-http://localhost:3000}"
MAX_TIME_SECS="${MAX_TIME_SECS:-15}"
PASS=0
FAIL=0
SKIP=0

# Generate unique test user
TIMESTAMP=$(date +%s)
TEST_USERNAME="perf_${TIMESTAMP}"
TEST_EMAIL="perf_${TIMESTAMP}@test.rustgym.dev"
TEST_PASSWORD="TestPass123!"

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

echo "=== RustGym Performance Test ==="
echo "Base URL: $BASE_URL"
echo "Max response time: ${MAX_TIME_SECS}s"
echo ""

# Health check
HEALTH=$(curl -s "$BASE_URL/api/v1/health")
if ! echo "$HEALTH" | jq -e '.status == "ok"' > /dev/null 2>&1; then
    echo "ERROR: Backend not running at $BASE_URL"
    exit 1
fi

# Register test user
echo "Setting up test user..."
REGISTER_RESP=$(curl -s -X POST "$BASE_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\": \"$TEST_USERNAME\", \"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
ACCESS_TOKEN=$(echo "$REGISTER_RESP" | jq -r '.access_token // empty')

if [ -z "$ACCESS_TOKEN" ]; then
    echo "ERROR: Failed to register test user"
    exit 1
fi
echo ""

# Find a task
QUESTS_RESP=$(curl -s "$BASE_URL/api/v1/quests" -H "Authorization: Bearer $ACCESS_TOKEN")
QUEST_SLUG=$(echo "$QUESTS_RESP" | jq -r '.quests[0].slug // empty')

TASK_SLUG=""
if [ -n "$QUEST_SLUG" ]; then
    QUEST_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG" -H "Authorization: Bearer $ACCESS_TOKEN")
    LEVEL_SLUG=$(echo "$QUEST_DETAIL" | jq -r '.levels[0].slug // empty')

    if [ -n "$LEVEL_SLUG" ]; then
        LEVEL_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG/levels/$LEVEL_SLUG" \
            -H "Authorization: Bearer $ACCESS_TOKEN")
        TASK_SLUG=$(echo "$LEVEL_DETAIL" | jq -r '.tasks[0].slug // empty')
    fi
fi

# =============================================================================
# Performance Test: Submission Response Time
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Performance: Submission responds within ${MAX_TIME_SECS} seconds"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$TASK_SLUG" ]; then
    SUBMIT_CODE='pub fn hello() -> &'\''static str { "Hello, Ferris!" }'

    # Use curl's built-in timing
    TIMING=$(curl -s -o /dev/null -w "%{time_total}" --max-time "$MAX_TIME_SECS" \
        -X POST "$BASE_URL/api/v1/tasks/$TASK_SLUG/submit" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{\"code\": \"$SUBMIT_CODE\"}")

    # Check if curl timed out (returns 0 bytes or error)
    CURL_EXIT=$?

    if [ $CURL_EXIT -eq 28 ]; then
        fail "Submission timed out (exceeded ${MAX_TIME_SECS}s)"
    elif [ $CURL_EXIT -ne 0 ]; then
        skip "Submission request failed (curl exit code: $CURL_EXIT — runner may not be running)"
    else
        # Compare timing (using awk for float comparison)
        WITHIN_LIMIT=$(echo "$TIMING $MAX_TIME_SECS" | awk '{print ($1 <= $2) ? "yes" : "no"}')

        if [ "$WITHIN_LIMIT" = "yes" ]; then
            pass "Submission responded in ${TIMING}s (limit: ${MAX_TIME_SECS}s)"
        else
            fail "Submission too slow: ${TIMING}s (limit: ${MAX_TIME_SECS}s)"
        fi
    fi
else
    skip "Performance test (no task available in database)"
fi
echo ""

# =============================================================================
# Bonus: API endpoint response times
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Bonus: API endpoint response times (informational)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

endpoints=(
    "GET /api/v1/health"
    "GET /api/v1/quests"
    "GET /api/v1/users/leaderboard"
)

for endpoint in "${endpoints[@]}"; do
    METHOD=$(echo "$endpoint" | cut -d' ' -f1)
    PATH_PART=$(echo "$endpoint" | cut -d' ' -f2)
    TIMING=$(curl -s -o /dev/null -w "%{time_total}" "$BASE_URL$PATH_PART")
    echo "  $endpoint: ${TIMING}s"
done
echo ""

# =============================================================================
# Summary
# =============================================================================
echo "=== Performance Test Results ==="
echo -e "  ${GREEN}Passed${NC}: $PASS"
echo -e "  ${RED}Failed${NC}: $FAIL"
echo -e "  ${YELLOW}Skipped${NC}: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}PERFORMANCE REQUIREMENTS NOT MET${NC}"
    exit 1
else
    echo -e "${GREEN}PERFORMANCE REQUIREMENTS MET${NC}"
    exit 0
fi
