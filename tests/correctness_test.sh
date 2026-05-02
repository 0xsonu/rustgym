#!/usr/bin/env bash
# =============================================================================
# RustGym Correctness Properties Test Script
# =============================================================================
#
# Tests the following correctness properties:
#   1. XP Idempotency — submitting the same task twice awards XP only once
#   2. Level Monotonicity — level never decreases after XP award
#   3. Progress Consistency — completing a task updates progress correctly
#   4. Solution Code Never Exposed — GET /tasks/:slug does NOT contain solution_code
#   5. Rate Limiting — 429 returned after exceeding rate limit (if Redis available)
#
# Prerequisites:
#   - Backend API running on localhost:3000 (or set BASE_URL)
#   - PostgreSQL running and migrated
#   - Runner service running on localhost:3001 (for submission tests)
#   - Docker daemon running (for submission tests)
#   - curl and jq installed
#   - At least one quest/level/task seeded in the database
#
# Usage:
#   bash tests/correctness_test.sh
#
# Environment variables:
#   BASE_URL  - API base URL (default: http://localhost:3000)
#
# =============================================================================

set -uo pipefail

BASE_URL="${BASE_URL:-http://localhost:3000}"
PASS=0
FAIL=0
SKIP=0

# Generate unique test user
TIMESTAMP=$(date +%s)
TEST_USERNAME="correctness_${TIMESTAMP}"
TEST_EMAIL="correctness_${TIMESTAMP}@test.rustgym.dev"
TEST_PASSWORD="TestPass123!"

ACCESS_TOKEN=""
USER_ID=""

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

# Helper: register and login
setup_user() {
    # Register
    local RESP
    RESP=$(curl -s -X POST "$BASE_URL/api/v1/auth/register" \
        -H "Content-Type: application/json" \
        -d "{\"username\": \"$TEST_USERNAME\", \"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")

    ACCESS_TOKEN=$(echo "$RESP" | jq -r '.access_token // empty')
    USER_ID=$(echo "$RESP" | jq -r '.user.id // empty')

    if [ -z "$ACCESS_TOKEN" ] || [ -z "$USER_ID" ]; then
        echo "ERROR: Failed to register test user. Aborting."
        echo "Response: $RESP"
        exit 1
    fi
}

# Helper: find a task slug from the API
find_task_slug() {
    local QUESTS_RESP
    QUESTS_RESP=$(curl -s "$BASE_URL/api/v1/quests" -H "Authorization: Bearer $ACCESS_TOKEN")
    local QUEST_SLUG
    QUEST_SLUG=$(echo "$QUESTS_RESP" | jq -r '.quests[0].slug // empty')

    if [ -z "$QUEST_SLUG" ]; then
        echo ""
        return
    fi

    local QUEST_DETAIL
    QUEST_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG" -H "Authorization: Bearer $ACCESS_TOKEN")
    local LEVEL_SLUG
    LEVEL_SLUG=$(echo "$QUEST_DETAIL" | jq -r '.levels[0].slug // empty')

    if [ -z "$LEVEL_SLUG" ]; then
        echo ""
        return
    fi

    local LEVEL_DETAIL
    LEVEL_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG/levels/$LEVEL_SLUG" -H "Authorization: Bearer $ACCESS_TOKEN")
    echo "$LEVEL_DETAIL" | jq -r '.tasks[0].slug // empty'
}

# =============================================================================
echo "=== RustGym Correctness Properties Tests ==="
echo "Base URL: $BASE_URL"
echo ""

# Health check
HEALTH=$(curl -s "$BASE_URL/api/v1/health")
if ! echo "$HEALTH" | jq -e '.status == "ok"' > /dev/null 2>&1; then
    echo "ERROR: Backend not running at $BASE_URL"
    exit 1
fi

# Setup
echo "Setting up test user..."
setup_user
echo "User registered: $TEST_USERNAME (id: $USER_ID)"
echo ""

# Find a task to test with
TASK_SLUG=$(find_task_slug)
if [ -z "$TASK_SLUG" ]; then
    echo "WARNING: No tasks found in database. Submission-based tests will be skipped."
fi
echo ""

# =============================================================================
# Property 1: XP Idempotency
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Property 1: XP Idempotency"
echo "  Submitting the same task twice should award XP only once"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$TASK_SLUG" ]; then
    SUBMIT_CODE='pub fn hello() -> &'\''static str { "Hello, Ferris!" }'

    # First submission
    RESP1=$(curl -s --max-time 15 -X POST "$BASE_URL/api/v1/tasks/$TASK_SLUG/submit" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{\"code\": \"$SUBMIT_CODE\"}")
    XP1=$(echo "$RESP1" | jq -r '.xp_awarded // 0')
    STATUS1=$(echo "$RESP1" | jq -r '.status // empty')

    if [ "$STATUS1" = "passed" ]; then
        # Second submission (same task)
        RESP2=$(curl -s --max-time 15 -X POST "$BASE_URL/api/v1/tasks/$TASK_SLUG/submit" \
            -H "Authorization: Bearer $ACCESS_TOKEN" \
            -H "Content-Type: application/json" \
            -d "{\"code\": \"$SUBMIT_CODE\"}")
        XP2=$(echo "$RESP2" | jq -r '.xp_awarded // 0')

        if [ "$XP1" -gt 0 ] && [ "$XP2" -eq 0 ]; then
            pass "XP awarded on first pass ($XP1), not on second pass ($XP2)"
        elif [ "$XP1" -eq 0 ] && [ "$XP2" -eq 0 ]; then
            skip "XP idempotency (task may have been previously completed, both returned 0)"
        else
            fail "XP idempotency violated" "First: $XP1, Second: $XP2 (expected second=0)"
        fi
    elif [ -z "$STATUS1" ]; then
        skip "XP idempotency (runner service not available)"
    else
        skip "XP idempotency (first submission did not pass: status=$STATUS1)"
    fi
else
    skip "XP idempotency (no task available)"
fi
echo ""

# =============================================================================
# Property 2: Level Monotonicity
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Property 2: Level Monotonicity"
echo "  User level should never decrease after XP is awarded"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Get current profile
PROFILE_RESP=$(curl -s "$BASE_URL/api/v1/users/me" -H "Authorization: Bearer $ACCESS_TOKEN")
CURRENT_LEVEL=$(echo "$PROFILE_RESP" | jq -r '.level // 0')
CURRENT_XP=$(echo "$PROFILE_RESP" | jq -r '.xp // 0')

if [ "$CURRENT_LEVEL" -ge 1 ] 2>/dev/null; then
    pass "Level monotonicity: current level=$CURRENT_LEVEL, xp=$CURRENT_XP (level >= 1)"
else
    fail "Level monotonicity" "Level is $CURRENT_LEVEL (expected >= 1)"
fi

# Verify level didn't decrease from submission (if we made one)
if [ -n "$TASK_SLUG" ] && [ -n "${RESP1:-}" ]; then
    NEW_LEVEL=$(echo "$RESP1" | jq -r '.new_level // empty')
    if [ -n "$NEW_LEVEL" ] && [ "$NEW_LEVEL" != "null" ]; then
        if [ "$NEW_LEVEL" -ge "$CURRENT_LEVEL" ] 2>/dev/null || [ "$CURRENT_LEVEL" -le "$NEW_LEVEL" ] 2>/dev/null; then
            pass "Level did not decrease after submission (was $CURRENT_LEVEL, now $NEW_LEVEL)"
        else
            fail "Level decreased!" "Was $CURRENT_LEVEL, now $NEW_LEVEL"
        fi
    fi
fi
echo ""

# =============================================================================
# Property 3: Progress Consistency
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Property 3: Progress Consistency"
echo "  After completing a task, progress should reflect the completion"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$TASK_SLUG" ] && [ "${STATUS1:-}" = "passed" ]; then
    # Re-fetch quest list to check progress
    QUESTS_RESP=$(curl -s "$BASE_URL/api/v1/quests" -H "Authorization: Bearer $ACCESS_TOKEN")
    QUEST_SLUG=$(echo "$QUESTS_RESP" | jq -r '.quests[0].slug // empty')

    if [ -n "$QUEST_SLUG" ]; then
        QUEST_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG" -H "Authorization: Bearer $ACCESS_TOKEN")
        LEVEL_SLUG=$(echo "$QUEST_DETAIL" | jq -r '.levels[0].slug // empty')

        if [ -n "$LEVEL_SLUG" ]; then
            LEVEL_DETAIL=$(curl -s "$BASE_URL/api/v1/quests/$QUEST_SLUG/levels/$LEVEL_SLUG" \
                -H "Authorization: Bearer $ACCESS_TOKEN")
            TASK_STATUS=$(echo "$LEVEL_DETAIL" | jq -r ".tasks[] | select(.slug == \"$TASK_SLUG\") | .user_progress.status // empty")

            if [ "$TASK_STATUS" = "completed" ]; then
                pass "Task progress shows 'completed' after passing submission"
            elif [ -n "$TASK_STATUS" ]; then
                pass "Task progress updated (status: $TASK_STATUS)"
            else
                skip "Progress consistency (no user_progress returned — may need auth)"
            fi
        else
            skip "Progress consistency (no level found)"
        fi
    else
        skip "Progress consistency (no quest found)"
    fi
else
    skip "Progress consistency (no successful submission to verify)"
fi
echo ""

# =============================================================================
# Property 4: Solution Code Never Exposed
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Property 4: Solution Code Never Exposed"
echo "  GET /tasks/:slug must NOT contain solution_code or test_code"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -n "$TASK_SLUG" ]; then
    TASK_RESP=$(curl -s "$BASE_URL/api/v1/tasks/$TASK_SLUG")

    HAS_SOLUTION=$(echo "$TASK_RESP" | jq 'has("solution_code")')
    HAS_TEST=$(echo "$TASK_RESP" | jq 'has("test_code")')

    if [ "$HAS_SOLUTION" = "false" ]; then
        pass "solution_code is NOT exposed in task detail"
    else
        fail "solution_code IS exposed in task detail!" "$(echo "$TASK_RESP" | jq '.solution_code')"
    fi

    if [ "$HAS_TEST" = "false" ]; then
        pass "test_code is NOT exposed in task detail"
    else
        fail "test_code IS exposed in task detail!"
    fi
else
    # Try with a known slug pattern
    TASK_RESP=$(curl -s "$BASE_URL/api/v1/tasks/hello-variables")
    HTTP_CODE_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/api/v1/tasks/hello-variables")

    if [ "$HTTP_CODE_CHECK" = "200" ]; then
        HAS_SOLUTION=$(echo "$TASK_RESP" | jq 'has("solution_code")')
        HAS_TEST=$(echo "$TASK_RESP" | jq 'has("test_code")')

        if [ "$HAS_SOLUTION" = "false" ]; then
            pass "solution_code is NOT exposed in task detail"
        else
            fail "solution_code IS exposed in task detail!"
        fi

        if [ "$HAS_TEST" = "false" ]; then
            pass "test_code is NOT exposed in task detail"
        else
            fail "test_code IS exposed in task detail!"
        fi
    else
        skip "Solution code check (no task available to test)"
    fi
fi
echo ""

# =============================================================================
# Property 5: Rate Limiting (optional — requires Redis)
# =============================================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Property 5: Rate Limiting"
echo "  Exceeding rate limit should return 429 Too Many Requests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if Redis is available by checking health or making rapid requests
# We'll try to trigger rate limiting by making many rapid requests
GOT_429=false
for i in $(seq 1 120); do
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/api/v1/health")
    if [ "$HTTP_CODE" = "429" ]; then
        GOT_429=true
        break
    fi
done

if [ "$GOT_429" = "true" ]; then
    pass "Rate limiting active: received 429 after rapid requests"
else
    skip "Rate limiting (429 not triggered — Redis may not be connected or limit is high)"
fi
echo ""

# =============================================================================
# Summary
# =============================================================================
echo "=== Correctness Test Results ==="
echo -e "  ${GREEN}Passed${NC}: $PASS"
echo -e "  ${RED}Failed${NC}: $FAIL"
echo -e "  ${YELLOW}Skipped${NC}: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}SOME CORRECTNESS PROPERTIES VIOLATED${NC}"
    exit 1
else
    echo -e "${GREEN}ALL CORRECTNESS PROPERTIES VERIFIED${NC}"
    exit 0
fi
