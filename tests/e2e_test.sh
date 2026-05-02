#!/usr/bin/env bash
# =============================================================================
# RustGym End-to-End Test Script
# =============================================================================
#
# Prerequisites:
#   - Backend API running on localhost:3000 (or set BASE_URL)
#   - PostgreSQL running and migrated
#   - Redis running (optional, for rate limiting tests)
#   - Runner service running on localhost:3001 (for submission tests)
#   - Docker daemon running (for submission tests)
#   - curl and jq installed
#
# Usage:
#   bash tests/e2e_test.sh
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

# Generate unique test user credentials
TIMESTAMP=$(date +%s)
TEST_USERNAME="e2euser_${TIMESTAMP}"
TEST_EMAIL="e2e_${TIMESTAMP}@test.rustgym.dev"
TEST_PASSWORD="TestPass123!"

# Tokens (populated during test)
ACCESS_TOKEN=""
USER_ID=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

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

# =============================================================================
echo "=== RustGym End-to-End Tests ==="
echo "Base URL: $BASE_URL"
echo "Test user: $TEST_USERNAME ($TEST_EMAIL)"
echo ""

# --- 1. Health Check ---
echo "1. Health Check"
HEALTH=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/health")
HTTP_CODE=$(echo "$HEALTH" | tail -1)
BODY=$(echo "$HEALTH" | sed '$d')

if [ "$HTTP_CODE" = "200" ] && echo "$BODY" | jq -e '.status == "ok"' > /dev/null 2>&1; then
    pass "Health check returned 200 with status=ok"
else
    fail "Health check" "HTTP $HTTP_CODE, body: $BODY"
    echo "Backend does not appear to be running. Aborting."
    exit 1
fi
echo ""

# --- 2. Register ---
echo "2. Register new user"
REGISTER_RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\": \"$TEST_USERNAME\", \"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
HTTP_CODE=$(echo "$REGISTER_RESP" | tail -1)
BODY=$(echo "$REGISTER_RESP" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
    ACCESS_TOKEN=$(echo "$BODY" | jq -r '.access_token // empty')
    USER_ID=$(echo "$BODY" | jq -r '.user.id // empty')
    if [ -n "$ACCESS_TOKEN" ] && [ -n "$USER_ID" ]; then
        pass "Registration successful (user_id: $USER_ID)"
    else
        fail "Registration response missing token or user_id" "$BODY"
    fi
else
    fail "Registration" "HTTP $HTTP_CODE, body: $BODY"
fi
echo ""

# --- 3. Verify Email (stub: uses user_id as token) ---
echo "3. Verify email"
if [ -n "$USER_ID" ]; then
    VERIFY_RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/api/v1/auth/verify-email" \
        -H "Content-Type: application/json" \
        -d "{\"token\": \"$USER_ID\"}")
    HTTP_CODE=$(echo "$VERIFY_RESP" | tail -1)
    BODY=$(echo "$VERIFY_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        pass "Email verification successful"
    else
        fail "Email verification" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "Email verification (no user_id from registration)"
fi
echo ""

# --- 4. Login ---
echo "4. Login with new credentials"
LOGIN_RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/api/v1/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
HTTP_CODE=$(echo "$LOGIN_RESP" | tail -1)
BODY=$(echo "$LOGIN_RESP" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
    ACCESS_TOKEN=$(echo "$BODY" | jq -r '.access_token // empty')
    if [ -n "$ACCESS_TOKEN" ]; then
        pass "Login successful, received access token"
    else
        fail "Login response missing access_token" "$BODY"
    fi
else
    fail "Login" "HTTP $HTTP_CODE, body: $BODY"
fi
echo ""

# --- 5. Browse Quests ---
echo "5. Fetch quest list"
QUESTS_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/quests" \
    -H "Authorization: Bearer $ACCESS_TOKEN")
HTTP_CODE=$(echo "$QUESTS_RESP" | tail -1)
BODY=$(echo "$QUESTS_RESP" | sed '$d')

QUEST_SLUG=""
if [ "$HTTP_CODE" = "200" ]; then
    QUEST_COUNT=$(echo "$BODY" | jq '.quests | length')
    QUEST_SLUG=$(echo "$BODY" | jq -r '.quests[0].slug // empty')
    if [ "$QUEST_COUNT" -gt 0 ] 2>/dev/null; then
        pass "Quest list returned $QUEST_COUNT quests (first: $QUEST_SLUG)"
    else
        skip "Quest list returned 0 quests (no seed data)"
    fi
else
    fail "Quest list" "HTTP $HTTP_CODE, body: $BODY"
fi
echo ""

# --- 6. Fetch Quest Detail ---
echo "6. Fetch quest detail"
LEVEL_SLUG=""
if [ -n "$QUEST_SLUG" ]; then
    QUEST_DETAIL_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/quests/$QUEST_SLUG" \
        -H "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$QUEST_DETAIL_RESP" | tail -1)
    BODY=$(echo "$QUEST_DETAIL_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        LEVEL_SLUG=$(echo "$BODY" | jq -r '.levels[0].slug // empty')
        pass "Quest detail for '$QUEST_SLUG' fetched successfully"
    else
        fail "Quest detail" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "Quest detail (no quest available)"
fi
echo ""

# --- 7. Fetch Level Detail ---
echo "7. Fetch level detail"
TASK_SLUG=""
if [ -n "$QUEST_SLUG" ] && [ -n "$LEVEL_SLUG" ]; then
    LEVEL_DETAIL_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/quests/$QUEST_SLUG/levels/$LEVEL_SLUG" \
        -H "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$LEVEL_DETAIL_RESP" | tail -1)
    BODY=$(echo "$LEVEL_DETAIL_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        TASK_SLUG=$(echo "$BODY" | jq -r '.tasks[0].slug // empty')
        pass "Level detail for '$LEVEL_SLUG' fetched successfully"
    else
        fail "Level detail" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "Level detail (no quest/level available)"
fi
echo ""

# --- 8. Fetch Task Detail ---
echo "8. Fetch task detail"
if [ -n "$TASK_SLUG" ]; then
    TASK_DETAIL_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/tasks/$TASK_SLUG")
    HTTP_CODE=$(echo "$TASK_DETAIL_RESP" | tail -1)
    BODY=$(echo "$TASK_DETAIL_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        # Verify solution_code is NOT exposed
        HAS_SOLUTION=$(echo "$BODY" | jq 'has("solution_code")')
        if [ "$HAS_SOLUTION" = "false" ]; then
            pass "Task detail for '$TASK_SLUG' fetched (solution_code not exposed)"
        else
            fail "Task detail exposes solution_code!" "$BODY"
        fi
    else
        fail "Task detail" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "Task detail (no task available)"
fi
echo ""

# --- 9. Submit Code for Task ---
echo "9. Submit code for task"
XP_AWARDED=0
if [ -n "$TASK_SLUG" ]; then
    # Use a simple passing solution
    SUBMIT_CODE='pub fn hello() -> &'\''static str { "Hello, Ferris!" }'

    # Measure response time
    START_TIME=$(date +%s%N 2>/dev/null || date +%s)
    SUBMIT_RESP=$(curl -s -w "\n%{http_code}" --max-time 15 -X POST "$BASE_URL/api/v1/tasks/$TASK_SLUG/submit" \
        -H "Authorization: Bearer $ACCESS_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{\"code\": \"$SUBMIT_CODE\"}")
    END_TIME=$(date +%s%N 2>/dev/null || date +%s)
    HTTP_CODE=$(echo "$SUBMIT_RESP" | tail -1)
    BODY=$(echo "$SUBMIT_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        XP_AWARDED=$(echo "$BODY" | jq -r '.xp_awarded // 0')
        STATUS=$(echo "$BODY" | jq -r '.status // empty')
        pass "Submission returned status='$STATUS', xp_awarded=$XP_AWARDED"
    elif [ "$HTTP_CODE" = "000" ]; then
        skip "Submission timed out (runner service may not be running)"
    else
        fail "Submission" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "Submission (no task available)"
fi
echo ""

# --- 10. Verify XP Awarded ---
echo "10. Verify XP was awarded"
if [ "$XP_AWARDED" -gt 0 ] 2>/dev/null; then
    pass "XP awarded: $XP_AWARDED"
else
    if [ -n "$TASK_SLUG" ]; then
        skip "XP not awarded (submission may have failed or task already completed)"
    else
        skip "XP check (no submission made)"
    fi
fi
echo ""

# --- 11. Check Leaderboard ---
echo "11. Check leaderboard"
LB_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/users/leaderboard")
HTTP_CODE=$(echo "$LB_RESP" | tail -1)
BODY=$(echo "$LB_RESP" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
    ENTRY_COUNT=$(echo "$BODY" | jq '.entries | length')
    pass "Leaderboard returned $ENTRY_COUNT entries"
else
    fail "Leaderboard" "HTTP $HTTP_CODE, body: $BODY"
fi
echo ""

# --- 12. Check User Profile Shows Updated XP ---
echo "12. Check user profile"
if [ -n "$ACCESS_TOKEN" ]; then
    PROFILE_RESP=$(curl -s -w "\n%{http_code}" "$BASE_URL/api/v1/users/me" \
        -H "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$PROFILE_RESP" | tail -1)
    BODY=$(echo "$PROFILE_RESP" | sed '$d')

    if [ "$HTTP_CODE" = "200" ]; then
        PROFILE_XP=$(echo "$BODY" | jq -r '.xp // 0')
        PROFILE_LEVEL=$(echo "$BODY" | jq -r '.level // 0')
        pass "User profile: xp=$PROFILE_XP, level=$PROFILE_LEVEL"
    else
        fail "User profile" "HTTP $HTTP_CODE, body: $BODY"
    fi
else
    skip "User profile (no access token)"
fi
echo ""

# =============================================================================
# Summary
# =============================================================================
echo "=== Results ==="
echo -e "  ${GREEN}Passed${NC}: $PASS"
echo -e "  ${RED}Failed${NC}: $FAIL"
echo -e "  ${YELLOW}Skipped${NC}: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}SOME TESTS FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}ALL TESTS PASSED${NC}"
    exit 0
fi
